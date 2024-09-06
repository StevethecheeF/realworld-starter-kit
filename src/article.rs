use leptos::*;
use super::types::*;

#[component]
pub fn Article(article:ReadSignal<ArticleInfo>) -> impl IntoView {
    let user_info = expect_context::<RwSignal<UserInfo>>();
    let (user_info_username, _) = create_slice(
      user_info,
      |user_info| user_info.username.clone(),
      |user_info, username| user_info.username = username,
    );
    let can_edit = move || {
      match user_info_username() {
        Some(v) => v == article.get().author.username,
        None => false,
      }
    };
    let profile_link = move || {
      let prefix = "/profile/";
      let username = article.get().author.username;
      return prefix.to_string() + &*username;
    };
    let creation_date = move || {
      return article.get().createdAt.format("%B %e, %Y").to_string();
    };
    view! {
    <div class="article-page">
      <div class="banner">
        <div class="container">
          <h1>How to build webapps that scale</h1>

          <div class="article-meta">
            <a href="/profile/eric-simons"><img src=article.get().author.image alt=article.get().author.username /></a>
            <div class="info">
              <a href=profile_link class="author">{article.get().author.username}</a>
              <span class="date">{creation_date}</span>
            </div>
            <button class="btn btn-sm btn-outline-secondary">
              <i class="ion-plus-round"></i>
              {move || if article.get().author.following {
                "Follow ".to_string() + &*article.get().author.username
              } else {
                "Unfollow ".to_string() + &*article.get().author.username
              }}
              <span class="counter"></span>
            </button>
            &nbsp;&nbsp;
            <button class="btn btn-sm btn-outline-primary">
              <i class="ion-heart"></i>
              Favorite Post <span class="counter">({article.get().favoritesCount})</span>
            </button>
            <Show
              when=move || {can_edit()}
              fallback=move || view! {<span></span>}
            >
              <button class="btn btn-sm btn-outline-secondary">
                <i class="ion-edit"></i> Edit Article
              </button>
              <button class="btn btn-sm btn-outline-danger">
                <i class="ion-trash-a"></i> Delete Article
              </button>
            </Show>
          </div>
        </div>
      </div>

      <div class="container page">
        <div class="row article-content">
          <div class="col-md-12">
            <p>
              Web development technologies have evolved at an incredible clip over the past few years.
            </p>
            <h2 id="introducing-ionic">Introducing RealWorld.</h2>
            <p>It s a great solution for learning how other frameworks work.</p>
            <ul class="tag-list">
              <li class="tag-default tag-pill tag-outline">realworld</li>
              <li class="tag-default tag-pill tag-outline">implementations</li>
            </ul>
          </div>
        </div>

        <hr />

        <div class="article-actions">
          <div class="article-meta">
            <a href="profile.html"><img src="http://i.imgur.com/Qr71crq.jpg" /></a>
            <div class="info">
              <a href="" class="author">Eric Simons</a>
              <span class="date">January 20th</span>
            </div>

            <button class="btn btn-sm btn-outline-secondary">
              <i class="ion-plus-round"></i>
              &nbsp; Follow Eric Simons
            </button>
            &nbsp;
            <button class="btn btn-sm btn-outline-primary">
              <i class="ion-heart"></i>
              &nbsp; Favorite Article <span class="counter">(29)</span>
            </button>
            <button class="btn btn-sm btn-outline-secondary">
              <i class="ion-edit"></i> Edit Article
            </button>
            <button class="btn btn-sm btn-outline-danger">
              <i class="ion-trash-a"></i> Delete Article
            </button>
          </div>
        </div>

        <div class="row">
          <div class="col-xs-12 col-md-8 offset-md-2">
            <form class="card comment-form">
              <div class="card-block">
                <textarea class="form-control" placeholder="Write a comment..." rows="3"></textarea>
              </div>
              <div class="card-footer">
                <img src="http://i.imgur.com/Qr71crq.jpg" class="comment-author-img" />
                <button class="btn btn-sm btn-primary">Post Comment</button>
              </div>
            </form>

            <div class="card">
              <div class="card-block">
                <p class="card-text">
                  With supporting text below as a natural lead-in to additional content.
                </p>
              </div>
              <div class="card-footer">
                <a href="/profile/author" class="comment-author">
                  <img src="http://i.imgur.com/Qr71crq.jpg" class="comment-author-img" />
                </a>
                &nbsp;
                <a href="/profile/jacob-schmidt" class="comment-author">Jacob Schmidt</a>
                <span class="date-posted">Dec 29th</span>
              </div>
            </div>

            <div class="card">
              <div class="card-block">
                <p class="card-text">
                  With supporting text below as a natural lead-in to additional content.
                </p>
              </div>
              <div class="card-footer">
                <a href="/profile/author" class="comment-author">
                  <img src="http://i.imgur.com/Qr71crq.jpg" class="comment-author-img" />
                </a>
                &nbsp;
                <a href="/profile/jacob-schmidt" class="comment-author">Jacob Schmidt</a>
                <span class="date-posted">Dec 29th</span>
                <span class="mod-options">
                  <i class="ion-trash-a"></i>
                </span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
    }
}