using System;
using System.Diagnostics;
using System.Text;
using System.Text.Json.Nodes;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Data;
using System.Windows.Documents;
using System.Windows.Input;
using System.Windows.Media;
using System.Windows.Media.Imaging;
using System.Windows.Navigation;
using System.Windows.Shapes;

using Microsoft.Web.WebView2.Core;
using System.ComponentModel;
using System.Runtime.InteropServices;
using Microsoft.Web.WebView2;
using Microsoft.Web.WebView2.Wpf;

namespace SimpleGUI;

/// <summary>
/// Interaction logic for MainWindow.xaml
/// </summary>
public partial class MainWindow : Window {
    
    [System.Runtime.InteropServices.DllImport("Kernel32.dll")]
    public static extern bool AttachConsole(int processId);
    
    public MainModel model;

    public MainWindow() {
        // dotnet run ではなく vs codeのF5ならdebug consoleにアタッチされる
        AttachConsole(-1);
        // Trace.Listeners.Add(new TextWriterTraceListener(Console.Out));
        InitializeComponent();
        Application.Current.DispatcherUnhandledException += OnDispatcherUnhandledException;
    }

    private void OnDispatcherUnhandledException(object sender, System.Windows.Threading.DispatcherUnhandledExceptionEventArgs e) {
        MessageBox.Show(e.Exception.ToString(), "DispatcherUnhandledException");
        Environment.Exit(1);
    }

    private async void Window_Loaded(object sender, RoutedEventArgs e) {
        Console.WriteLine("Window_Loaded");
        var wv = this.webView;

        wv.CoreWebView2InitializationCompleted += (/*object*/ s, /*CoreWebView2InitializationCompletedEventArgs*/e) => Console.WriteLine("WebView2InitializationCompleted");
        var webview_options = new CoreWebView2EnvironmentOptions("--allow-file-access-from-files");
        var environment = await CoreWebView2Environment.CreateAsync(null, null, webview_options);  
        await wv.EnsureCoreWebView2Async(environment);// WebView2初期化完了確認

        model = await MainModel.Build(wv);
        NamedPipeSingleton.GetInstance().Run("SimpleGui");
        
        /* postMessage({'a': 'b'}), postMessage(1.2) -> ArgumentException, postMessage('example') -> "example" */
        wv.CoreWebView2.WebMessageReceived += (/*object*/ s, /*CoreWebView2WebMessageReceivedEventArgs*/ e) => { model.dispatch( e.TryGetWebMessageAsString() ); };
        wv.NavigationCompleted += (/*object*/ s, /*CoreWebView2NavigationCompletedEventArgs*/ e) => { model.dispatch( e.ToString().to_json() ); };
        NamedPipeSingleton.GetInstance().PipeMessageReceived += (s, e) => { this.Dispatcher.Invoke(() => { model.dispatch(e); }); };

        wv.CoreWebView2.AddHostObjectToScript("SimpleGuiMmf", MemoryMapSingleton.GetInstance());
        wv.CoreWebView2.AddHostObjectToScript("actionCreator", ActionCreator.GetInstance());

        wv.CoreWebView2.AddHostObjectToScript("State", State.GetInstance());
        wv.CoreWebView2.AddHostObjectToScript("statusLabel", this.statusLabel);
        
        // window.webView.ExecuteScriptAsync($$"""  """);
        // window?.webView.CoreWebView2.PostWebMessageAsString(propertyName);
        State.GetInstance().PropertyChanged += (s, e) => wv.CoreWebView2.PostWebMessageAsString((e as PropertyChangedEventArgs).PropertyName);

    }
}

[ClassInterface(ClassInterfaceType.AutoDual)]
[ComVisible(true)]
public class TextBlockJs : TextBlock { }

[ClassInterface(ClassInterfaceType.AutoDual)]
[ComVisible(true)]
public class MainWindowJs {

}

