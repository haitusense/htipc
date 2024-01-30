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

using System.Text.Json;
using Microsoft.Web.WebView2.Core;

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

        // (object sender, CoreWebView2InitializationCompletedEventArgs e)
        this.webView.CoreWebView2InitializationCompleted += (s, e) => Console.WriteLine("WebView2InitializationCompleted");
    }

    private void OnDispatcherUnhandledException(object sender, System.Windows.Threading.DispatcherUnhandledExceptionEventArgs e) {
        MessageBox.Show(e.Exception.ToString(), "DispatcherUnhandledException");
        Environment.Exit(1);
    }

    private async void Window_Loaded(object sender, RoutedEventArgs e) {
        Console.WriteLine("Window_Loaded");

        var webview_options = new CoreWebView2EnvironmentOptions("--allow-file-access-from-files");
        var environment = await CoreWebView2Environment.CreateAsync(null, null, webview_options);  
        await this.webView.EnsureCoreWebView2Async(environment);// WebView2初期化完了確認

        model = await MainModel.Build(this);
        NamedPipeSingleton.GetInstance().Run("SimpleGui");
        
        /*
            postMessage({'a': 'b'})      ArgumentException
            postMessage(1.2)             ArgumentException
            postMessage('example')       "example"
        */
        this.webView.CoreWebView2.WebMessageReceived += (/*object*/ s, /*CoreWebView2WebMessageReceivedEventArgs*/ e) => { model.Actions( e.TryGetWebMessageAsString() ); };
        this.webView.NavigationCompleted += (/*object*/ s, /*CoreWebView2NavigationCompletedEventArgs*/ e) => { model.Actions( e.ToString().to_json() ); };
        NamedPipeSingleton.GetInstance().PipeMessageReceived += (s, e) => { this.Dispatcher.Invoke(() => { model.Actions(e); }); };

        this.webView.CoreWebView2.AddHostObjectToScript("SimpleGuiMmf", MemoryMapSingleton.GetInstance());
    
    }
}

public struct A {
    public string type {get; set; }
    public string[] payload {get; set; }
}

public static class ActionEx{
    public static string to_json(this string src) => JsonSerializer.Serialize(new A { type = "message", payload = new string[]{ src } });
}
