use leptos::*;
use super::types::*;
use leptos_router::*;
use gloo::storage::{LocalStorage, Storage};

#[derive(Params, PartialEq)]
struct ContactParams {
    slug: Option<String>,
}

#[component]
pub fn Editor() -> impl IntoView {
    let params = use_params::<ContactParams>();
    let slug = move || {
        params.with(|params| {
            params.as_ref()
                .map(|params| params.slug.clone())
                .unwrap_or_default()
        })
    };

    let (title, set_title) = create_signal("".to_string());
    let (description, set_description) = create_signal("".to_string());
    let (body, set_body) = create_signal("".to_string());
    let (tags,set_tags) = create_signal("".to_string());

    let async_data = create_resource(
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
                .await;
            if let Ok(data) = response {
                if data.status().is_success() {
                    let data: Result<ProfileInfoWrapper, _> = data.json::<ProfileInfoWrapper>().await;
                    if let Ok(data) = data {
                        Some(data)
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        },
    );

    let save_action = create_action(move |_|{
        async move {
            if title.get() == ""{
                return;
            }
            let tag_list = tags.get().split(" ").map(|s| s.to_string()).collect();
            let client = reqwest::Client::new();
            let info = ArticleCreateUpdateInfo {
                description: description.get(),
                title: title.get(),
                body: body.get(),
                tag_list: Some(tag_list),
            };
            let info_wrapper = ArticleCreateUpdateInfoWrapper {
                article: info,
            };
            if slug().is_none() {
                let mut builder =  client
                    .post("http://localhost:3000/api/articles/".to_owned())
                    .header("Content-Type", "application/json");
                if let Ok(token) = LocalStorage::get::<String>(SESSION_TOKEN) {
                    builder = builder.bearer_auth(token);
                }
                builder.json(&info_wrapper)
                    .send()
                    .await;
            }else {
                let mut builder = client
                    .put("http://localhost:3000/api/articles/".to_owned() + &slug().unwrap_or_default())
                    .header("Content-Type", "application/json");
                if let Ok(token) = LocalStorage::get::<String>(SESSION_TOKEN) {
                    builder = builder.bearer_auth(token);
                }
                builder.json(&info_wrapper)
                    .send()
                    .await;
            }

        }
    });

    let on_submit = move |_| {
        logging::log!("{:?}",title.get());
        logging::log!("{:?}",description.get());
        logging::log!("{:?}",body.get());
        logging::log!("{:?}",tags.get());
        save_action.dispatch(())
    };

    view! {
    <div class="editor-page">
      <div class="container page">
        <div class="row">
          <div class="col-md-10 offset-md-1 col-xs-12">
          <Show
            when=move || { title() == "" }
          >
          <ul class="error-messages">
              <li>That title is required</li>
            </ul>
        </Show>
            <form>
                <fieldset class="form-group">
                  <input
                    type="text"
                    class="form-control form-control-lg"
                    placeholder="Article Title"
                    prop:value=title
                    on:input=move |ev| {
                        set_title(event_target_value(&ev));
                    }
                  />
                </fieldset>
                <fieldset class="form-group">
                  <input
                    type="text"
                    class="form-control"
                    placeholder="What's this article about?"
                    prop:value=description
                    on:input=move |ev| {
                        set_description(event_target_value(&ev));
                    }
                  />
                </fieldset>
                <fieldset class="form-group">
                  <textarea
                    class="form-control"
                    rows="8"
                    placeholder="Write your article (in markdown)"
                    prop:value=move || body.get()
                    on:input=move |ev| {
                        set_body(event_target_value(&ev));
                    }
                  >
                    { body.get_untracked() }
                  </textarea>
                </fieldset>
                <fieldset class="form-group">
                  <input
                    type="text"
                    class="form-control"
                    placeholder="Enter tags"
                    prop:value=tags
                    on:input=move |ev| {
                        set_tags(event_target_value(&ev));
                    }
                  />
                  <div class="tag-list">
                    <span class="tag-default tag-pill"> <i class="ion-close-round"></i> tag </span>
                  </div>
                </fieldset>
                <button class="btn btn-lg pull-xs-right btn-primary" type="button" on:click=on_submit>
                  Publish Article
                </button>
            </form>
          </div>
        </div>
      </div>
    </div>
    }

}
