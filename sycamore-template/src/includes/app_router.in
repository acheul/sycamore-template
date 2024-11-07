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