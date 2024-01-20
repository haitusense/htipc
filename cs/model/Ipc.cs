using System;
using System.Collections.Generic;
using System.IO.MemoryMappedFiles;
using System.Runtime.InteropServices;
using System.IO;
using System.IO.Pipes;

namespace SimpleGUI;

public static class NamedPipe {

  public static void Run<T>(string path, T window, Func<string, bool, string> callback) where T : System.Windows.Window {
    string Dispatcher(string src, bool err){
      String dst = null;
      window.Dispatcher.Invoke((Action)(() => { dst = callback(src, err); }));
      return dst;
    }
    var task = Task.Run(() => {
      while (true) {
        try{
          using var stream = new NamedPipeServerStream(path, PipeDirection.InOut);
          stream.WaitForConnection();
          using var sr = new StreamReader(stream);
          using var sw = new StreamWriter(stream);
          var src = sr.ReadLine();
          var dst = Dispatcher(src, true);
          sw.AutoFlush = true;
          sw.WriteLine("OK");
  
        } catch (Exception e) {
          Dispatcher(e.ToString(), false);
        }
      }
    });
  }
}



public class MemoryMap {

  [StructLayout(LayoutKind.Sequential)]
  public struct Header {
    public int Size;
    public int Sizeof;
    public int Width;
    public int Height;
    
    public int Length(){ return Size * Sizeof; }
  }

  protected MemoryMappedFile mmf;

  public Header GetHeader() {
    using var accessor = mmf.CreateViewAccessor();
    Header dst;
    accessor.Read(0, out dst);
    return dst;
  }

  // public int Length() -> this.GetHeader().Length();

  public void Close() {
    mmf.Dispose();
    mmf = null;
  }

  public virtual dynamic Read() => null;

  public virtual dynamic Read(int index) => null;

  public virtual void Write(dynamic data) { }

}


public class MemoryMap<T> : MemoryMap where T : struct {

  public MemoryMap(string key, int width, int height) {
    var dst = new Header() {
      Width = width,
      Height = height,
      Sizeof = Marshal.SizeOf(typeof(T)),
      Size = width * height * Marshal.SizeOf(typeof(T))
    };    
    mmf = MemoryMappedFile.CreateNew(key, Marshal.SizeOf(typeof(Header)) + width * height * Marshal.SizeOf(typeof(T)));
    using var accessor = mmf.CreateViewAccessor();
    accessor.Write(0, ref dst);
  }

  public MemoryMap(string key, T[] src) {
    mmf = MemoryMappedFile.CreateNew(key, Marshal.SizeOf(typeof(Header)) + src.Length * Marshal.SizeOf(typeof(T)));
    using var accessor = mmf.CreateViewAccessor();
    accessor.Write(0, src.Length);
    accessor.WriteArray(Marshal.SizeOf(typeof(Header)), src, 0, src.Length);
  }

  public override dynamic Read() => (dynamic)_Read();

  public override dynamic Read(int index) => (dynamic)_Read(index);

  public T[] _Read() {
    using var accessor = mmf.CreateViewAccessor();
    int size = accessor.ReadInt32(0); // サイズの読み込み
    var dst = new T[size];
    accessor.ReadArray<T>(sizeof(int), dst, 0, dst.Length);
    return dst;
  }

  public T _Read(int index) {
    using var accessor = mmf.CreateViewAccessor();
    int size = accessor.ReadInt32(0);
    T dst = default;
    if(index < size)
      accessor.Read<T>(sizeof(int) + index * Marshal.SizeOf(typeof(T)), out dst);
    return dst;
  }
  
  public override void Write(dynamic data) => _Write(data);

  public void _Write(T[] data) {
    using var accessor = mmf.CreateViewAccessor();
    accessor.Write(0, data.Length);
    accessor.WriteArray<T>(sizeof(int), data, 0, data.Length);
  }

}