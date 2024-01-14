mod ipc;
use clap::Parser;
use colored::Colorize;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {

  #[arg(num_args(1))] 
  pipename: String,

  #[arg(num_args(0..))] 
  commands: Option<Vec<String>>,

}

fn main() {
  let args = Args::parse();

  // println!("{:?}", args.pipename);
  // println!("{:?}", args.commands);

  match ipc::ipc_write(
    args.pipename.as_str(), 
    args.commands.unwrap().join(" ").as_str()
  ) {
    Ok(src) => println!("{}", src.green().bold()),
    Err(e) => println!("{}", e.to_string().red().bold())
  };

}