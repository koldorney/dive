#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;
use serde::{Deserialize, Serialize};

#[derive(Clone, Props, PartialEq, Serialize, Deserialize)]
pub struct Post {
    pub id: i64,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub content: String
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

pub fn App() -> Element{
    rsx!{ Posty { id: 1, title: "duhhh", content: "and Im back to tearin up" } }
}

fn Posty(post: Post) -> Element{
    rsx!{
        h1 {"{post.title}"}
        p {"{post.content}"}
    }
}


