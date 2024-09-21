use clap::Parser;
use std::process::Command;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

#[derive(Parser, Debug)]
pub struct Config {
  /// crate's name
  #[arg(short, long, required=true)]
  name: String,
  /// use sycamore-router or not
  #[arg(long, default_value_t=false)]
  router: std::primitive::bool,
  /// data-trunk's copy-dir href
  #[arg(long)]
  copy_dir: Option<String>,
  /// data-trunk's css href
  #[arg(long)]
  css: Option<String>,
  /// favicon's href
  #[arg(long)]
  favicon: Option<String>,
  /// version of sycamore
  #[arg(short, long, default_value_t=String::from("0.9.0-beta.4"))]
  version: String
}

impl Config {
  /// run
  /// 1. cargo init --bin root+name
  /// 2. cargo add sycamore@version --features suspense; & wasm-bindgen; (with --package option)
  ///   2-1. cargo add sycamore-router@version
  /// 3. construct main.rs, app.rs and index.html
  pub fn run(&self) -> Result<String, String> {

    let name = self.name.as_str();

    Command::new("cargo").args(["init", "--bin", name])
      .output()
      .expect(format!(r"`cargo init --bin {}` failed", name).as_str());

    let package_ = format!("--package={name}");

    let syca = format!("sycamore@{}", self.version);
    Command::new("cargo").args(["add", syca.as_str(), "--features=suspense", package_.as_str()])
      .output()
      .expect(format!(r"`carg add {} --features=suspense {}` failed", syca, package_).as_str());

    Command::new("cargo").args(["add", "wasm-bindgen", package_.as_str()])
      .output().expect(format!(r"`cargo add wasm-bindgen {}` failed", package_).as_str());

    if self.router {
      let syca_router = format!("sycamore-router@{}", self.version);
      Command::new("cargo").args(["add", syca_router.as_str(), package_.as_str()])
        .output()
        .expect(format!(r"`carg add {} --features=suspense {}` failed", syca_router, package_).as_str());
    }


    let path = Path::new(name);
    let src = path.join("src");

    let mut f = File::create(path.join("index.html")).unwrap();
    if let Err(e) = f.write_all(index_literal(self.name.as_str(), self.copy_dir.clone(), self.css.clone(), self.favicon.clone()).as_bytes()) {
      return Err(format!("{:?}", e));
    }

    let mut f = File::create(src.join("main.rs")).unwrap();
    if let Err(e) = f.write_all(main_literal().as_bytes()) {
      return Err(format!("{:?}", e));
    }

    let mut f = File::create(src.join("app.rs")).unwrap();
    if let Err(e) = f.write_all(app_literal(self.router).as_bytes()) {
      return Err(format!("{:?}", e));
    }

    let msg = format!("Sycamore project {} just initiated!", self.name);
    Ok(msg)
  }
}


// literals //

pub fn index_literal(
  title: &str,
  copy_dir: Option<String>,
  css: Option<String>,
  favicon: Option<String>
) -> String {

  format!(r#"
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8"/>
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    {}
    {}
    {}
  </head>
</html>
  "#,
  title,
  copy_dir.map(|x| format!(r#"<link data-trunk rel="copy-dir" href="{x}"/>"#)).unwrap_or_default(),
  css.map(|x| format!(r#"<link data-trunk rel="css" href="{x}"/>"#)).unwrap_or_default(),
  favicon.map(|x| format!(r#"<link rel="icon" type="image/x-icon" href="{x}">"#)).unwrap_or_default()
  ).trim()
    .split("\n").filter_map(|x| if x.trim().len()>0 { Some(x)} else {None}).collect::<Vec<_>>().join("\n")
}

pub fn main_literal() -> String {
  r#"
use sycamore::prelude::*;
use sycamore::futures::spawn_local_scoped;
use wasm_bindgen::prelude::*;

mod app;

fn main() {
  sycamore::render(app::App);
}
  "#.trim().to_string()
}

pub fn app_literal(
  router: bool
) -> String {

  let x = if !router {
    r##"
use crate::*;

#[component]
pub fn App() -> View {
  view! {
    main() {
      "Hello Sycamore!"
    }
  }
}
    "##
  } else {
    r##"
use crate::*;
use sycamore_router::{Route, Router, HistoryIntegration};

#[derive(Clone, Route)]
pub enum Routes {
  #[to("/")] Index,
  #[not_found] NotFound
}

fn switch(route: ReadSignal<Routes>) -> View {
  
  let view = move || match route.get_clone() {
    Routes::Index => view! { "index" },
    Routes::NotFound => view! { "Not Found" },
  };
  
  view! { (view) }
}

#[component]
pub fn App() -> View {
  view! {
    main() {
      Router(
        integration=HistoryIntegration::new(),
        view=switch
      )
    }
  }
}
    "##
  };
  x.trim().to_string()
}