use gloo::storage::{LocalStorage, Storage};
use crate::types::data_beans::{ArticleInfo, ArticleInfoWrapper, ProfileInfo, ProfileInfoWrapper};
use crate::types::{API_ENDPOINT, SESSION_TOKEN};

pub async fn favorite_article_action(is_favorite:bool, article_slug: &str) -> Option<ArticleInfo> {
	let client = reqwest::Client::new();
	let url = format!("{}{}{}{}",API_ENDPOINT,"/articles/", article_slug, "/favorite");

	let mut builder;
	if is_favorite {
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
	Some(data.article)
}

pub async fn follow_user(is_following:bool, username: &str) -> Option<ProfileInfo>{
	let client = reqwest::Client::new();
	let mut builder;
	let url = format!("{}{}{}{}",API_ENDPOINT,"/profiles/",username,"/follow");
	if is_following {
		builder = client.delete(url);
	}else {
		builder = client.post(url);
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

	let data = response.json::<ProfileInfoWrapper>().await.ok()?;
	Some(data.profile)
}