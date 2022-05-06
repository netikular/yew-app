use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main class="prose">
            <img class="h-16 w-16" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <h1 class="text-3xl">{ "Hello World!" }</h1>
            <span class="text-sm">{ "from Yew with " }<i class="heart" /></span>
        </main>
    }
}
