
using System.Collections;
using System.Runtime.InteropServices;
using System.Windows;
using System.Windows.Media;
using System.Windows.Media.TextFormatting;
using Accessibility;

namespace SimpleGUI;

/*


   ┌──────────────┐  NamedPipe ┌──Action─────────┐
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
  protected internal MainWindow window;
  Dictionary<String, Action<string[]>> actions;

  private MainModel() { }

  protected internal static async Task<MainModel> Build(MainWindow window) {

    var obj = new MainModel();
    obj.window = window;
    obj.actions = ActionCreater(window);
    MemoryMapSingleton.GetInstance().Create<int>("SimpleGuiMmf",320, 240);

    window.webView.CoreWebView2.OpenDevToolsWindow();
    "SimpleGUI.resource.app.js".res_to_contents().register_js(window.webView);
    // "SimpleGUI.resource.index.html".res_to_resContents().ShowMessageBoxLite();
    "SimpleGUI.resource.index.html".res_to_contents().navigate_form_content(window.webView);
    // @"https://www.microsoft.com".navigate_form_url(webView);
// webView2.CoreWebView2.AddHostObjectToScript("jscall", new JsCall());
    return obj;
  }

  public int shift = 0;

  public static Dictionary<String, Action<string[]>> ActionCreater(MainWindow window) {
    var dst = new Dictionary<String, Action<string[]>>();
    dst["DOMContentLoaded"] =(_)=>{ window.statusLabel.Text = "DOMContentLoaded"; };
    dst["mousemove"] =(n)=>{ 
      var k = MemoryMapSingleton.GetInstance().ReadPixel<int>(int.Parse(n[1]), int.Parse(n[2]));
      window.statusLabel.Text = $"{n[1]}-{n[2]} : {k}";
    };
    dst["load"] =(_)=>{ window.statusLabel.Text = "load"; };
    dst["draw"] =(_)=>{
      var shift = window.model.shift;
      window.webView.ExecuteScriptAsync($$""" drawFromMemoryMap("canvas", {{shift}}) """);
      // webView.CoreWebView2.PostWebMessageAsJson(json);
    };
    dst["fill"] =(_)=>{
      var mmf = MemoryMapSingleton.GetInstance();
      var w = mmf.Width();
      var h = mmf.Height();
      for(int y=0; y < h; y++){ 
        for(int x=0; x < w; x++){
          mmf.WritePixel<int>(x + y * w, (x / 10) % 2); 
        } 
      }
      // MemoryMapSingleton.GetInstance().WritePixels(buf);
      window.webView.ExecuteScriptAsync($$""" drawFromMemoryMap("canvas" {{window.model.shift}}) """);
    };
    dst["eval"] =(n)=>{ window.webView.ExecuteScriptAsync(n[1]); };
    dst["shiftup"] =(_)=>{
      window.model.shift++;
      window.webView.ExecuteScriptAsync($$""" drawFromMemoryMap("canvas", {{window.model.shift}}) """);
    };
    dst["shiftdown"] =(_)=>{ 
      window.model.shift--;
      window.webView.ExecuteScriptAsync($$""" drawFromMemoryMap("canvas", {{window.model.shift}}) """);
    };
    return dst;
  }

  public void Actions(string json, string sub = ""){
    var com = System.Text.Json.JsonSerializer.Deserialize<string[]>(json);
    Action<string[]> act;
    if (actions.TryGetValue(com[0], value: out act)) { act(com); }
  }
}

