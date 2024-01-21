using System;
using System.Collections.Generic;
using System.IO.MemoryMappedFiles;
using System.Runtime.InteropServices;
using System.IO;
using System.IO.Pipes;
using System.Windows.Controls;

namespace SimpleGUI;

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
  private MemoryMapSingleton() { }
  private static MemoryMapSingleton _instance;
  public static MemoryMapSingleton GetInstance() {
    if (_instance == null) { _instance = new MemoryMapSingleton(); }
    return _instance;
  }

  protected MemoryMappedFile mmf;

  private int headersize = 32;

  public void Create(string key, int w, int h) {
    if(mmf != null) this.Close();
    mmf = MemoryMappedFile.CreateNew(key, headersize +  w * h * Marshal.SizeOf(typeof(byte)));
    using var accessor = mmf.CreateViewAccessor();
    accessor.Write(Marshal.SizeOf(typeof(int)) * 0, w * h);
    accessor.Write(Marshal.SizeOf(typeof(int)) * 1, w);
    accessor.Write(Marshal.SizeOf(typeof(int)) * 2, h);
  }

  public int Length() {
    using var accessor = mmf.CreateViewAccessor();
    return accessor.ReadInt32(0);
  }

  public int Width() {
    using var accessor = mmf.CreateViewAccessor();
    return accessor.ReadInt32(Marshal.SizeOf(typeof(int)) * 1);
  }

  public int Height() {
    using var accessor = mmf.CreateViewAccessor();
    return accessor.ReadInt32(Marshal.SizeOf(typeof(int)) * 2);
  }

  public byte ReadPixel(int index) {
    using var accessor = mmf.CreateViewAccessor();
    return accessor.ReadInt32(0) > index ? accessor.ReadByte(headersize + index) : (byte)0;
  }

  public void WritePixel(int index, byte val) {
    using var accessor = mmf.CreateViewAccessor();
    if(accessor.ReadInt32(0) > index){ accessor.Write(headersize + index, val); }
  }

  public byte[] ReadPixels() {
    using var accessor = mmf.CreateViewAccessor();
    var dst = new byte[accessor.ReadInt32(0)];
    accessor.ReadArray(headersize, dst, 0, dst.Length);
    return dst;
  }

  public void WritePixels(byte[] src) {
    using var accessor = mmf.CreateViewAccessor();
    accessor.WriteArray(headersize, src, 0, src.Length);
  }

  public byte[] ReadPixelsForJS() {
    using var accessor = mmf.CreateViewAccessor();
    var src = new byte[accessor.ReadInt32(0)];
    var dst = new byte[accessor.ReadInt32(0) * 4];
    accessor.ReadArray(headersize, src, 0, src.Length);
    for(var i=0; i< src.Length; i++){
      dst[i*4] = src[i];
      dst[i*4+1] = src[i];
      dst[i*4+2] = src[i];
      dst[i*4+3] = 255;
    }
    return dst;
  }

  public void Close() {
    mmf.Dispose();
    mmf = null;
  }

}

  // [StructLayout(LayoutKind.Sequential)]
  // public struct Header {
  //   public int Size;
  //   public int Sizeof;
  //   public int Width;
  //   public int Height;
    
  //   public int Length(){ return Size * Sizeof; }
  // }