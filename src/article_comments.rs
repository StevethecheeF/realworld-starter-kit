use leptos::*;
use super::types::*;
use leptos_router::*;
use gloo::storage::{LocalStorage, Storage};

#[derive(Params, PartialEq)]
struct ContactParams {
	slug: Option<String>,
}

#[component]
pub fn ArticleComments() -> impl IntoView {
	let params = use_params::<ContactParams>();
	let slug = move || {
		params.with(|params| {
			params.as_ref()
				.map(|params| params.slug.clone())
				.unwrap_or_default()
		})
	};

	view! {
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
	}
}
