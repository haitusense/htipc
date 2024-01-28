using System;
using System.Collections.Generic;
using System.IO.MemoryMappedFiles;
using System.Runtime.InteropServices;
using System.IO;
using System.IO.Pipes;
using System.Windows.Controls;
using System.Numerics;
using System.CodeDom;
using System.Windows;
using System.Windows.Documents;

namespace SimpleGUI;

// https://ufcpp.net/study/csharp/oop/generic-math-operators/

public class NamedPipeSingleton {
  private NamedPipeSingleton() { }
  private static NamedPipeSingleton _instance;
  public static NamedPipeSingleton GetInstance() {
    if (_instance == null) { _instance = new NamedPipeSingleton(); }
    return _instance;
  }

  private CancellationTokenSource _cancelServer = new CancellationTokenSource();

  public void Cancel() => _cancelServer.Cancel();

  public void Run<T>(string path, T window, Action<string, bool> callback) where T : System.Windows.Window {
    void Dispatcher(string src, bool err){
      window.Dispatcher.Invoke((Action)(() => { callback(src, err); }));
    }
    var task = Task.Run(() => {
      while (true) {
        try{
          using(var stream = new NamedPipeServerStream(path, PipeDirection.InOut))
            using(var sr = new StreamReader(stream))
            using(var sw = new StreamWriter(stream)){
              stream.WaitForConnection();
              var src = sr.ReadLine();
              Dispatcher(src, true);
              sw.WriteLine($"OK");
              stream.WaitForPipeDrain();
              sw.Flush();
          }
        } catch (Exception e) {
          Dispatcher(e.ToString(), false);
        }
      }
    });
  }
}


[ClassInterface(ClassInterfaceType.AutoDual)]
[ComVisible(true)]
public class MemoryMapSingleton {

  [StructLayout(LayoutKind.Sequential)]
  public struct Header {
    public int length;
    private int _typecode;
    public int width;
    public int height;
    public int depth;
    
    int a;
    int b;
    int c;

    public TypeCode typecode {
		  set{ this._typecode = (int)value; }
		  get{ return (TypeCode) Enum.ToObject(typeof(TypeCode), this._typecode); }
	  }
    public int HeaderByteSize => Marshal.SizeOf(typeof(Header));

    public int FullByteSize => HeaderByteSize + length * depth;

  }

  private MemoryMapSingleton() { }
  private static MemoryMapSingleton _instance;
  public static MemoryMapSingleton GetInstance() {
    if (_instance == null) { _instance = new MemoryMapSingleton(); }
    return _instance;
  }

  protected MemoryMappedFile mmf;

  public void Create<T> (string key, int w, int h) where T : struct, INumber<T> {
    if(mmf != null) this.Close();
    var header = new Header(){
      length = w * h,
      typecode = Type.GetTypeCode(typeof(T)),
      width = w,
      height = h,
      depth = Marshal.SizeOf(typeof(T)),
    };
    mmf = MemoryMappedFile.CreateNew(key, header.FullByteSize);
    using var accessor = mmf.CreateViewAccessor();
    accessor.Write(0, ref header);
  }

  public int Length() {
    using var accessor = mmf.CreateViewAccessor();
    accessor.Read(0, out Header header);
    return header.length;
  }
  public int Width() {
    using var accessor = mmf.CreateViewAccessor();
    accessor.Read(0, out Header header);
    return header.width;
  }
  public int Height() {
    using var accessor = mmf.CreateViewAccessor();
    accessor.Read(0, out Header header);
    return header.height;
  }
  // public Type GetMmfType() {
  //   using var accessor = mmf.CreateViewAccessor();
  //   accessor.Read(0, out Header header);
  //   return header.typecode switch {
  //     3 => typeof(Boolean),
  //     6 => typeof(Byte),
  //     4 => typeof(Char),
  //     16 => typeof(DateTime),
  //     2 => typeof(DBNull),
  //     15 => typeof(Decimal),
  //     14 => typeof(Double),
  //     7 => typeof(Int16),
  //     9 => typeof(Int32),
  //     11 => typeof(Int64),
  //     1 => typeof(Object),
  //     5 => typeof(SByte),
  //     13 => typeof(Single),
  //     18 => typeof(String),
  //     8 => typeof(UInt16),
  //     10 => typeof(UInt32),
  //     12 => typeof(UInt64),
  //     _ => throw(new Exception())
  //   };
  // }

  public T ReadPixel<T>(int x, int y) where T : struct, INumber<T> {
    using var accessor = mmf.CreateViewAccessor();
    accessor.Read(0, out Header header);
    int index = x + y * header.width;

    V A<U, V>(MemoryMappedViewAccessor acc, Header header, int index) 
    where U : struct, INumber<U>
    where V : struct, INumber<V> {
      acc.Read(header.HeaderByteSize + index * header.depth, out U dst);
      return header.length > index ? V.CreateSaturating(dst) : V.Zero;
    }

    return header.typecode switch {
      TypeCode.Int32 => A<Int32, T>(accessor, header, index),
      _ => throw(new Exception())
    };
  }


  // public T ReadPixel<T>(int index) where T : struct, INumber<T> {
  //   using var accessor = mmf.CreateViewAccessor();
  //   accessor.Read(0, out Header header);

  //   V A<U, V>(MemoryMappedViewAccessor acc, Header header, int index) 
  //   where U : struct, INumber<U>
  //   where V : struct, INumber<V> {
  //     acc.Read(header.HeaderByteSize + index, out U dst);
  //     return header.length > index ? V.CreateSaturating(dst) : V.Zero;
  //   }

  //   return header.typecode switch {
  //     6 => A<Byte, T>(accessor, header, index),
  //     15 => A<Decimal, T>(accessor, header, index),
  //     14 => A<Double, T>(accessor, header, index),
  //     7 => A<Int16, T>(accessor, header, index),
  //     9 => A<Int32, T>(accessor, header, index),
  //     11 => A<Int64, T>(accessor, header, index),
  //     5 => A<SByte, T>(accessor, header, index),
  //     13 => A<Single, T>(accessor, header, index),
  //     8 => A<UInt16, T>(accessor, header, index),
  //     10 => A<UInt32, T>(accessor, header, index),
  //     12 => A<UInt64, T>(accessor, header, index),
  //     _ => throw(new Exception())
  //   };
  // }

  // public T[] ReadPixels<T>() where T : struct, INumber<T> {
  //   using var accessor = mmf.CreateViewAccessor();
  //   accessor.Read(0, out Header header);
  //   var dst = new T[header.size];
  //   accessor.ReadArray(header.HeaderSize, dst, 0, dst.Length);
  //   return dst;
  // }

  // public void ReadPixels<T>(T[] src) where T : struct, INumber<T> {
  //   using var accessor = mmf.CreateViewAccessor();
  //   accessor.Read(0, out Header header);
  //   accessor.ReadArray(header.HeaderSize, src, 0, src.Length);
  // }

  public void WritePixel<T>(int index, T val) where T : struct, INumber<T> {
    using var accessor = mmf.CreateViewAccessor();
    accessor.Read(0, out Header header);
    int dst = Int32.CreateSaturating(val);
    if(header.length > index){ accessor.Write(header.HeaderByteSize + index * header.depth, dst); }
  }


  public void WritePixels(byte[] src) {
    using var accessor = mmf.CreateViewAccessor();
    accessor.Read(0, out Header header);
    accessor.WriteArray(header.HeaderByteSize, src, 0, src.Length);
  }

  public byte[] ReadPixelsForJS(int shift = 0) {
    using var accessor = mmf.CreateViewAccessor();
    accessor.Read(0, out Header header);


    byte[] bitshift_pos<U, V>(MemoryMappedViewAccessor acc, Header header, int shift) 
    where U : struct, INumber<U>, IShiftOperators<U, int, U> {
      var src = new U[header.length];
      var dst = new byte[header.length*4];
      acc.ReadArray(header.HeaderByteSize, src, 0, src.Length);
      return src.SelectMany(n => {
        var buf = byte.CreateSaturating(n << shift);
        return new byte[]{buf,buf,buf,255};
      }).ToArray();
    };
    byte[] bitshift_neg<U, V>(MemoryMappedViewAccessor acc, Header header, int shift) 
    where U : struct, INumber<U>, IShiftOperators<U, int, U> {
      var src = new U[header.length];
      acc.ReadArray(header.HeaderByteSize, src, 0, src.Length);
      return src.SelectMany(n => {
        var buf = byte.CreateSaturating(n >> shift);
        return new byte[]{buf,buf,buf,255};
      }).ToArray();
    };
    byte[] normal<U, V>(MemoryMappedViewAccessor acc, Header header, int shift) 
    where U : struct, INumber<U> {
      var src = new U[header.length];
      acc.ReadArray(header.HeaderByteSize, src, 0, src.Length);
      return src.SelectMany(n => {
        var buf = byte.CreateSaturating(double.CreateSaturating(n) * Math.Pow(2, shift));
        return new byte[]{buf,buf,buf,255};
      }).ToArray();
    };

    return (header.typecode, shift >= 0) switch {
      (TypeCode.Byte, true) => bitshift_pos<Byte, Byte>(accessor, header, shift),
      (TypeCode.Byte, false) => bitshift_neg<Byte, Byte>(accessor, header, -1*shift),
      (TypeCode.Int32, true) => bitshift_pos<Int32, Byte>(accessor, header, shift),
      (TypeCode.Int32, false) => bitshift_neg<Int32, Byte>(accessor, header, -1*shift),
      // 15 => A<Decimal, T>(accessor, header, index),
      (_, _) => normal<Double, Byte>(accessor, header, shift),
      // 7 => A<Int16, T>(accessor, header, index),
      // 9 => A<Int32, T>(accessor, header, index),
      // 11 => A<Int64, T>(accessor, header, index),
      // 5 => A<SByte, T>(accessor, header, index),
      // 13 => A<Single, T>(accessor, header, index),
      // 8 => A<UInt16, T>(accessor, header, index),
      // 10 => A<UInt32, T>(accessor, header, index),
      // 12 => A<UInt64, T>(accessor, header, index),
    };
  }

  public void Close() {
    mmf.Dispose();
    mmf = null;
  }

}
