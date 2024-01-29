using System;
using System.Diagnostics;
using System.Text;
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
        this.webView.CoreWebView2InitializationCompleted += WebView2InitializationCompleted;
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

        this.webView.NavigationCompleted += webView_NavigationCompleted;
        this.webView.CoreWebView2.WebMessageReceived += MessageReceived;

        model = await MainModel.Build(this);
        this.webView.CoreWebView2.AddHostObjectToScript("SimpleGuiMmf", MemoryMapSingleton.GetInstance());
        NamedPipeSingleton.GetInstance().Run("SimpleGui", this, (n, m) => model.Actions(n) );
    }
    
    private void WebView2InitializationCompleted(object sender, CoreWebView2InitializationCompletedEventArgs e) {
        Console.WriteLine("WebView2InitializationCompleted");
    }

    private void webView_NavigationCompleted(object sender, CoreWebView2NavigationCompletedEventArgs e) {
        Console.WriteLine("NavigationCompleted");
        // this.statusLabel.Text = "webView_NavigationCompleted";
        model.Actions("", e.ToString());
    }

    private /*async*/ void MessageReceived(object sender, CoreWebView2WebMessageReceivedEventArgs args) {
        /*
            postMessage({'a': 'b'})      ArgumentException
            postMessage(1.2)             ArgumentException
            postMessage('example')       "example"
        */
        var json = args.TryGetWebMessageAsString();
        // Console.WriteLine($" MessageReceived : {json} ");
        model.Actions(json);
    }
}

