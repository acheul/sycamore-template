use crate::*;
use sycamore_router::{Route, Router, HistoryIntegration};

#[derive(Clone, Route)]
pub enum Routes {
  #[to("/")] Index,
  #[not_found] NotFound
}

fn switch<G: Html>(route: ReadSignal<Routes>) -> View<G> {
  
  let view = create_memo(on(route, move || match route.get_clone() {
    Routes::Index => view! { "index" },
    Routes::NotFound => view! { "Not Found" },
  }));
  
  view! { (view.get_clone()) }
}

#[component]
pub fn App<G: Html>() -> View<G> {
  view! {
    main() {
      Router(
        integration=HistoryIntegration::new(),
        view=switch
      )
    }
  }
}