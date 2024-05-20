#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;
mod icons;
use icons::Virto;

use crate::icons::Icon;


#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {

    rsx! {
        Link {
            to: "https://virto.network",
            "Learn more about Kreivo"
        }

        Icon {
            icon: Virto,
            height: 70,
            width: 70,
            stroke_width: 2,
            class: "logo"
        }

        div {
            h1 { "Crea tu cuenta en Virto" }
            h3 { "Con esta cuenta podrás crear Comunidades y participar en ellas" }
        }

        form {
            onsubmit: move |event| {
                println!("Submitted! {event:?}")
            },
            label { class: "label", "Nombre de usuario"},
            input { name: "usuario", placeholder: "Ej: ail3n_xyz", },
            label { class: "label", "Contraseña"},
            input { name: "contraseña", placeholder: "Contraseña", r#type: "password", },
            button { class: "button button--primary", "Crear cuenta", },
            button { class: "button button--secondary", "Ya tengo una cuenta", },
        }
    }
}
