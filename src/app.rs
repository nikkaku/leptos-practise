use crate::components::header::Header;
use crate::components::footer::Footer;
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
  view! {
    <Header />
    <main>
      "main"
    </main>
    <Footer />
  }
}
