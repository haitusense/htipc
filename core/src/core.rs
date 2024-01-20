pub mod namedpipe;
// pub mod memorymappedfile;
use clap::{Parser, Subcommand, Args};
use serde::{Serialize, Deserialize};

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

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Commands,
}

#[derive(Subcommand, Debug)]
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


#[derive(Args, Serialize, Deserialize, Debug)]
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
  pub fn to_json(&self) -> String { serde_json::to_string(self).unwrap() }
  pub fn to_value(&self) -> serde_json::Value { serde_json::to_value(self).unwrap() }
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
  println!("{}", args.to_json());
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
