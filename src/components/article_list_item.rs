use leptos::*;
use crate::types::data_beans::ArticleInfo;
use crate::helper::favorite_article_action;

#[component]
pub fn ArticleListItem(article: ArticleInfo) -> impl IntoView {
  let (article_clone,set_article_clone) = create_signal(article);
  let author = article_clone.get().author;
  let author_profile_url = "/profile/".to_string() + &author.username.clone();
  let article_url = "/article/".to_string() + &article_clone.get().slug.clone();
  let created_at = article_clone.get().created_at.format("%B %e, %Y").to_string();
  let (favorite_count, set_favorite_count) = create_signal(article_clone.get().favorites_count);

  let favorite_article_action = create_action(move |_| {
    async move {
      let data = favorite_article_action(article_clone.get().favorited,&*article_clone.get().slug).await;
      if let Some(article_info) = data {
        set_article_clone(article_info.clone());
        set_favorite_count(article_info.favorites_count);
        Some(())
      }else {
        None
      }
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
            {article_clone.get().tag_list.into_iter()
              .map(|tag| view! { <li class="tag-default tag-pill tag-outline">{tag}</li>})
              .collect_view()}
          </ul>
        </a>
      </div>
  }
}
