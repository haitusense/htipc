using System;
using System.Runtime.InteropServices;
using System.ComponentModel;

// https://qiita.com/soi/items/d0c83a0cc3a4b23237ef

[ClassInterface(ClassInterfaceType.AutoDual)]
[ComVisible(true)]
public class State : INotifyPropertyChanged {

  private State() { }
  private static State _instance;
  public static State GetInstance() {
    if (_instance == null) { _instance = new State(); }
    return _instance;
  }

  public event PropertyChangedEventHandler PropertyChanged;
  private void RaisePropertyChanged([System.Runtime.CompilerServices.CallerMemberName]string propertyName = null) {
    PropertyChanged?.Invoke(this, new PropertyChangedEventArgs(propertyName));
    // window.webView.ExecuteScriptAsync($$""" drawFromMemoryMap("canvas", {{window.model.shift}}) """);
    // window?.webView.CoreWebView2.PostWebMessageAsString(propertyName);
  }
  public bool addEventListener(string name, dynamic act) {
    Console.WriteLine(act);
    return true;
  }

  public object this[string propertyName] {
    get => typeof(State).GetProperty(propertyName).GetValue(this);
    set { typeof(State).GetProperty(propertyName).SetValue(this, value); }
  }

  private string _Name = "Hejlsberg";
  public string Name {
    get => _Name;
    set { 
      if (value == _Name) return;
      _Name = value;
      RaisePropertyChanged();
      // RaisePropertyChanged(nameof(FullName));
    }
  }

  private string _Label = "";
  public string Label {
    get => _Label;
    set { 
      if (value == _Label) return;
      _Label = value;
      RaisePropertyChanged();
    }
  }

  private int _Shift = 0;
  public int Shift {
    get => _Shift;
    set { 
      if (value == _Shift) return;
      _Shift = value;
      RaisePropertyChanged();
    }
  }

}