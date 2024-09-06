use leptos::*;
use crate::types::ArticleInfo;

#[component]
pub fn HomeArticleListItem(article: ArticleInfo) -> impl IntoView {

  let author = article.author;
  let author_profile_url = "/profile/".to_string() + &author.username.clone();
  let created_at = article.createdAt.format("%B %e, %Y").to_string();

  view! {
      <div class="article-preview">
        <div class="article-meta">
          <a href=author_profile_url.clone()><img src=author.image/></a>
          <div class="info">
            <a href=author_profile_url class="author">{author.username}</a>
            <span class="date">{created_at}</span>
          </div>
          <button class="btn btn-outline-primary btn-sm pull-xs-right">
            <i class="ion-heart"></i> {article.favoritesCount}
          </button>
        </div>
        <a href="/article/how-to-build-webapps-that-scale" class="preview-link">
          <h1>{article.title}</h1>
          <p>{article.description}</p>
          <span>Read more...</span>
          <ul class="tag-list">
            {article.tagList.into_iter()
              .map(|tag| view! { <li class="tag-default tag-pill tag-outline">{tag}</li>})
              .collect_view()}
          </ul>
        </a>
      </div>
  }
}
