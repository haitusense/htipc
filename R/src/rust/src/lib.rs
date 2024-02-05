#![allow(non_snake_case)]

use extendr_api::prelude::*;


/*
suppressMessages(
invisible(capture.output(htipcR::namedPipe("namedPipe", "value"), type = "message")) 
*/
#[extendr]
fn namedpipe(op: Robj) -> String {
  let pa = htipc::core::PipeArgs::from_clap_robj(op).unwrap();
  println!("{:?}", pa);
  let dst = htipc::core::namedpipe(pa).to_string();
  dst
}

#[extendr]
fn env() -> Pairlist {
  let v = htipc::env().unwrap().into_iter().collect::<Vec<_>>();
  Pairlist::from_pairs(&v)
}


#[extendr]
fn header(path: &str) -> Pairlist {
  let v = htipc::core::header(path).unwrap();
  let a = serde_json::to_value(v).unwrap();
  let b = a.as_object().unwrap().iter().filter_map(|n|{
    if let Some(m) = n.1.as_i64() { return Some((n.0, r!(m))); }
    if let Some(m) = n.1.as_f64() { return Some((n.0, r!(m))); }
    if let Some(m) = n.1.as_str() { return Some((n.0, r!(m))); }
    if let Some(m) = n.1.as_bool() { return Some((n.0, r!(m))); }
    None
  }).collect::<Vec<_>>();
  Pairlist::from_pairs(&b)
}


// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
  mod htipcR;
  fn namedpipe;
  fn env;
  fn header;
}
