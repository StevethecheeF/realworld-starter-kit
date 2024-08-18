use leptos::*;
use serde::{Deserialize, Serialize};
use super::types::*;
use gloo::storage::{LocalStorage, Storage};

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
pub fn Register() -> impl IntoView {
    let username_input_element: NodeRef<html::Input> = create_node_ref();
    let email_input_element: NodeRef<html::Input> = create_node_ref();
    let password_input_element: NodeRef<html::Input> = create_node_ref();

    let (failed_response, set_failed_response) = create_signal(false);

    let action = create_action(move |input: &(String, String, String)|{
        let input_copy = input.to_owned();
        async move {
            let result = handle_request(input_copy.0.clone(), input_copy.1.clone(), input_copy.2.clone()).await;
            match result {
                Ok(v) => {
                    logging::log!("{:?}",v);
                    LocalStorage::set(SESSION_TOKEN,v.user.token);
                    set_failed_response(false);
                }
                Err(v) => {
                    logging::log!("{:?}",v);
                    set_failed_response(true);
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
        let email_value = email_input_element()
            .expect("<input> should be mounted")
            .value();
        let password_value = password_input_element()
            .expect("<input> should be mounted")
            .value();

        action.dispatch((username_value.to_string(), email_value.to_string(), password_value.to_string()));
    };

    view! {
        <div class="auth-page">
            <div class="container page">
                <div class="row">
                    <div class="col-md-6 offset-md-3 col-xs-12">
                        <h1 class="text-xs-center">Sign up</h1>
                        <p class="text-xs-center">
                            <a href="/login">Have an account?</a>
                        </p>
                        <Show
                            when=move || { failed_response.get() }
                        >
                            <ul class="error-messages">
                                <li>That email is already taken</li>
                            </ul>
                        </Show>
                        <form on:submit=on_submit>
                            <fieldset class="form-group">
                                <input class="form-control form-control-lg"
                                    type="text"
                                    placeholder="Username"
                                    node_ref=username_input_element
                                />
                            </fieldset>
                            <fieldset class="form-group">
                                <input class="form-control form-control-lg"
                                    type="text"
                                    placeholder="Email"
                                    node_ref=email_input_element
                                />
                            </fieldset>
                            <fieldset class="form-group">
                                <input class="form-control form-control-lg"
                                    type="password"
                                    placeholder="Password"
                                    node_ref=password_input_element
                                />
                            </fieldset>
                            <button class="btn btn-lg btn-primary pull-xs-right" type="submit">Sign up</button>
                        </form>
                    </div>
                </div>
            </div>
        </div>
    }
}
