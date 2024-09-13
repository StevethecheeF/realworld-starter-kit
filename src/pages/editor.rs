use leptos::*;
use leptos_router::*;
use gloo::storage::{LocalStorage, Storage};
use crate::types::data_beans::{ArticleCreateUpdateInfo, ArticleCreateUpdateInfoWrapper, ArticleInfoWrapper};
use crate::types::{API_ENDPOINT, SESSION_TOKEN};

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

    let title_input_element: NodeRef<html::Input> = create_node_ref();
    let description_input_element: NodeRef<html::Input> = create_node_ref();
    let tag_input_element: NodeRef<html::Input> = create_node_ref();
    let body_input_element: NodeRef<html::Textarea> = create_node_ref();
    let (tags,set_tags) = create_signal(vec![]);

    let save_action = create_action(move |input:&(String,String,String) |{
        let input_copy = input.to_owned();

        async move {
            let client = reqwest::Client::new();
            let info = ArticleCreateUpdateInfo {
                title: input_copy.0,
                description: input_copy.1,
                body: input_copy.2,
                tag_list: tags.get(),
            };
            let info_wrapper = ArticleCreateUpdateInfoWrapper {
                article: info,
            };
            if slug().is_none() {
                let mut builder =  client
                    .post(format!("{}{}",API_ENDPOINT,"/articles/"))
                    .header("Content-Type", "application/json");
                if let Ok(token) = LocalStorage::get::<String>(SESSION_TOKEN) {
                    builder = builder.bearer_auth(token);
                }
                let _ = builder.json(&info_wrapper)
                    .send()
                    .await;
            }else {
                let mut builder = client
                    .put(format!("{}{}{}",API_ENDPOINT,"/articles/", slug().unwrap_or_default()))
                    .header("Content-Type", "application/json");
                if let Ok(token) = LocalStorage::get::<String>(SESSION_TOKEN) {
                    builder = builder.bearer_auth(token);
                }
                let _ = builder.json(&info_wrapper)
                    .send()
                    .await;
            }

        }
    });

    let async_data = create_resource(
        || (),
        move |_| async move {
            if slug().is_none() {
                let article_info = ArticleCreateUpdateInfo {
                    title:"".to_string(),
                    tag_list:vec![].into(),
                    description:"".to_string(),
                    body:"".to_string()
                };
                return Some(article_info);
            }
            let client = reqwest::Client::new();
            let response = client
                .get(format!("{}{}{}",API_ENDPOINT,"/articles/", &slug().unwrap_or_default()))
                .header("Content-Type", "application/json")
                .send()
                .await
                .ok()?;

            if !response.status().is_success() {
                return None;
            }
            let data = response.json::<ArticleInfoWrapper>().await.ok()?;
            set_tags(data.article.tag_list.clone());
            let article_info = ArticleCreateUpdateInfo {
                title:data.article.title,
                tag_list:data.article.tag_list,
                description:data.article.description,
                body:data.article.body,
            };
            Some(article_info)
        },
    );

    let (need_title, set_need_title) = create_signal(false);

    let on_submit = move |_| {
        let title_value = title_input_element()
            .expect("<input> should be mounted")
            .value();
        if title_value.to_string() == "".to_string() {
            set_need_title(true);
            return;
        }
        let body_value = body_input_element()
            .expect("<input> should be mounted")
            .value();
        let description_value = description_input_element()
            .expect("<input> should be mounted")
            .value();

        save_action.dispatch((title_value.to_string(),description_value.to_string(),body_value.to_string()))
    };

    let on_keypress = move |e: leptos::ev::KeyboardEvent| {
        // Prevent submit the form when press Enter
        if e.key() == "Enter" {
            e.prevent_default();
        }
    };
    let on_keyup = move |e: leptos::ev::KeyboardEvent| {
        // Add a new tag when press Enter
        if e.key() == "Enter" {
            e.prevent_default();
            // Add a new tag
            let tag_value = tag_input_element()
                .expect("<input> should be mounted")
                .value();

            set_tags.update(|n| n.push(tag_value.to_string()));
        }
    };

    view! {
    <div class="editor-page">
      <div class="container page">
        <div class="row">
          <div class="col-md-10 offset-md-1 col-xs-12">
          <Show
            when=move || { need_title.get() }
          >
            <ul class="error-messages">
              <li>That title is required</li>
            </ul>
          </Show>
          <form>
            {
                move || match async_data.get() {
                  Some(Some(article)) => view! {
                <fieldset class="form-group">
                  <input
                    type="text"
                    class="form-control form-control-lg"
                    placeholder="Article Title"
                    prop:value=article.title
                    node_ref=title_input_element
                  />
                </fieldset>
                <fieldset class="form-group">
                  <input
                    type="text"
                    class="form-control"
                    placeholder="What's this article about?"
                    prop:value=article.description
                    node_ref=description_input_element
                  />
                </fieldset>
                <fieldset class="form-group">
                  <textarea
                    class="form-control"
                    rows="8"
                    placeholder="Write your article (in markdown)"
                    prop:value=article.body.clone()
                    node_ref=body_input_element
                  >
                    { article.body }
                  </textarea>
                </fieldset>
                <fieldset class="form-group">
                  <input
                    type="text"
                    class="form-control"
                    placeholder="Enter tags"
                    on:keyup=on_keyup
                    on:keypress=on_keypress
                    node_ref=tag_input_element
                  />
                  <div class="tag-list">
                    {
                      tags().into_iter()
                        .map(|n| view! { <span class="tag-default tag-pill"> <i class="ion-close-round"></i> {n} </span> })
                        .collect::<Vec<_>>()
                    }
                  </div>
                </fieldset>
                }.into_view(),
                Some(_) => view! { <p>"Failed to load profile."</p> }.into_view(),
                None => view! { <p>"Loading profile..."</p> }.into_view(),
              }
            }
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
