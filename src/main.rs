mod navigation;
mod home_article_list_item;
mod article_comment_list;
mod helper;
mod article_comment_list_item;

mod pages;
mod types;

use leptos_router::*;
use leptos::*;
use crate::types::UserInfo;
use pages::login::Login;
use pages::register::Register;
use pages::setting::Setting;
use pages::profile::Profile;
use pages::editor::Editor;
use pages::article::Article;
use pages::home::Home;

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
              <Route path="/" view=Home/>/>
              <Route path="/login" view=Login/>
              <Route path="/register" view=Register/>
              <Route path="/profile/:user_id" view=Profile/>
              <Route path="/settings" view=Setting/>
              <Route path="/editor" view=Editor/>
              <Route path="/editor/:slug" view=Editor/>
              <Route path="/article/:slug" view=Article/>
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