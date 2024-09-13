use leptos::*;
use leptos_router::*;
use crate::types::*;
use crate::helper::{favorite_article_action,follow_user};
use gloo::storage::{LocalStorage, Storage};
use crate::article_comment_list;

#[derive(Params, PartialEq)]
struct ContactParams {
	slug: Option<String>,
}

#[component]
pub fn Article() -> impl IntoView {
	let params = use_params::<ContactParams>();
	let slug = move || {
		params.with(|params| {
			params.as_ref()
				.map(|params| params.slug.clone())
				.unwrap_or_default()
		})
	};

	let article_data = create_resource(
		|| (),
		move |_| async move {
			if slug().is_none() {
				return None;
			}
			let client = reqwest::Client::new();
			let response = client
				.get("http://localhost:3000/api/articles/".to_owned() + &slug().unwrap_or_default())
				.header("Content-Type", "application/json")
				.send()
				.await
				.ok()?;

			if !response.status().is_success() {
				return None;
			}
			let data = response.json::<ArticleInfoWrapper>().await.ok()?;
			Some(data.article)
		},
	);

	let user_info = expect_context::<RwSignal<UserInfo>>();
	let (user_info_username, _) = create_slice(
		user_info,
		|user_info| user_info.username.clone(),
		|user_info, username| user_info.username = username,
	);
	let can_edit = move || {
		match user_info_username() {
			Some(v) => {
				match article_data.get(){
					Some(Some(article)) =>v == article.author.username,
					_=>false
				}
			},
			None => false,
		}
	};
	let profile_link = move || {
		match article_data.get(){
			Some(Some(article)) =>"/profile/".to_string() +  &*article.author.username,
			_=>"".to_string()
		}
	};
	let creation_date = move || {
		match article_data.get(){
			Some(Some(article)) => article.created_at.format("%B %e, %Y").to_string(),
			_=>"".to_string()
		}
	};
	let follow_text = move || {
		match article_data.get(){
			Some(Some(article)) => {
				if article.author.following {
					"Unfollow ".to_string() + &*article.author.username
				} else {
					"Follow ".to_string() + &*article.author.username
				}
			},
			_=>"".to_string()
		}
	};
	let favorite_article_action = create_action(move |_| {
		async move {
			if let Some(Some(article_date_some)) = article_data.get() {
				let data = favorite_article_action(article_date_some.favorited, &*article_date_some.slug).await;
				article_data.set(data.clone());
				Some(())
			}else {
				None
			}
		}
	});

	let favorite_article = move |_| {
		favorite_article_action.dispatch(());
	};


	let delete_article_action = create_action(move |_| {
		async move {
			match article_data.get() {
				Some(Some(article)) => {
					let client = reqwest::Client::new();
					let mut builder = client
						.delete("http://localhost:3000/api/articles/".to_owned() + &*article.slug)
						.header("Content-Type", "application/json");

					if let Ok(token) = LocalStorage::get::<String>(SESSION_TOKEN) {
						builder = builder.bearer_auth(token);
					}

					let response=	builder.send()
						.await
						.ok()?;

					if !response.status().is_success() {
						return None;
					}
					let navigate = leptos_router::use_navigate();
					navigate("/", Default::default());
					Some(())
				},
				_ => None
			}
		}
	});

	let delete_article = move |_| {
		delete_article_action.dispatch(());
	};

	let follow_action = create_action(move |_|{
		async move {
			match article_data.get() {
				Some(Some(mut article)) =>{
					let profile_info_option = follow_user(article.author.following,&*article.author.username).await;
					if let Some(profile_info) = profile_info_option {
						article.author = profile_info;
						article_data.set(Option::from(article));
					}

				},
				_ => logging::log!("no profile data")
			};
		}
	});

	let on_follow_click = move |_| {
		follow_action.dispatch(());
	};
	view! {
    <div class="article-page">
        {move || match article_data.get() {
			Some(Some(article)) => view! {
			<div class="banner">
				<div class="container">
				  <h1>{article.title}</h1>

				  <div class="article-meta">
					<a href="/profile/eric-simons"><img src=article.author.image.clone() alt=article.author.username.clone() /></a>
					<div class="info">
					  <a href=profile_link class="author">{article.author.username.clone()}</a>
					  <span class="date">{creation_date}</span>
					</div>
					<Show
					  when=move || {can_edit()}
					  fallback=move || view! {
						<button class="btn btn-sm btn-outline-secondary">
						  <i class="ion-plus-round"></i>
						  {follow_text}
						  <span class="counter"></span>
						</button>
						<button class="btn btn-sm btn-outline-primary" on:click=favorite_article>
						  <i class="ion-heart"></i>
						  Favorite Post <span class="counter">{article.favorites_count}</span>
						</button>
					  }
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
					  {article.description}
					</p>
					<p>{article.body}</p>
					<ul class="tag-list">
						<For
						  each=move || article.tag_list.clone()
						  key=|tag| tag.clone()
						  children=move |tag| {
							view!{<li class="tag-default tag-pill tag-outline">{tag}</li>}
						 }
						/>
					</ul>
				  </div>
				</div>

				<hr />

				<div class="article-actions">
				  <div class="article-meta">
					<a href=profile_link><img src=article.author.image /></a>
					<div class="info">
					  <a href="" class="author">{article.author.username.clone()}</a>
					  <span class="date">{creation_date}</span>
					</div>
					<Show
						when=move || Some(article.author.username.clone()) == user_info_username.get()
						fallback=move || view! {
							<button class="btn btn-sm btn-outline-secondary" on:click=on_follow_click>
							  <i class="ion-plus-round"></i>
							  {follow_text}
							</button>
							<button class="btn btn-sm btn-outline-primary" on:click=favorite_article>
							  <i class="ion-heart"></i>
							  Favorite Article <span class="counter">{article.favorites_count}</span>
							</button>
						}
					>
						<button class="btn btn-sm btn-outline-secondary" href=profile_link >
						  <i class="ion-edit"></i> Edit Article
						</button>
						<button class="btn btn-sm btn-outline-danger" on:click=delete_article>
						  <i class="ion-trash-a"></i> Delete Article
						</button>
					</Show>
				  </div>
				</div>
				{if let Some(slug_value) = slug() {
					view! {<article_comment_list::ArticleCommentList slug=slug_value />}.into_view()
				} else { view! {}.into_view()}
				}
			  </div>
			}.into_view(),
			Some(_) => view! { <p>"Failed to load."</p> }.into_view(),
			None => view! { <p>"Loading..."</p> }.into_view(),
            }
        }
    </div>
    }
}