use leptos::*;
use super::types::*;
use gloo::storage::{LocalStorage, Storage};
use super::home_article_list_item;

#[component]
pub fn Home() -> impl IntoView {
  let user_info = expect_context::<RwSignal<UserInfo>>();
  let (user_info_is_authenticated, _) = create_slice(
    user_info,
    |user_info| user_info.token.clone(),
    |user_info, token| user_info.token = token,
  );

  // tab handling
  let async_tag_list = create_resource(
    || (),
    move |_| async move {
      let client = reqwest::Client::new();
      let mut builder = client
          .get("http://localhost:3000/api/tags")
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

  let (current_tab, set_current_tab) = create_signal(Tab::Global);
  let (tab_tag, set_tab_tag) = create_signal(String::default());

  // article data
  let (article_data, set_article_data) = create_signal(vec![]);
  let (article_count, set_article_count) = create_signal(0);

  let global_article_request = create_action(move |input: &Option<String>| {
    let client = reqwest::Client::new();
    let input_copy = input.to_owned();
    async move {
      let mut builder = client
          .get("http://localhost:3000/api/articles".to_owned())
          .header("Content-Type", "application/json");
      if let Ok(token) = LocalStorage::get::<String>(SESSION_TOKEN) {
        builder = builder.bearer_auth(token);
      }

      if let Some(tag) = input_copy {
        let query = vec![("tag",tag)];
        builder = builder.query(&query);
      }
      let response = builder
          .send()
          .await
          .ok()?;
      if !response.status().is_success() {
        return None;
      }
      let data = response.json::<ArticleListInfo>().await.ok()?;
      set_article_data(data.articles);
      set_article_count(data.articlesCount);
      Some(())
    }
  });

  let feed_article_request = create_action(move |_| {
    let client = reqwest::Client::new();
    async move {
      let mut builder = client
          .get("http://localhost:3000/api/articles/feed".to_owned())
          .header("Content-Type", "application/json");
      if let Ok(token) = LocalStorage::get::<String>(SESSION_TOKEN) {
        builder = builder.bearer_auth(token);
      }
      let response = builder
          .send()
          .await
          .ok()?;
      if !response.status().is_success() {
        return None;
      }

      let data = response.json::<ArticleListInfo>().await.ok()?;
      set_article_data(data.articles);
      set_article_count(data.articlesCount);
      Some(())
    }
  });

  let async_article_list = create_resource(
    current_tab,
    move |current_tab| async move {
      match current_tab {
        Tab::Global => {
          logging::log!("global");
          global_article_request.dispatch(None);
        }
        Tab::Personal => {
          logging::log!("personal");
          feed_article_request.dispatch(());
        }
        Tab::Keyword => {
          global_article_request.dispatch(Some(tab_tag.get()))
        }
      }
    },
  );

  // pagination
  let (current_page, set_current_page) = create_signal(1);
  let available_pages = move || (article_count.get() / 20) +1 ;


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
                    <Show
                      when=move || { user_info_is_authenticated().is_some() }
                    >
                      <li class="nav-item">
                        <a class="nav-link" href="" on:click=move |_| set_current_tab(Tab::Personal)>Your Feed</a>
                      </li>
                    </Show>
                    <li class="nav-item">
                      <a class="nav-link active" href="" on:click=move |_| set_current_tab(Tab::Global)>Global Feed</a>
                    </li>
                  </ul>
                </div>
                <For
                  each=article_data
                  key=|article| article.slug.clone()
                  let:child
                >
                  <home_article_list_item::HomeArticleListItem article=child />
                </For>
                <ul class="pagination">
                  {(0..(available_pages())).collect::<Vec<_>>().into_iter()
                    .map(|n| view! {
                      <li class="page-item active">
                        <a class="page-link" href="">{n}</a>
                      </li>
                  }).collect_view()}
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
                                  set_current_tab(Tab::Keyword);
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
