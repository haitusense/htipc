using System;
using System.Diagnostics;
using System.IO;
using System.IO.Pipes;
using System.Security.Principal;
using System.Text;
using System.Threading;

using System.IO.MemoryMappedFiles;

PipeServer.Run(Args[0], (src) => {
  Console.WriteLine($"{"reserved".green()} {src}");
  Func<string> func = src switch {
    "test" =>()=> { return "ERR"; },
    "test" =>()=> { return "ERR"; },
    _=>()=> {
      Console.Write($"heavy process");
      for (int i = 0; i < 8; i++) {
        Console.Write(".");
        Thread.Sleep(500);
      }
      Console.WriteLine("finished.");
      return "OK";
    }
  };
  return func();
});

class PipeServer {
  MemoryMappedServer mmf = new MemoryMappedServer();

  public static void Run(string pipename, Func<MemoryMappedServer, string,string> fn) {
    while(true) {
      try {
        using(var pipeServer = new NamedPipeServerStream(pipename, PipeDirection.InOut)) {
          Console.WriteLine($"{"connecting".blue()} created namedPipeServerStream object '{pipename}'.");
          Console.WriteLine($"{"connecting".blue()} waiting for client connection....");

          pipeServer.WaitForConnection();
          Console.WriteLine($"{"connected".green()} client connected.");

          using (var sr = new StreamReader(pipeServer))
          using (var sw = new StreamWriter(pipeServer)) {
            var dst = fn(sr.ReadLine() ?? "");
            sw.AutoFlush = true;
            sw.WriteLine(dst);
            Console.WriteLine($"{"sent".green(),-12} {dst}");
          }

        }
      } catch (IOException ofex) { 
        Console.WriteLine(ofex.Message);
      } catch (OperationCanceledException oce) { 
        Console.WriteLine(oce.Message);
        // パイプサーバーのキャンセル要求(OperationCanceledExceptionをthrowしてTaskが終わると、Taskは「Cancel」扱いになる)
        throw;
      } finally {
        Console.WriteLine($"{"disconnected".blue()} client disconnected.");
      }
    }
  }
}

class MemoryMappedServer {
  MemoryMappedFile mmf;
  public void init(int x, int y) {
    mmf = MemoryMappedFile.CreateNew("SharedMemory", x*y*sizeof(int));
    using (var accessor = mmf.CreateViewAccessor()) {
      accessor.Write(0, data.Length);
      accessor.WriteArray(sizeof(int), data, 0, data.Length);
    }
  }
}

#region ExtensionMethods
public static string red(this string src){ return $"\x1b[31m{src}\x1b[39m"; }
public static string green(this string src){ return $"\x1b[32m{src}\x1b[39m"; }
public static string blue(this string src){ return $"\x1b[34m{src}\x1b[39m"; }
public static string yellow(this string src){ return $"\x1b[33m{src}\x1b[39m"; }

#endregion
