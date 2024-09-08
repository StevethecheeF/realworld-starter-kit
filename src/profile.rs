use leptos::*;
use super::types::*;
use leptos_router::*;
use gloo::storage::{LocalStorage, Storage};
use super::helper::follow_user;
use super::home_article_list_item;

#[derive(Params, PartialEq)]
struct ContactParams {
    user_id: Option<String>,
}

async fn handle_request(username_value:String, email_value:String, password_value:String) -> Result<UserInfoWrapper, String>{
    let user_data = RegisterInfo {
        username: username_value,
        email: email_value,
        password: password_value,
    };
    let login_data = RegisterInfoWrapper{
        user: user_data,
    };
    let client = reqwest::Client::new();
    let response = client
        .post("http://localhost:3000/api/users")
        .header("Content-Type", "application/json")
        .json(&login_data)
        .send()
        .await;
    if let Ok(data) = response {
        if data.status().is_success() {
            let data: Result<UserInfoWrapper, _> = data.json::<UserInfoWrapper>().await;
            if let Ok(data) = data {
                Ok(data)
            } else {
                Err("Error occurred".to_string())
            }
        } else {
            Err("Error occurred".to_string())
        }
    } else {
        Err("Error occurred".to_string())
    }
}

#[component]
pub fn Profile() -> impl IntoView {
    let params = use_params::<ContactParams>();
    let user_id = move || {
        params.with(|params| {
            params.as_ref()
                .map(|params| params.user_id.clone())
                .unwrap_or_default()
        })
    };
    let (is_following, set_is_following) = create_signal(false);

    let async_data = create_resource(
        user_id,
        move |_| async move {
            let client = reqwest::Client::new();
            let response = client
                .get("http://localhost:3000/api/profiles/".to_owned() + &user_id().unwrap_or_default())
                .header("Content-Type", "application/json")
                .send()
                .await
                .ok()?;

            if !response.status().is_success() {
                return None;
            }

            let data = response.json::<ProfileInfoWrapper>().await.ok()?;
            set_is_following.set(data.clone().profile.following);
            Some(data.profile)
        },
    );

    let user_info = expect_context::<RwSignal<UserInfo>>();

    // follow
    let follow_text = move |is_following| {
        if is_following {
            "Unfollow ".to_owned() + &user_id().unwrap_or_default()
        }else {
            "Follow ".to_owned() + &user_id().unwrap_or_default()
        }
    };

    let follow_action = create_action(move |_|{
        async move {
            match async_data.get() {
                Some(Some(profile_info)) =>{
                    let profile_info_option = follow_user(profile_info.following,&*user_id().unwrap_or_default()).await;
                    if let Some(profile_info) = profile_info_option {
                        async_data.set(Option::from(profile_info.clone()));
                        set_is_following(profile_info.following);
                    }

                }
                _ => logging::log!("no profile data")
            };
        }
    });

    let on_follow_click = move |_| {
        follow_action.dispatch(());
    };

    let (current_tab, set_current_tab) = create_signal("User".to_string());
    let (article_data, set_article_data) = create_signal(vec![]);
    let (article_count, set_article_count) = create_signal(0);
    let (current_page, set_current_page) = create_signal(1);

    let global_article_request = create_action(move |_| {
        let client = reqwest::Client::new();
        async move {
            let mut builder = client
                .get("http://localhost:3000/api/articles".to_owned())
                .header("Content-Type", "application/json");
            if let Ok(token) = LocalStorage::get::<String>(SESSION_TOKEN) {
                builder = builder.bearer_auth(token);
            }
            let offset = (current_page.get() - 1) * 20;
            let mut query = vec![("offset",offset.to_string())];
            match current_tab.get().as_str() {
                "User" => query.push(("author",user_id().unwrap_or_default())),
                "Favorited" => query.push(("favorite",user_id().unwrap_or_default())),
                _ => return None
            }
            builder = builder.query(&query);

            let response = builder
                .send()
                .await
                .ok()?;
            if !response.status().is_success() {
                return None;
            }
            let data = response.json::<ArticleListInfo>().await.ok()?;
            set_article_data(data.articles);
            set_article_count(data.articlesCount);
            Some(())
        }
    });

    let async_article_list = create_resource(
        current_tab,
        move |current_tab| async move {
            set_current_page(1);
            global_article_request.dispatch(());
        },
    );

    let available_pages = move || {
        let page_count = (article_count.get() / 20) + 2;
        (1..page_count).collect::<Vec<_>>()

    };
    let on_pagination_click = move |page: u32| {
        if current_page.get() != page {
            set_current_page(page);
            global_article_request.dispatch(());
        }
    };
    view! {
    <div class="profile-page">
      <div class="user-info">
        <div class="container">
          <div class="row">
            <div class="col-xs-12 col-md-10 offset-md-1">
              {
                move || match async_data.get() {
                  Some(Some(profile)) => view! {
                    <img src=profile.image class="user-img" />
                    <h4> { profile.username } </h4>
                    <p> { profile.bio } </p>
                    <button class="btn btn-sm btn-outline-secondary action-btn" on:click=on_follow_click>
                      <i class="ion-plus-round"></i>
                      { follow_text(profile.following) }
                    </button>
                    <button class="btn btn-sm btn-outline-secondary action-btn">
                      <i class="ion-gear-a"></i>
                      Edit Profile Settings
                    </button>
                  }.into_view(),
                  Some(_) => view! { <p>"Failed to load profile."</p> }.into_view(),
                  None => view! { <p>"Loading profile..."</p> }.into_view(),
                }
              }
            </div>
          </div>
        </div>
      </div>

      <div class="container">
        <div class="row">
          <div class="col-xs-12 col-md-10 offset-md-1">
            <div class="articles-toggle">
              <ul class="nav nav-pills outline-active">
                <li class="nav-item">
                  <a class="nav-link active"
                    href=""
                    on:click=move |_| set_current_tab("User".to_string())
                    >My Articles</a>
                </li>
                <li class="nav-item">
                  <a class="nav-link"
                    href=""
                    on:click=move |_| set_current_tab("Favorited".to_string())
                    >Favorited Articles</a>
                </li>
              </ul>
            </div>
            <For
              each=article_data
              key=|article| article.slug.clone()
              let:child
            >
              <home_article_list_item::HomeArticleListItem article=child />
            </For>
            <ul class="pagination">
                <For
                    each=available_pages
                    key=|n| n.clone()
                    let:child
                >
                    <li class="page-item">
                      <a class="page-link" href="" on:click=move |_| on_pagination_click(child.clone())>{child}</a>
                    </li>
                </For>
            </ul>
          </div>
        </div>
      </div>
    </div>
    }
}
