using System.Text.RegularExpressions;
using System.Diagnostics;
using System.Reflection;

namespace SimpleGUI;

/* Navigate */

public static class NavigateEx {

  public static void navigate_form_content(this string content, Microsoft.Web.WebView2.Wpf.WebView2 webView) {
    if (webView != null && webView.CoreWebView2 != null) {
      webView.CoreWebView2.NavigateToString(content);
    }
  }

  public static void navigate_form_url(this string url, Microsoft.Web.WebView2.Wpf.WebView2 webView) {
    if (webView != null && webView.CoreWebView2 != null) {
      if (Uri.IsWellFormedUriString(url, UriKind.Absolute)){
        webView.CoreWebView2.Navigate(url);
      }else{
        webView.CoreWebView2.Navigate(System.IO.Path.Combine(System.IO.Directory.GetCurrentDirectory(), url));
      }
    }
  }


  public static async void register_js(this string content, Microsoft.Web.WebView2.Wpf.WebView2 webView, bool immediately_invok = false) {
    var _result = await webView.CoreWebView2.AddScriptToExecuteOnDocumentCreatedAsync(content);
    // await webView.CoreWebView2.AddScriptToExecuteOnDocumentCreatedAsync($$""" console.log("invok {{i}}") """);
    // await webView.CoreWebView2.ExecuteScriptAsync($$""" console.log("register {{i}}") """);
    if(immediately_invok) {
      await webView.ExecuteScriptAsync(content);
      // await webView.CoreWebView2.ExecuteScriptAsync($$""" console.log("immediately_invok {{i}}") """);
    }
  }


  // public async Task<string> AddScript(string key, string code, bool immediately_invok = false) {
  //   if(id.ContainsKey(key)) { 
  //     webView.CoreWebView2.RemoveScriptToExecuteOnDocumentCreated(id[key]);
  //     id.Remove(key);
  //   }
  //   var result = await webView.CoreWebView2.AddScriptToExecuteOnDocumentCreatedAsync(code);
  //   if(immediately_invok) {
  //     await webView.ExecuteScriptAsync(code);
  //   }
  //   id.Add(key, result);
  //   return result;
  // }
  // public void RemoveScript(string key) {
  //   if(id.ContainsKey(key)){
  //     webView.CoreWebView2.RemoveScriptToExecuteOnDocumentCreated(id[key]);
  //     id.Remove(key);
  //   }
  // }

  // public async Task<string> AddJS2(string test) {
  //   var result = await webView.CoreWebView2.ExecuteScriptAsync(test);
  //   return result;
  // }
  
  
  public static void ShowMessageBoxLite(this string s) {
    System.Windows.MessageBox.Show(s);
  }

}

public static class ResourceEx {

  public static List<string> reg_to_resPaths(this string reg) {
    var assembly = System.Reflection.Assembly.GetExecutingAssembly();
    var paths = assembly.GetManifestResourceNames().ToList().FindAll(n => Regex.IsMatch(n, reg));
    return paths;
  }

  public static string res_to_resContents(this string path) {
    var assembly = System.Reflection.Assembly.GetExecutingAssembly();
    using(var stream = assembly.GetManifestResourceStream(path))
    using(var streamReader = new System.IO.StreamReader(stream)) 
    {
      var code = streamReader.ReadToEnd();
      return code;
    }
  }

}