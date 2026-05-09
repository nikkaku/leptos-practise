// mod components;
// use components::counter::Counter;
// use leptos::prelude::*;

mod components;
mod app;
use app::App;
use leptos::mount::mount_to_body;

fn main() {
  // leptos::mount::mount_to_body(|| view! {
  //   <p>"hello world!"</p>
  //   <Counter />
  // });
  mount_to_body(App);
}
