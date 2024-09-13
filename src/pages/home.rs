use leptos::*;
use gloo::storage::{LocalStorage, Storage};
use crate::types::data_beans::{ArticleListInfo, TagListInfo, UserInfo};
use crate::types::{API_ENDPOINT, SESSION_TOKEN};
use crate::components::article_list_item::ArticleListItem;

#[component]
pub fn Home() -> impl IntoView {
  let user_info = expect_context::<RwSignal<UserInfo>>();
  let user_info_is_authenticated = create_read_slice(
    user_info,
    |user_info| user_info.is_authenticated(),
  );

  // tab handling
  let async_tag_list = create_resource(
    || (),
    move |_| async move {
      let client = reqwest::Client::new();
      let mut builder = client
          .get(format!("{}{}",API_ENDPOINT,"/tags"))
          .header("Content-Type", "application/json");

      if let Ok(token) = LocalStorage::get::<String>(SESSION_TOKEN) {
        builder = builder.bearer_auth(token);
      }
      let response = builder.send()
          .await
          .ok()?;

      if !response.status().is_success() {
        return None;
      }

      let data = response.json::<TagListInfo>().await.ok()?;
      Some(data)
    },
  );

  let (current_tab, set_current_tab) = create_signal("Global".to_string());
  let (tab_tag, set_tab_tag) = create_signal(String::default());

  // article data
  let (current_page, set_current_page) = create_signal(1);
  let (article_data, set_article_data) = create_signal(vec![]);
  let (article_count, set_article_count) = create_signal(0);

  let global_article_request = create_action(move |input: &Option<String>| {
    let client = reqwest::Client::new();
    let input_copy = input.to_owned();
    async move {
      let mut builder = client
          .get(format!("{}{}",API_ENDPOINT,"/articles"))
          .header("Content-Type", "application/json");
      if let Ok(token) = LocalStorage::get::<String>(SESSION_TOKEN) {
        builder = builder.bearer_auth(token);
      }
      let offset = (current_page.get() - 1) * 20;
      let mut query = vec![("offset",offset.to_string())];
      if let Some(tag) = input_copy {
         query.push(("tag", tag));
      }
      builder = builder.query(&query);

      let response = builder
          .send()
          .await
          .ok()?;
      if !response.status().is_success() {
        return None;
      }
      let data = response.json::<ArticleListInfo>().await.ok()?;
      set_article_data(data.articles);
      set_article_count(data.articles_count);
      Some(())
    }
  });

  let feed_article_request = create_action(move |_| {
    let client = reqwest::Client::new();
    let offset = (current_page.get() - 1) * 20;
    async move {
      let mut builder = client
          .get(format!("{}{}",API_ENDPOINT, "/articles/feed"))
          .header("Content-Type", "application/json");
      if let Ok(token) = LocalStorage::get::<String>(SESSION_TOKEN) {
        builder = builder.bearer_auth(token);
      }
      let query = vec![("offset",offset.to_string())];
      builder = builder.query(&query);

      let response = builder
          .send()
          .await
          .ok()?;
      if !response.status().is_success() {
        return None;
      }

      let data = response.json::<ArticleListInfo>().await.ok()?;
      set_article_data(data.articles);
      set_article_count(data.articles_count);
      Some(())
    }
  });

  let send_article_request = move |current_tab: String| {
    match current_tab.as_str() {
      "Global" => {
        global_article_request.dispatch(None);
      }
      "Personal" => {
        feed_article_request.dispatch(());
      }
      _ => {
        global_article_request.dispatch(Some(tab_tag.get()))
      }
    }
  };

  let async_article_list = create_resource(
    current_tab,
    move |current_tab| async move {
      set_current_page(1);
      send_article_request(current_tab.to_string());
    },
  );


  // pagination
  let available_pages = move || {
    let page_count = (article_count.get() / 20) + 2;
    (1..page_count).collect::<Vec<_>>()

  };
  let on_pagination_click = move |page: u32| {
    if current_page.get() != page {
      set_current_page(page);
      send_article_request(current_tab.get().to_string());
    }
  };

    view! {
        <div class="home-page">
          <div class="banner">
            <div class="container">
              <h1 class="logo-font">conduit</h1>
              <p>A place to share your knowledge.</p>
            </div>
          </div>

          <div class="container page">
            <div class="row">
              <div class="col-md-9">
                <div class="feed-toggle">
                  <ul class="nav nav-pills outline-active">
                    <li class="nav-item">
                      <a class=move|| if current_tab.get() == "Global" {"nav-link active"}else{"nav-link"} on:click=move |_| {set_current_tab("Global".to_string());set_tab_tag(String::default());}>Global Feed</a>
                    </li>
                    <Show
                      when=move || user_info_is_authenticated()
                    >
                      <li class="nav-item">
                        <a class=move|| if current_tab.get() == "Personal" {"nav-link active"}else{"nav-link"} on:click=move |_| {set_current_tab("Personal".to_string());set_tab_tag(String::default());}>Your Feed</a>
                      </li>
                    </Show>
                    <Show
                      when=move || tab_tag() != String::default()
                    >
                      <li class="nav-item">
                        <a class="nav-link active">#{tab_tag()}</a>
                      </li>
                    </Show>
                  </ul>
                </div>
                <For
                  each=article_data
                  key=|article| article.slug.clone()
                  let:child
                >
                  <ArticleListItem article=child />
                </For>
                <ul class="pagination">
                 <For
                    each=available_pages
                    key=|n| n.clone()
                    let:child
                  >
                    <li class="page-item">
                      <a class="page-link" href="" on:click=move |_| on_pagination_click(child.clone())>{child}</a>
                    </li>
                </For>
                </ul>
              </div>

              <div class="col-md-3">
                <div class="sidebar">
                  <p>Popular Tags</p>
                  {
                    move || {
                    match async_tag_list() {
                      Some(Some(list)) => {
                        view!{
                          <div class="tag-list">
                            <For
                              each=move || list.tags.clone()
                              key=|tag| tag.clone()
                              children=move |tag| {
                                let tag_owned =tag.to_owned();
                                let onclick = move |_| {
                                  set_tab_tag(tag_owned.clone());
                                  set_current_tab(tag_owned.clone()+"_tag");
                                };
                                view!{<a href="" class="tag-pill tag-default" on:click=onclick>{tag}</a>}
                             }
                            />
                          </div>
                        }.into_view()
                      }
                      _ => view! {<p>Loading ... </p>}.into_view()
                    }
                  }
                  }
                </div>
              </div>
            </div>
          </div>
        </div>
    }
}
