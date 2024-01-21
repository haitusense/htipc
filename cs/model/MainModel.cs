
using System.Runtime.InteropServices;
using System.Windows;
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


[ClassInterface(ClassInterfaceType.AutoDual)]
[ComVisible(true)]
public class A;

// webView2.CoreWebView2.AddHostObjectToScript("jscall", new JsCall());

public class MainModel {
  protected internal MainWindow window;

  private MainModel() { }

  protected internal static async Task<MainModel> Build(MainWindow window) {

    var obj = new MainModel();
    obj.window = window;
    MemoryMapSingleton.GetInstance().Create("SimpleGuiMmf",320, 240);

    window.webView.CoreWebView2.OpenDevToolsWindow();
    "SimpleGUI.resource.app.js".res_to_resContents().register_js(window.webView);
    // "SimpleGUI.resource.index.html".res_to_resContents().ShowMessageBoxLite();
    "SimpleGUI.resource.index.html".res_to_resContents().navigate_form_content(window.webView);
    // @"https://www.microsoft.com".navigate_form_url(webView);

    return obj;
  }

  public void Actions(string src, string sub = ""){
    var com = System.Text.Json.JsonSerializer.Deserialize<string[]>(src);
    window.webView.ExecuteScriptAsync($$""" console.log({{src}}) """);
    Action act = com[0] switch {
      "DOMContentLoaded" =>()=> { window.statusLabel.Text = "DOMContentLoaded"; },
      "load" =>()=> { window.statusLabel.Text = "load"; },
      "fill" =>()=> {
        window.webView.ExecuteScriptAsync($$""" console.log("fill start") """);
        byte n = 0;
        byte.TryParse(com[1], out n);
        var buf = new byte[MemoryMapSingleton.GetInstance().Length()];
        for(int i=0;i < buf.Length; i++){
          buf[i] = n;
        }
        MemoryMapSingleton.GetInstance().WritePixels(buf);
        window.webView.ExecuteScriptAsync($$""" console.log("fill end") """);
        window.webView.ExecuteScriptAsync($$""" drawFromMemoryMap("canvas") """);
      },
      "draw" =>()=> {
        // webView.CoreWebView2.AddScriptToExecuteOnDocumentCreatedAsync($$""" console.log("fill") """);
        window.webView.ExecuteScriptAsync($$""" drawFromMemoryMap("canvas") """);
        // webView.CoreWebView2.PostWebMessageAsJson(json);
      },
      "eval" =>()=> {
        window.webView.ExecuteScriptAsync(com[1]);
      },
      _=>()=> {}
    };
    act();
  }
}

