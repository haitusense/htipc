
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
  
  Dictionary<string, Action<string[]>> actions = new Dictionary<string, Action<string[]>>();

  public int shift = 0;

  private MainModel() { }

  protected internal static async Task<MainModel> Build(MainWindow window) {

    var obj = new MainModel();
  
    MemoryMapSingleton.GetInstance().Create<int>("SimpleGuiMmf",320, 240);

    window.webView.CoreWebView2.OpenDevToolsWindow();
    "SimpleGUI.resource.app.js".res_to_contents().register_js(window.webView);
    // "SimpleGUI.resource.index.html".res_to_resContents().ShowMessageBoxLite();
    "SimpleGUI.resource.index.html".res_to_contents().navigate_form_content(window.webView);
    // @"https://www.microsoft.com".navigate_form_url(webView);
    // webView2.CoreWebView2.AddHostObjectToScript("jscall", new JsCall());

    
    obj.actions["message"] =(n)=>{     
      window.statusLabel.Text = n[0];
      // webView.CoreWebView2.PostWebMessageAsJson(json);
    };
    obj.actions["eval"] =(n)=>{ window.webView.ExecuteScriptAsync(n[1]); };
    obj.actions["mousemove"] =(n)=>{ 
      var k = MemoryMapSingleton.GetInstance().ReadPixel<int>(int.Parse(n[0]), int.Parse(n[1]));
      window.statusLabel.Text = $"{n[0]}-{n[1]} : {k}";
    };
    obj.actions["draw"] =(_)=>{
      var shift = window.model.shift;
      window.webView.ExecuteScriptAsync($$""" drawFromMemoryMap("canvas", {{shift}}) """);
    };
    obj.actions["fill"] =(_)=>{
      var mmf = MemoryMapSingleton.GetInstance();
      var w = mmf.Width();
      var h = mmf.Height();
      for(int y=0; y < h; y++){ 
        for(int x=0; x < w; x++){
          mmf.WritePixel<int>(x + y * w, x % 2); 
        } 
      }
      // MemoryMapSingleton.GetInstance().WritePixels(buf);
      window.webView.ExecuteScriptAsync($$""" drawFromMemoryMap("canvas" {{window.model.shift}}) """);
    };
    obj.actions["shiftup"] =(_)=>{
      window.model.shift++;
      window.webView.ExecuteScriptAsync($$""" drawFromMemoryMap("canvas", {{window.model.shift}}) """);
    };
    obj.actions["shiftdown"] =(_)=>{ 
      window.model.shift--;
      window.webView.ExecuteScriptAsync($$""" drawFromMemoryMap("canvas", {{window.model.shift}}) """);
    };
    obj.actions["shift"] =(n)=>{ 
      window.model.shift = int.Parse(n[0]);
      window.webView.ExecuteScriptAsync($$""" drawFromMemoryMap("canvas", {{window.model.shift}}) """);
    };
    obj.actions["rendering"] =(n)=>{
      window.webView.ExecuteScriptAsync($$""" document.getElementById("canvas").style.imageRendering = "{{n[0]}}"; """);
    };
    
    return obj;
  }

 
  public struct Act {
      public string type {get; set; }
      public string[] payload {get; set; }
  }
  
  public void Actions(string json){
    try{
      Console.WriteLine(json);
      var obj = System.Text.Json.JsonSerializer.Deserialize<Act>(json);
      Action<string[]> act;
      if (actions.TryGetValue(obj.type, out act)) { act(obj.payload); }

    }catch(Exception e){
      Console.WriteLine(e.ToString());
    }
  }

}

