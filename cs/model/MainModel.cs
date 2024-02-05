
using System.Collections;
using System.Runtime.InteropServices;
using System.Windows;
using System.Windows.Media;
using System.Windows.Media.TextFormatting;
using Accessibility;

namespace SimpleGUI;

/*


   ┌──────────────┐  NamedPipe ┌──Model ─────────┐
   │              ├───────────►│                 │
   │    scripts   │            │                 │
   │              ├─────┐      │                 │
   └──────────────┘     │      │                 │
                        │      │                 │
   ┌──UI──────────┐     │      │                 │
   │              │     ▼      │                 │
   │ ┌──────────┐   ┌───────┐  │                 │
   │ │          ├──►│  mmf  │◄─┤                 │
   │ │javascript│   └───────┘  │                 │
   │ │          │   postMessage│                 │
   │ │          ├─────────────►│                 │
   │ └──────────┘              │                 │
   │       ▲      │            └──────┬──┬───────┘
   └────── │ ─────┘                   │  │
        ▲  │                          │  │
        │  └──────────────────────────┘  │
        │        addEventListener        │
        │       ExecuteScriptAsync       │
        │                                │
        └────────────────────────────────┘
                 System.Windows
*/


public class MainModel {
  
  Dictionary<string, Action<string[]>> dispatcher = new Dictionary<string, Action<string[]>>();

  private MainModel() { }

  protected internal static async Task<MainModel> Build(Microsoft.Web.WebView2.Wpf.WebView2 wv2) {
    var obj = new MainModel();
  
    MemoryMapSingleton.GetInstance().Create<int>("SimpleGuiMmf",320, 240);

    wv2.CoreWebView2.OpenDevToolsWindow();
    "SimpleGUI.resource.app.js".res_to_contents().register_js(wv2);
    "SimpleGUI.resource.rxjs.umd.min.js".res_to_contents().register_js(wv2);
    // "SimpleGUI.resource.index.html".res_to_resContents().ShowMessageBoxLite();
    "SimpleGUI.resource.index.html".res_to_contents().navigate_form_content(wv2);
    // @"https://www.microsoft.com".navigate_form_url(webView);
    // webView2.CoreWebView2.AddHostObjectToScript("jscall", new JsCall());

    
    obj.dispatcher["message"] =(n)=>{ State.GetInstance().Label = n[0]; };
    obj.dispatcher["eval"] =(n)=>{ wv2.ExecuteScriptAsync(n[0]); };
    obj.dispatcher["mousemove"] =(n)=>{ 
      var k = MemoryMapSingleton.GetInstance().ReadPixel<int>(int.Parse(n[0]), int.Parse(n[1]));
      State.GetInstance().Label = $"{n[0]}-{n[1]} : {k}";
    };
    obj.dispatcher["draw"] =(_)=>{ wv2.ExecuteScriptAsync($$""" drawFromMemoryMap("canvas", {{State.GetInstance().Shift}}) """); };
    obj.dispatcher["fill"] =(_)=>{
      var mmf = MemoryMapSingleton.GetInstance();
      var w = mmf.Width();
      var h = mmf.Height();
      for(int y=0; y < h; y++){ 
        for(int x=0; x < w; x++){
          mmf.WritePixel<int>(x + y * w, x % 2); 
        } 
      }
    };
    obj.dispatcher["shiftup"] =(n)=>{ State.GetInstance().Shift = State.GetInstance().Shift + int.Parse(n[0]); };
    obj.dispatcher["shiftdown"] =(n)=>{ State.GetInstance().Shift = State.GetInstance().Shift + int.Parse(n[0]); };
    obj.dispatcher["shift"] =(n)=>{ State.GetInstance().Shift = int.Parse(n[0]); };
    obj.dispatcher["rendering"] =(n)=>{
      wv2.ExecuteScriptAsync($$""" document.getElementById("canvas").style.imageRendering = "{{n[0]}}"; """);
    };
    
    return obj;
  }

 
  public struct Act {
      public string type {get; set; }
      public string[] payload {get; set; }
  }
  
  public void dispatch(string json){
    try{
      Console.WriteLine(json);
      var obj = System.Text.Json.JsonSerializer.Deserialize<Act>(json);
      Action<string[]> act;
      if (dispatcher.TryGetValue(obj.type, out act)) { act(obj.payload); }

    }catch(Exception e){
      Console.WriteLine(e.ToString());
    }
  }

}

