mod core;
use colored::Colorize;
use core::*;
use clap::Parser;


fn main() {
  let cli = Cli::parse();
  match cli.command {
    Commands::Hello { opt } => {
      println!("hello {} !!", opt);
    }
    Commands::PIPE( args) => {
      cprintln!(blue: "connecting", args.pipename);
      cprintln!(blue: "sending",  args.get_command_string());
      println!("{}", args.to_json());
      println!("{:?}", args.to_value());

      match core::namedpipe::send(
        args.pipename.as_str(), 
        args.get_command_string().as_str()
      ) {
        Ok(src) => cprintln!(green: "received", src),
        Err(e) => cprintln!(red: "ERROR", e)
      };
    }
  }

}