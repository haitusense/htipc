// # load "namedpipe.csx"
using System;
using System.Diagnostics;
using System.IO;
using System.IO.Pipes;
using System.Security.Principal;
using System.Text;
using System.Threading;
using System.Text.Json;
using System.IO.MemoryMappedFiles;

static CancellationTokenSource _cancelServer = new CancellationTokenSource();

PipeServer.Run(Args[0], (src) => {
  Console.WriteLine($"{"reserved".green()} {src}");
  var json = JsonSerializer.Deserialize<string[]>(src);
  Func<string> func = src switch {
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
}, _cancelServer.Token);

// Console.WriteLine("===");
// Thread.Sleep(3000);
// Console.WriteLine("===");
// _cancelServer.Cancel();
// https://tera1707.com/entry/2023/04/30/004032
while(true){ }

class PipeServer {
  public static void Run(string pipename, Func<string, string> fn, CancellationToken ct) {
    Task.Run(async () => {
      while(true) {
        try {
          using (var pipeServer = new NamedPipeServerStream(pipename, PipeDirection.InOut, 1, PipeTransmissionMode.Byte, PipeOptions.Asynchronous)){
          // using(var pipeServer = new NamedPipeServerStream(pipename, PipeDirection.InOut)){
            Console.WriteLine($"{"connecting".blue()} created namedPipeServerStream object '{pipename}'.");
            Console.WriteLine($"{"connecting".blue()} waiting for client connection....");
            // pipeServer.WaitForConnection();
                    // if (token.IsCancellationRequested) {
          //   // Thread.Sleep(2000);
          //   return;
          // }
            await pipeServer.WaitForConnectionAsync(ct);
            Console.WriteLine($"{"connected".green()} client connected.");
            using(var sr = new StreamReader(pipeServer))
            using(var sw = new StreamWriter(pipeServer)){
              // var recv = sr.ReadLine();
              var recv = await sr.ReadLineAsync();
              // var dst = await fn.Invoke(recv ?? "");
              var dst = fn(recv ?? ""); // 戻り値受ける前に抜けるので動かん
              sw.AutoFlush = true;
              await sw.WriteLineAsync(dst);
              Console.WriteLine($"{"sent".green(),-12} {dst}");
            }
          }
        } catch (IOException ofex) { 
          Console.WriteLine($"{"disconnected".blue()} client disconnected.");
          Console.WriteLine($"ofex : {ofex.Message}");
        } catch (OperationCanceledException oce) { // パイプサーバーのキャンセル要求(OperationCanceledExceptionをthrowしてTaskが終わると、Taskは「Cancel」扱いになる)
          Console.WriteLine($"oce : {oce.Message}");
          throw;
        } finally {
          Console.WriteLine($"{"disconnected".blue()} client disconnected.");
        }
      }
    });
  }
}


#region ExtensionMethods
public static string red(this string src){ return $"\x1b[31m{src}\x1b[39m"; }
public static string green(this string src){ return $"\x1b[32m{src}\x1b[39m"; }
public static string blue(this string src){ return $"\x1b[34m{src}\x1b[39m"; }
public static string yellow(this string src){ return $"\x1b[33m{src}\x1b[39m"; }

#endregion
