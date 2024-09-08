use leptos::*;
use super::types::*;
use gloo::storage::{LocalStorage, Storage};

#[component]
pub fn ArticleCommentListItem(comment: CommentInfo) -> impl IntoView {
	let user_info = expect_context::<RwSignal<UserInfo>>();
	let author_username = move || comment.author.username.clone();
	let creation_date = move || comment.created_at.format("%B %e, %Y").to_string();
	let profile_link = move |username:&str| {
		"/profile/".to_string() +  username
	};

	/*let delete_comment_action = create_action(move |_| {
		let comment_id = comment.id.clone();
		async move {
			let client  = reqwest::Client::new();
			let mut builder = client
				.delete("http://localhost:3000/api/articles/".to_owned()+&*slug_clone+"/comments/"+&*comment_id.to_string())
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

			let data = response.json::<CommentInfoWrapper>().await.ok()?;
			Some(())
		}
	});*/

	let delete_comment = move |_| {
		logging::log!("todo");
		//delete_comment_action.dispatch(());
	};

	view! {

	<div class="card">
	  <div class="card-block">
		<p class="card-text">
		  {comment.body}
		</p>
	  </div>
	  <div class="card-footer">
		<a href=profile_link(&*author_username()) class="comment-author">
		  <img src=comment.author.image class="comment-author-img" />
		</a>
		<a href=profile_link(&*author_username()) class="comment-author">{author_username()}</a>
		<span class="date-posted">{creation_date}</span>
		<Show
			when=move || Some(author_username()) == user_info().username
			fallback= move || view! {}
		>
			<span class="mod-options" on:click=delete_comment>
			  <i class="ion-trash-a"></i>
			</span>
		</Show>
	  </div>
	</div>
	}
}
