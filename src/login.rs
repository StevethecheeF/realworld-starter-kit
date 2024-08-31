use leptos::*;
use super::types::*;
use gloo::storage::{LocalStorage, Storage};

async fn handle_request(email_value:String, password_value:String) -> Result<UserInfoWrapper, String>{
    let user_data = LoginInfo {
        email: email_value,
        password: password_value,
    };
    let login_data = LoginInfoWrapper{
        user: user_data,
    };
    let client = reqwest::Client::new();
    let response = client
        .post("http://localhost:3000/api/users/login")
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
pub fn Login() -> impl IntoView {
    let email_input_element: NodeRef<html::Input> = create_node_ref();
    let password_input_element: NodeRef<html::Input> = create_node_ref();

    let (failed_response, set_failed_response) = create_signal(false);

    let user_info = expect_context::<RwSignal<UserInfo>>();

    let action = create_action(move |input: &(String, String)|{
        let input_copy = input.to_owned();
        async move {
            let result = handle_request(input_copy.0.clone(), input_copy.1.clone()).await;
            match result {
                Ok(v) => {
                    logging::log!("{:?}",v);
                    LocalStorage::set(SESSION_TOKEN, &v.user.token).expect("failed to set");
                    user_info.set(v.user);
                    set_failed_response(false);
                    let navigate = leptos_router::use_navigate();
                    navigate("/", Default::default());
                }
                Err(v) => {
                    set_failed_response(true);
                }
            }
        }

    });

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        let email_value = email_input_element()
            .expect("<input> should be mounted")
            .value();
        let password_value = password_input_element()
            .expect("<input> should be mounted")
            .value();

        action.dispatch((email_value.to_string(),password_value.to_string()));
    };

    view! {
        <div class="auth-page">
            <div class="container page">
                <div class="row">
                    <div class="col-md-6 offset-md-3 col-xs-12">
                        <h1 class="text-xs-center">Sign in</h1>
                        <p class="text-xs-center">
                            <a href="/register">Need an account?</a>
                        </p>
                        <Show
                            when=move || { failed_response.get() }
                        >
                            <ul class="error-messages">
                                <li>Email or password were wrong</li>
                            </ul>
                        </Show>
                        <form on:submit=on_submit>
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
                            <button class="btn btn-lg btn-primary pull-xs-right" type="submit">Sign in</button>
                        </form>
                    </div>
                </div>
            </div>
        </div>
    }
}
