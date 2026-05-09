use leptos::prelude::*;
use leptos_router::{
  components::{Router, Routes, Route},
  hooks::{ use_params },
  params::Params,
  path
};

use crate::components::header::Header;
use crate::components::footer::Footer;

#[component]
pub fn App() -> impl IntoView {
  view! {
    <Router>
      <Header />
      <main>
        <Routes fallback=|| "Not found.">
          <Route path=path!("/") view=Home/>
          <Route path=path!("/post/:id") view=BlogPost/>
        </Routes>
      </main>
      <Footer />
    </Router>
  }
}

#[component]
fn Home() -> impl IntoView { view! { <h1>"home"</h1> } }

#[derive(Params, PartialEq)]
struct ContactParams {
  id: Option<usize>
}

#[component]
fn BlogPost() -> impl IntoView {
  let params = use_params::<ContactParams>();
  let id = move || {
    params
        .read()
        .as_ref()
        .ok()
        .and_then(|params| params.id)
        .unwrap_or_default()
  };
  view! { <article>"article id:"{id}</article> }
}
