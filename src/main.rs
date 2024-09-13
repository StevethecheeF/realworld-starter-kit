mod home_article_list_item;
mod article_comment_list;
mod helper;
mod article_comment_list_item;

mod pages;
mod types;
mod components;

use leptos_router::*;
use leptos::*;
use crate::types::UserInfo;

use pages::{
    login::Login,
    register::Register, setting::Setting,
    profile::Profile,
    editor::Editor,
    article::Article, home::Home
};

use components::{
    footer::Footer,
    navigation::Navigation
};


fn main() {
    let user_info = create_rw_signal(UserInfo::default());
    provide_context(user_info);

    mount_to_body(|| view! {
        <Navigation/>
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
        <Footer/>
    })
}