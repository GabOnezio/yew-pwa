use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <h1>{ "Olá Coder!" }</h1>
            <span class="subtitle">{ "De GabOnezio " }<i class="sirs" /></span>
        </main>
    }
}
