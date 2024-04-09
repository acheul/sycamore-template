use crate::*;

#[component]
pub fn App<G: Html>() -> View<G> {
  view! {
    main() {
      "Hello Sycamore!"
    }
  }
}