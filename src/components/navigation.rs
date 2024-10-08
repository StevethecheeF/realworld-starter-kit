use leptos::*;
use crate::types::data_beans::UserInfo;

#[component]
pub fn Navigation() -> impl IntoView {
    let user_info = expect_context::<RwSignal<UserInfo>>();
    let (user_info_username, _) = create_slice(
        user_info,
        |user_info| user_info.username.clone(),
        |user_info, username| user_info.username = username,
    );
    let user_info_is_authenticated = create_read_slice(
        user_info,
        |user_info| user_info.is_authenticated(),
    );

    let profile_url = move || {
        match user_info_username() {
            Some(v) => "/profile/".to_owned()+&v,
            None => "/profile/".to_string(),
        }
    };


    let logged_out_view = move || {
        view! {
            <ul class="nav navbar-nav pull-xs-right">
                <li class="nav-item">
                    <a class="nav-link active" href="/">Home</a>
                </li>
                <li class="nav-item">
                    <a class="nav-link" href="/login">Sign in</a>
                </li>
                <li class="nav-item">
                    <a class="nav-link" href="/register">Sign up</a>
                </li>
            </ul>
        }
    };
    let logged_in_view = move ||{
            view! {
            <ul class="nav navbar-nav pull-xs-right">
                <li class="nav-item">
                    <a class="nav-link active" href="/">Home</a>
                </li>
                <li class="nav-item">
                    <a class="nav-link" href="/editor"> <i class="ion-compose"></i>New Article </a>
                </li>
                <li class="nav-item">
                    <a class="nav-link" href="/settings"> <i class="ion-gear-a"></i>Settings </a>
                </li>
                <li class="nav-item">
                    <a class="nav-link" href=profile_url>
                <img src="" class="user-pic" />
                    { user_info_username }
                </a>
                </li>
            </ul>
        }
    };

    view! {
        <nav class="navbar navbar-light">
            <div class="container">
                <a class="navbar-brand" href="/">conduit</a>
                <Show
                    when=move || user_info_is_authenticated()
                    fallback=move ||  view! {{logged_out_view()}}
                >
                    {logged_in_view()}

                </Show>
            </div>
        </nav>
    }


}