mod login;
mod register;
mod types;
mod navigation;
mod profile;
mod editor;
mod setting;
mod article;
mod home;
mod home_article_list_item;
mod article_comments;
mod helper;

use leptos_router::*;
use leptos::*;
use crate::types::UserInfo;

fn main() {
    let user_info = create_rw_signal(UserInfo::default());
    provide_context(user_info);

    mount_to_body(|| view! {
        <nav class="navbar navbar-light">
            <div class="container">
                <a class="navbar-brand" href="/">conduit</a>
                <navigation::Navigation/>
            </div>
        </nav>
        <Router>
          <main>
            <Routes>
              <Route path="/" view=home::Home/>/>
              <Route path="/login" view=login::Login/>
              <Route path="/register" view=register::Register/>
              <Route path="/profile/:user_id" view=profile::Profile/>
              <Route path="/settings" view=setting::Setting/>
              <Route path="/editor" view=editor::Editor/>
              <Route path="/editor/:slug" view=editor::Editor/>
              <Route path="/article/:slug" view=article::Article/>
            </Routes>
          </main>
        </Router>
        <footer>
            <div class="container">
                <a href="/" class="logo-font">conduit</a>
                <span class="attribution">
                    An interactive learning project from <a href="https://thinkster.io">Thinkster</a>. Code &amp;
                    design licensed under MIT.
                </span>
            </div>
        </footer>
    })
}