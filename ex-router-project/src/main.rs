use sycamore::prelude::*;
use sycamore::futures::spawn_local_scoped;
use wasm_bindgen::prelude::*;

mod app;

fn main() {
  sycamore::render(app::App);
}