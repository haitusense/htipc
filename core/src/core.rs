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
pub struct Cli {
  #[command(subcommand)]
  pub command: Commands,
}

#[derive(Subcommand, Debug)]
#[command(author, version, about, long_about = None)]
pub enum Commands {
  #[command(about = "help for hoge")]
  Hello {
    #[arg(short, long)]
    opt: String,
  },
  #[command(about = "help for fuga")]
  PIPE(PipeArgs),
}

#[derive(Args, Serialize, Deserialize, Debug)]
pub struct PipeArgs {
  #[arg(num_args(1))] 
  pub pipename: String,

  #[arg(num_args(0..))] 
  pub commands: Option<Vec<String>>,
}

impl PipeArgs {
  pub fn to_json(&self) -> String { serde_json::to_string(self).unwrap() }
  pub fn to_value(&self) -> serde_json::Value { serde_json::to_value(self).unwrap() }
  pub fn get_command_string(&self) -> String { 
    match self.commands.clone() {
      Some(n) => n.join(" "),
      None => "".to_string()
    }
  }
}