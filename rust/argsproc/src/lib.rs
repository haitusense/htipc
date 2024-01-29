extern crate proc_macro;
use quote::quote;
use syn::{parse_macro_input, ItemFn, ItemStruct};

#[proc_macro_derive(HelloMacro)]
pub fn derive_hellomacro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let item = parse_macro_input!(input as ItemStruct);

  // let parsed = parse_macro_input!(input as syn::DeriveInput);
  println!("{:?}", item);

  let struct_name = item.ident;
  let gen = quote! {

    impl #struct_name {
      pub fn a<'a>() -> &'a str {
        stringify!(#struct_name)
      }
    }

  };
  gen.into()
}

#[proc_macro_derive(PyRffi)]
pub fn derive_pyrffi(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let item = parse_macro_input!(input as ItemStruct);
  let struct_name = item.ident;
  let gen = quote! {

    #[cfg(feature="python")]
    use pyo3::prelude::*;
    #[cfg(feature="python")]
    use pyo3::types::{PyDict, PyTuple};
    #[cfg(feature="r-lang")]
    use extendr_api::*;

    impl #struct_name {
      #[cfg(feature="python")]
      pub fn from_clap_py(args: &pyo3::types::PyTuple, kwargs:Option<&pyo3::types::PyDict>) -> anyhow::Result<Self> {
        let mut vec_args: Vec<String> = args.into_iter().map(|x| x.to_string() ).collect();
        let mut vec_kwargs: Vec<String> = match kwargs {
          None => Vec::new(),
          Some(n) => n.into_iter().fold(Vec::new(), |mut acc, i| {
            acc.push(format!("--{}", i.0));
            acc.push(i.1.to_string().to_lowercase()); // True -> true
            acc
          })
        };
        let mut dst = vec![stringify!(#struct_name).to_string()];
        dst.append(&mut vec_args);
        dst.append(&mut vec_kwargs);
        Self::from_clap_vec(dst)
      }

      #[cfg(feature="r-lang")]
      pub fn from_clap_robj(args: Robj) -> anyhow::Result<Self> {
        use extendr_api::prelude::*;
        let vec : Vec<(&str, Robj)> = args.as_list().unwrap().iter().collect();
        let dst : Vec<String> = vec.iter().fold(
          vec![stringify!(#struct_name).to_string()],
          |mut acc, n|{
            match n.0 {
              "" => { },
              "NA" => { },
              _ => { acc.push(format!("--{}", n.0)) },
            };
            let m = R!("eval({{ &n.1 }})").unwrap();
            match m.rtype() {
              Rtype::Logicals => acc.push(m.as_bool().unwrap().to_string()),
              Rtype::Integers => acc.push(m.as_integer().unwrap().to_string()),
              Rtype::Doubles => acc.push(m.as_real().unwrap().to_string()),
              Rtype::Rstr => acc.push(m.as_str().unwrap().to_string()),
              Rtype::Strings => acc.push(m.as_str().unwrap().to_string()),
              _=> panic!("Syntax error")
            };
            acc
          }
        );
        Self::from_clap_vec(dst)
      }

      pub fn from_clap_vec<I, T>(itr: I) -> anyhow::Result<Self>
      where I: IntoIterator<Item = T>, T: Into<std::ffi::OsString> + Clone, {
        use anyhow::Context as _;
        use clap::{CommandFactory, FromArgMatches};
        let am = Self::command().try_get_matches_from(itr).context("err")?;
        let dst = Self::from_arg_matches(&am).context("err")?;
        Ok(dst)
      }
    }

  };
  gen.into()
}

#[proc_macro_derive(PyRffiSerde)]
pub fn derive_serde(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let item = parse_macro_input!(input as ItemStruct);
  let struct_name = item.ident;
  let gen = quote! {

    impl #struct_name {

      #[cfg(feature="python")]
      pub fn to_strcut_pyany<'p>(&self, py: Python<'p>) -> anyhow::Result<&'p PyAny> {
        let obj: &'p PyAny = serde_pyobject::to_pyobject(py, &self).context("err")?
        Ok(obj)
      }

      #[cfg(feature="r-lang")]
      pub fn to_strcut_pairlist(&self) -> anyhow::Result<Pairlist> {
        let a = serde_json::to_value(self).context("err")?;
        let b = a.as_object().context("err")?.iter().filter_map(|n|{
          if let Some(m) = n.1.as_i64() { return Some((n.0, r!(m))); }
          if let Some(m) = n.1.as_f64() { return Some((n.0, r!(m))); }
          if let Some(m) = n.1.as_str() { return Some((n.0, r!(m))); }
          if let Some(m) = n.1.as_bool() { return Some((n.0, r!(m))); }
          None
        }).collect::<Vec<_>>();
        Ok(Pairlist::from_pairs(&b))
      }
    }

  };
  gen.into()
}



#[proc_macro_attribute]
pub fn show_streams(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
  println!("attr: \"{}\"", attr.to_string());
  println!("item: \"{}\"", item.to_string());

  let itemfn = parse_macro_input!(item as ItemFn);
  // let name = itemfn.
  println!(r#"itemfn: "{:?}""#, &itemfn.sig.ident);
  let fn_name = &itemfn.sig.ident;
  let gen = quote! {

    pub fn #fn_name () {
      println!(stringify!(#fn_name));
    }

  };
  gen.into()
}

  // println!("option a = {}", args.to_real("a").unwrap_or(-1f64) );
  // println!("option b = {}", args.to_char("b").unwrap_or("na") );
  // let mut send_string = String::new();
  // // RObj -> Option<&'a str>
  // if let Some(n) = value.as_str() {
  //   send_string = n.to_string();
  // }
  // // Rtype::List
  // if value.rtype() == Rtype::List {
  //   let robj = R!("jsonlite::toJSON({{ value }})").unwrap();
  //   send_string = robj.as_str().unwrap().to_string();
  // }