
using System.Runtime.InteropServices;
using System.Windows;
using System.Windows.Media.TextFormatting;
using Accessibility;

namespace SimpleGUI;

[ClassInterface(ClassInterfaceType.AutoDual)]
[ComVisible(true)]
public class MainModel {
  protected internal Microsoft.Web.WebView2.Wpf.WebView2 webView;
  protected internal MainWindow window;

  private MemoryMap<int> memoryMap = new MemoryMap<int>("MyMemoyMap",320,240);

  private MainModel() { }

  protected internal static async Task<MainModel> Build(MainWindow window, Microsoft.Web.WebView2.Wpf.WebView2 webView) {

    var obj = new MainModel();
    obj.window = window;
    obj.webView = webView;
    webView.CoreWebView2.AddHostObjectToScript("SimpleGUI", obj);
    
    webView.CoreWebView2.OpenDevToolsWindow();
    
    "SimpleGUI.resource.app.js".res_to_resContents().register_js(webView);
    // "SimpleGUI.resource.index.html".res_to_resContents().ShowMessageBoxLite();
    "SimpleGUI.resource.index.html".res_to_resContents().navigate_form_content(webView);
    // @"https://www.microsoft.com".navigate_form_url(webView);

    NamedPipe.Run("SimpleGUI", window, (n, m) =>{
      var dst = System.Text.Json.JsonSerializer.Deserialize<string[]>(n);
      obj.Actions(dst)
    );

    return obj;
  }

  public string Actions(string[] src){
    Action act = src[0] switch {
      "DOMContentLoaded" =>()=> { window.statusLabel.Text = "DOMContentLoaded"; },
      "load" =>()=> { window.statusLabel.Text = "load"; },
      "draw" =>()=> { window.statusLabel.Text = "draw"; },
      "fill" =>()=> {
        // webView.CoreWebView2.AddScriptToExecuteOnDocumentCreatedAsync($$""" console.log("fill") """);
        webView.ExecuteScriptAsync($$""" drawFromMemoryMap("canvas", {{src[1]}}) """);
        // webView.CoreWebView2.PostWebMessageAsJson(json);
      },
      "eval" =>()=> {
        webView.ExecuteScriptAsync($$""" {{src[1]}}) """);
      },
      _=>()=> {}
    };
    act();
    return "OK";
  }
}
// await 
