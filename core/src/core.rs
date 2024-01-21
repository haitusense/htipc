pub mod namedpipe;
pub mod memorymappedfile;
use anyhow::Context as _;

use colored::Colorize;

// debug print

#[macro_export]
macro_rules! cprintln {
  ($a:expr, $b:expr) => { println!("{:>12} {:?}", $a.bold(), $b) };
  (red : $a:expr, $b:expr) => { println!("{:>12} {:?}", $a.red().bold(), $b) };
  (green : $a:expr, $b:expr) => { println!("{:>12} {:?}", $a.green().bold(), $b) };
  (blue : $a:expr, $b:expr) => { println!("{:>12} {:?}", $a.blue().bold(), $b) };
  (yellow : $a:expr, $b:expr) => { println!("{:>12} {:?}", $a.yellow().bold(), $b) };
  (magenta : $a:expr, $b:expr) => { println!("{:>12} {:?}", $a.magenta().bold(), $b) };
  (purple : $a:expr, $b:expr) => { println!("{:>12} {:?}", $a.purple().bold(), $b) };
}

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Commands,
}

#[derive(clap::Subcommand, Debug)]
pub enum Commands {
  #[command(about = "Hello World :-)")]
  Hello {
    #[arg(short, long)]
    opt: String,
  },
  #[command(about = "Send Message using NamedPipe")]
  PIPE(PipeArgs),
  
  #[command(about = "Search NamedPipe by wildcard pattern")]
  Search{
    #[arg(num_args(1), help = "wildcard pattern")] 
    pipename: String,
  },
}

// commandを使用するため、clap::Args -> clap::Parser
#[derive(clap::Parser, serde::Serialize, serde::Deserialize, Debug)]
pub struct PipeArgs {
  #[arg(num_args(1))] 
  pub addr: String,

  #[arg(num_args(0..))] 
  pub commands: Option<Vec<String>>,

  #[arg(short, long, help = "default : inf.[ms]")] 
  pub connect_timeout: Option<u32>,

  #[arg(short, long, help = "default : inf.[ms]")] 
  pub read_timeout: Option<u32>,

  #[arg(short, long, help = "default : inf.[ms]")] 
  pub write_timeout: Option<u32>,

  #[arg(short, long, default_value_t = false, help = "use serialize json")] 
  pub json: bool,
}

impl PipeArgs {
  #[cfg(feature="python")]
  pub fn from_pydict(args: &pyo3::types::PyAny, kwargs: Option<&pyo3::types::PyDict>) -> anyhow::Result<Self> {
    let args = serde_pyobject::from_pyobject::<serde_json::Value>(args).context("err")?;
    let kwargs = match kwargs {
      Some(n) => serde_pyobject::from_pyobject::<serde_json::Value>(n).context("err")?,
      None => serde_json::json!({})
    };
    let dst = Self::from_args_value(args, kwargs)?;
    Ok(dst)
  }
  pub fn from_args_vec<I, T>(itr: I) -> anyhow::Result<Self>
    where I: IntoIterator<Item = T>, T: Into<std::ffi::OsString> + Clone, {
    use clap::{CommandFactory, FromArgMatches};
    let am = Self::command().try_get_matches_from(itr).context("err")?;
    let dst = Self::from_arg_matches(&am).context("err")?;
    Ok(dst)
  }
  pub fn from_args_value(args:serde_json::Value, kwargs:serde_json::Value) -> anyhow::Result<Self> {
    let mut args_vec = vec!["Cli".to_string()];
    for val in args.as_array().context("err")? {
      args_vec.push(format!("{}", val.to_string()));
    }
    let object = kwargs.as_object().context("err")?;
    for (key, value) in object {
      args_vec.push(format!("--{}", key));
      args_vec.push(format!("{}", value.to_string()));
    }
    let dst = Self::from_args_vec(args_vec)?;
    Ok(dst)
  }
  pub fn to_value(&self) -> anyhow::Result<serde_json::Value> {
    Ok(serde_json::to_value(&self).context("err")?)
  }
  pub fn to_json(&self) -> anyhow::Result<String> { Ok(serde_json::to_string(self).context("err")?) }

  pub fn get_command_string(&self) -> String { 
    match (self.commands.clone(), self.json) {
      (Some(n), false) => format!("{}", n.join(" ")),
      (Some(n), true) => serde_json::to_string(&n).unwrap(),
      (None, true) => serde_json::json!({ }).to_string(),
      _ => "".to_string()
    }
  }
  pub fn get_addr_string(&self) -> String { format!(r##"\\.\pipe\{}"##, self.addr) }
}

pub fn namedpipe(args:PipeArgs) -> &'static str {
  cprintln!(blue: "connecting", args.addr);
  cprintln!(blue: "sending",  args.get_command_string());
  println!("{:?}", args.to_json());
  println!("{:?}", args.to_value());
  match namedpipe::send(args) {
    Ok(src) => {
      cprintln!(green: "received", src);
      "OK"
    },
    Err(e) => {
      cprintln!(red: "ERROR", e);
      "ERR"
    }
  }
}
