mod login;
mod register;
mod types;

use leptos_router::*;
use leptos::*;

fn main() {
    mount_to_body(|| view! {
        <Router>
          <main>
            <Routes>
              <Route path="/" view=|| view! { <h1>"Home page"</h1> }/>/>
              <Route path="/login" view=login::Login/>
              <Route path="/register" view=register::Register/>
            </Routes>
          </main>
        </Router>
    })
}