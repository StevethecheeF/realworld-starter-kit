use leptos::*;
use crate::types::*;
use gloo::storage::{LocalStorage, Storage};

#[component]
pub fn HomeArticleListItem(article: ArticleInfo) -> impl IntoView {
  let (article_clone,set_article_clone) = create_signal(article);
  let author = article_clone.get().author;
  let author_profile_url = "/profile/".to_string() + &author.username.clone();
  let article_url = "/article/".to_string() + &article_clone.get().slug.clone();
  let created_at = article_clone.get().createdAt.format("%B %e, %Y").to_string();
  let (favorite_count, set_favorite_count) = create_signal(article_clone.get().favoritesCount);

  let favorite_article_action = create_action(move |_| {
    let client = reqwest::Client::new();
    let mut url = "http://localhost:3000/api/articles/".to_owned();
    url.push_str(&*article_clone.get().slug);
    url.push_str("/favorite");

    async move {
      let mut builder;
      logging::log!("{:?}",article_clone.get().favorited);
      if article_clone.get().favorited {
        builder = client.delete(url.to_owned());
      } else {
        builder = client.post(url.to_owned());
      }
      builder = builder.header("Content-Type", "application/json");

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

      let data = response.json::<ArticleInfoWrapper>().await.ok()?;
      set_article_clone(data.article.clone());
      set_favorite_count(data.article.favoritesCount);
      Some(())
    }
  });

  let favorite_article = move |_| {
    favorite_article_action.dispatch(());
  };
  view! {
      <div class="article-preview">
        <div class="article-meta">
          <a href=author_profile_url.clone()><img src=author.image/></a>
          <div class="info">
            <a href=author_profile_url class="author">{author.username}</a>
            <span class="date">{created_at}</span>
          </div>
          <button class="btn btn-outline-primary btn-sm pull-xs-right" on:click=favorite_article>
            <i class="ion-heart"></i> {favorite_count}
          </button>
        </div>
        <a href=article_url class="preview-link">
          <h1>{article_clone.get().title}</h1>
          <p>{article_clone.get().description}</p>
          <span>Read more...</span>
          <ul class="tag-list">
            {article_clone.get().tagList.into_iter()
              .map(|tag| view! { <li class="tag-default tag-pill tag-outline">{tag}</li>})
              .collect_view()}
          </ul>
        </a>
      </div>
  }
}
