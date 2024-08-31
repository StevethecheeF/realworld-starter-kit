use leptos::*;
use super::types::*;
use gloo::storage::{LocalStorage, Storage};

pub fn Setting() -> impl IntoView{
    let user_info = expect_context::<RwSignal<UserInfo>>();

    let bio_input_element: NodeRef<html::Textarea> = create_node_ref();
    let email_input_element: NodeRef<html::Input> = create_node_ref();
    let image_input_element: NodeRef<html::Input> = create_node_ref();
    let username_input_element: NodeRef<html::Input> = create_node_ref();
    let password_input_element: NodeRef<html::Input> = create_node_ref();

    let (need_name, set_need_name) = create_signal(false);
    let action = create_action(move |input: &(String, String, Option<String>, String, String)|{
      let input_copy = input.to_owned();
      let user_update_info = UserUpdateInfo{
        image:input_copy.4,
        username: input_copy.0,
        bio:input_copy.3,
        email: input_copy.1,
        password: input_copy.2,
      };
      let user_update_info_wrapper = UserUpdateInfoWrapper{
        user: user_update_info,
      };
      async move {
        let client = reqwest::Client::new();
        let mut builder =  client
            .put("http://localhost:3000/api/user".to_owned())
            .header("Content-Type", "application/json");
        if let Ok(token) = LocalStorage::get::<String>(SESSION_TOKEN) {
          builder = builder.bearer_auth(token);
        }
        let response = builder.json(&user_update_info_wrapper)
            .send()
            .await;
        if let Ok(data) = response {
          if data.status().is_success() {
            let user_info_response = data.json::<UserInfoWrapper>().await;
            if let Ok(user_info_data) = user_info_response {
              LocalStorage::set(SESSION_TOKEN, &user_info_data.user.token).expect("failed to set");
              user_info.set(user_info_data.user);
            }
          }
        }
      }
    });

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
      // stop the page from reloading!
      ev.prevent_default();

      let username_value = username_input_element()
          .expect("<input> should be mounted")
          .value();
      if username_value.to_string() == "".to_string() {
        set_need_name(true);
        return;
      }
      let password_value = password_input_element()
          .expect("<input> should be mounted")
          .value();
      let mut password_option_value = None;
      if password_value.to_string() != "".to_string(){
        password_option_value = Some(password_value.to_string());
      }
      let image_value = image_input_element()
          .expect("<input> should be mounted")
          .value();
      let email_value = email_input_element()
          .expect("<input> should be mounted")
          .value();
      let bio_value = bio_input_element()
          .expect("<input> should be mounted")
          .value();

      action.dispatch((username_value.to_string(), email_value.to_string(), password_option_value,bio_value.to_string(),image_value.to_string()));
    };
    view! {
    <div class="settings-page">
      <div class="container page">
        <div class="row">
          <div class="col-md-6 offset-md-3 col-xs-12">
            <h1 class="text-xs-center">Your Settings</h1>

            <Show
                when=move || { need_name.get() }
            >
              <ul class="error-messages">
                <li>That name is required</li>
              </ul>
            </Show>

            <form on:submit=on_submit>
              <fieldset>
                <fieldset class="form-group">
                  <input
                    class="form-control"
                    type="text"
                    placeholder="URL of profile picture"
                    node_ref=image_input_element
                    prop:value=user_info.get().image
                  />
                </fieldset>
                <fieldset class="form-group">
                  <input
                    class="form-control form-control-lg"
                    type="text"
                    placeholder="Your Name"
                    node_ref=username_input_element
                    prop:value=user_info.get().username
                  />
                </fieldset>
                <fieldset class="form-group">
                  <textarea
                    class="form-control form-control-lg"
                    rows="8"
                    placeholder="Short bio about you"
                    node_ref=bio_input_element
                    prop:value=user_info.get().bio.clone()
                  >
                    {user_info.get().bio}
                  </textarea>
                </fieldset>
                <fieldset class="form-group">
                  <input
                    class="form-control form-control-lg"
                    type="text"
                    placeholder="Email"
                    node_ref=email_input_element
                    prop:value=user_info.get().email
                  />
                </fieldset>
                <fieldset class="form-group">
                  <input
                    class="form-control form-control-lg"
                    type="password"
                    placeholder="New Password"
                    node_ref=password_input_element
                  />
                </fieldset>
                <button class="btn btn-lg btn-primary pull-xs-right" type="submit">Update Settings</button>
              </fieldset>
            </form>
            <hr />
            <button class="btn btn-outline-danger">Or click here to logout.</button>
          </div>
        </div>
      </div>
    </div>
    }
}