use leptos::*;
use super::types::*;
use leptos_router::*;
use gloo::storage::{LocalStorage, Storage};
use super::helper::follow_user;

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
                  <a class="nav-link active" href="">My Articles</a>
                </li>
                <li class="nav-item">
                  <a class="nav-link" href="">Favorited Articles</a>
                </li>
              </ul>
            </div>
          </div>
        </div>
      </div>
    </div>
    }
}
