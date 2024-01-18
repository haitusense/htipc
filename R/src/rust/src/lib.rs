use extendr_api::prelude::*;
use colored::Colorize;

#[macro_export]
macro_rules! rprintln {
  () => {
    R!("message()").unwrap();
  };
  ($($arg:tt)*) => {{
    let out = format!($($arg)*);
    R!("message( {{ out }} )").unwrap();
  }};
}

/*
htipcR::namedPipe("namedPipe", "value")

dst <- list(type = jsonlite::unbox("text"), payload = list(id = "text", value = "aa"))
htipcR::namedPipe("namedPipe", dst)

suppressMessages(
invisible(capture.output(htipcR::namedPipe("namedPipe", "value"), type = "message")) 
*/
#[allow(non_snake_case)]
#[extendr]
fn namedpipe(path: &str, value: Robj, op: Robj) -> String {

  // let del = args.to_char("delimiter").unwrap_or("_");
  // println!("delimiter = {}", del );
  // let path_vec = rpxlog_core::path_split(path, del)?;
  // println!(" -> {:?}", path_vec );
  // println!("option a = {}", args.to_real("a").unwrap_or(-1f64) );
  // println!("option b = {}", args.to_char("b").unwrap_or("na") );
  
  let mut send_string = String::new();
  // RObj -> Option<&'a str>
  if let Some(n) = value.as_str() {
    send_string = n.to_string();
  }
  // Rtype::List
  if value.rtype() == Rtype::List {
    let robj = R!("jsonlite::toJSON({{ value }})").unwrap();
    send_string = robj.as_str().unwrap().to_string();
  }

  rprintln!("{:>12} {:?}", "connecting".blue().bold(), path);
  rprintln!("{:>12} {:?}", "sending".blue().bold(), send_string);
  match htipc::namedpipe(path, send_string.as_str()){
    Ok(dst) =>{
      rprintln!("{:>12} {:?}", "received".green().bold(), dst);
      dst.lines().collect::<String>()
    },
    Err(e) => {
      rprintln!("{}", e.to_string().red());
      "".to_string()
    }
  }
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
  mod htipcR;
  fn namedpipe;
}
