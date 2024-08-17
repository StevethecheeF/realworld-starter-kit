mod login;

use leptos::*;

fn main() {
    mount_to_body(|| view! { <login::Login/> })
}