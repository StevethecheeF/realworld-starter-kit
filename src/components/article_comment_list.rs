use leptos::*;
use gloo::storage::{LocalStorage, Storage};
use crate::types::data_beans::{CommentCreateInfo, CommentCreateInfoWrapper, CommentInfoWrapper, CommentListInfo, UserInfo};
use crate::types::{API_ENDPOINT, SESSION_TOKEN};
use crate::components::article_comment_list_item::ArticleCommentListItem;


#[component]
pub fn ArticleCommentList(slug:String) -> impl IntoView {
	let user_info = expect_context::<RwSignal<UserInfo>>();
	let slug_copy = slug.clone();
	let slug_copy2 = slug.clone();
	let comment_data = create_resource(
		|| (),
		move |_| {
			let slug_clone = slug.clone();
			async move {
				let client = reqwest::Client::new();
				let mut builder = client
					.get(format!("{}{}{}{}",API_ENDPOINT,"/articles/", &slug_clone ,"/comments"))
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
				let data = response.json::<CommentListInfo>().await.ok()?;
				Some(data.comments)
			}
		}
	);
	let input_element: NodeRef<html::Textarea> = create_node_ref();
	let save_comment_action = create_action(move |input:&(String,String)| {
		let input_clone = input.0.clone();
		let slug_clone = input.1.clone();;
		async move {
			let client  = reqwest::Client::new();
			let mut builder = client
				.post(format!("{}{}{}{}",API_ENDPOINT, "/articles/", slug_clone, "/comments"))
				.header("Content-Type", "application/json");
			if let Ok(token) = LocalStorage::get::<String>(SESSION_TOKEN) {
				builder = builder.bearer_auth(token);
			}
			let comment_create_info = CommentCreateInfoWrapper {
				comment: CommentCreateInfo {
					body: input_clone,
				},
			};
			let response = builder.json(&comment_create_info)
				.send()
				.await
				.ok()?;
			if !response.status().is_success() {
				return None;
			}

			let data = response.json::<CommentInfoWrapper>().await.ok()?;
			comment_data.update(|list|
				if let Some(Some(list_some)) = list {list_some.push(data.comment)});
			Some(())
		}
	});

	let on_submit = move |ev: leptos::ev::SubmitEvent| {
		// stop the page from reloading!
		ev.prevent_default();

		let value = input_element()
			.expect("<input> should be mounted")
			.value();
		save_comment_action.dispatch((value.to_string(),slug_copy2.clone()))
	};
	view! {
		<div class="row">
		  <div class="col-xs-12 col-md-8 offset-md-2">
			<form class="card comment-form" on:submit=on_submit>
			  <div class="card-block">
				<textarea class="form-control" placeholder="Write a comment..." node_ref=input_element rows="3"></textarea>
			  </div>
			  <div class="card-footer">
				<img src=user_info().image class="comment-author-img" />
				<button class="btn btn-sm btn-primary" type="submit">Post Comment</button>
			  </div>
			</form>
			{move || match comment_data() {
				Some(Some(comment_list)) => view! {
					<For
					  each=move || comment_list.clone()
					  key=|comment| comment.id.clone()
					  let:child
					>
						<ArticleCommentListItem comment=child />
					</For>
				}.into_view(),
				Some(_) => view! { <p>"Failed to load."</p> }.into_view(),
				None => view! { <p>"Loading..."</p> }.into_view(),
				}
			}
		  </div>
		</div>
	}
}
