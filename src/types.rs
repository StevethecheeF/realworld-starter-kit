use serde::{Deserialize, Serialize};
use chrono::prelude::*;

pub const SESSION_TOKEN:&str = "session.token";

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct RegisterInfo {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RegisterInfoWrapper {
    pub user: RegisterInfo,
}

#[derive(Serialize,Deserialize, Clone, Debug, Default)]
pub struct LoginInfo {
    pub email: String,
    pub password: String,
}

#[derive(Serialize,Deserialize, Clone, Debug)]
pub struct LoginInfoWrapper {
    pub user: LoginInfo,
}

#[derive(Serialize,Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct UserInfo {
    pub email: Option<String>,
    pub token: Option<String>,
    pub username: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
}

impl UserInfo {
    pub fn is_authenticated(&self) -> bool { self.token.is_some() }
}

#[derive(Serialize,Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct UserInfoWrapper {
    pub user: UserInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ProfileInfo {
    pub username: String,
    pub bio: Option<String>,
    pub image: String,
    pub following: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ProfileInfoWrapper {
    pub profile: ProfileInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ArticleCreateUpdateInfo {
    pub title: String,
    pub description: String,
    pub body: String,
    pub tagList: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ArticleCreateUpdateInfoWrapper {
    pub article: ArticleCreateUpdateInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ArticleInfo {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tagList: Vec<String>,
    pub createdAt: DateTime<Utc>,
    pub updatedAt: DateTime<Utc>,
    pub favorited: bool,
    pub favoritesCount: u32,
    pub author: ProfileInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ArticleInfoWrapper {
    pub article: ArticleInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct UserUpdateInfo {
    pub email: String,
    pub username: String,
    pub password: String,
    pub image: String,
    pub bio: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserUpdateInfoWrapper {
    pub user: UserUpdateInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TagListInfo {
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ArticleListInfo {
    pub articles: Vec<ArticleInfo>,
    pub articlesCount: u32,
}
