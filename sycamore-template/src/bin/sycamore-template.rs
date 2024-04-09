use clap::Parser;
use sycamore_template::Config;


fn main() {
  let cfg = Config::parse();
  match cfg.run() {
    Ok(msg) => println!("{}", msg),
    Err(err) => println!("{}", err),
  };
}