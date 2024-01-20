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
using System.Text.RegularExpressions;
namespace SimpleGUI;

/// <summary>
/// Interaction logic for MainWindow.xaml
/// </summary>
public partial class MainWindow : Window {
    MainModel model;

    public MainWindow() {
        InitializeComponent();
        Application.Current.DispatcherUnhandledException += OnDispatcherUnhandledException;
    }

    private void OnDispatcherUnhandledException(object sender, System.Windows.Threading.DispatcherUnhandledExceptionEventArgs e) {
        MessageBox.Show(e.Exception.ToString(), "DispatcherUnhandledException");
        Environment.Exit(1);
    }

    private async void Window_Loaded(object sender, RoutedEventArgs e) {
        var webview_options = new CoreWebView2EnvironmentOptions("--allow-file-access-from-files");
        var environment = await CoreWebView2Environment.CreateAsync(null, null, webview_options);
        await webView.EnsureCoreWebView2Async(environment);
        webView.NavigationCompleted += webView_NavigationCompleted;
        webView.CoreWebView2.WebMessageReceived += MessageReceived;

        model = await MainModel.Build(this, webView);
    }

    private void webView_NavigationCompleted(object sender, Microsoft.Web.WebView2.Core.CoreWebView2NavigationCompletedEventArgs e) {
        // this.statusLabel.Text = "webView_NavigationCompleted";
    }

    private /*async*/ void MessageReceived(object sender, CoreWebView2WebMessageReceivedEventArgs args) {
        model.Actions(args.TryGetWebMessageAsString());
    }
}