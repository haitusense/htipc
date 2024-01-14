using System;
using System.Diagnostics;
using System.IO;
using System.IO.Pipes;
using System.Security.Principal;
using System.Text;
using System.Threading;

using System.IO.MemoryMappedFiles;

PipeServer.Run(Args[0], (src) => {
  Console.WriteLine($"{src}");
  for (int i = 0; i < 3; i++) {
    Console.Write(".");
    Thread.Sleep(1000);
  }
  Console.WriteLine("\r\nprocess finished.");
  return "OK.";
});

class PipeServer {
  public static void Run(string pipename, Func<string,string> fn) {
    while(true) {
      try {
        using(var pipeServer = new NamedPipeServerStream(pipename, PipeDirection.InOut)) {
          Console.WriteLine($"namedPipeServerStream object '{pipename}' created.");
          Console.WriteLine("waiting for client connection...");

          pipeServer.WaitForConnection();
          Console.WriteLine("client connected.");

          using (var sr = new StreamReader(pipeServer))
          using (var sw = new StreamWriter(pipeServer)) {
            var dst = fn(sr.ReadLine() ?? "");
            sw.AutoFlush = true;
            sw.WriteLine(dst);
            Console.WriteLine($"sent : {dst}");
          }

        }
      } catch (IOException ofex) {
        Console.WriteLine(ofex.Message);
      } catch (OperationCanceledException oce) {
        // パイプサーバーのキャンセル要求(OperationCanceledExceptionをthrowしてTaskが終わると、Taskは「Cancel」扱いになる)
        throw;
      } finally {
        Console.WriteLine("client disconnected");
      }
    }
  }
}
