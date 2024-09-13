use gloo::storage::{LocalStorage, Storage};
use crate::types::data_beans::{ArticleInfo, ArticleInfoWrapper, ProfileInfo, ProfileInfoWrapper};
use crate::types::SESSION_TOKEN;

pub async fn favorite_article_action(is_favorite:bool, article_slug: &str) -> Option<ArticleInfo> {
	let client = reqwest::Client::new();
	let mut url = "http://localhost:3000/api/articles/".to_owned();
	url.push_str(article_slug);
	url.push_str("/favorite");

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
	if is_following {
		builder = client.delete("http://localhost:3000/api/profiles/".to_owned() + username + "/follow");
	}else {
		builder = client.post("http://localhost:3000/api/profiles/".to_owned() + username + "/follow");
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