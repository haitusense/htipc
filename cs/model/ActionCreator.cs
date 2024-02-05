using System;
using System.Collections.Generic;
using System.IO.MemoryMappedFiles;
using System.Runtime.InteropServices;
using System.IO;
using System.IO.Pipes;
using System.Windows.Controls;
using System.Numerics;
using System.CodeDom;
using System.Windows;
using System.Windows.Documents;
using System.Text.Json;

namespace SimpleGUI;

public struct Message {
    public string type {get; set; }
    public object payload {get; set; }
    public string to_json() => JsonSerializer.Serialize(this);
}

[ClassInterface(ClassInterfaceType.AutoDual)]
[ComVisible(true)]
public class ActionCreator {

  private ActionCreator() { }
  private static ActionCreator _instance;
  public static ActionCreator GetInstance() {
    if (_instance == null) { _instance = new ActionCreator(); }
    return _instance;
  }

  public string setUserName(string userName) {
    return new Message(){
      type = "name",
      payload = userName
    }.to_json();
  }

}