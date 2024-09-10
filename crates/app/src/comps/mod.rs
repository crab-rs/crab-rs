use rust_utils::data::IData;
use dioxus::prelude::*;
use crate::api::PostAPI;
use crate::{MyState, Route};
use crate::models::*;
#[component]
pub fn PageNotFound(segments: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {segments:?}" }
    }
}

pub fn NoPosts() -> Element {
    rsx! {
       h2{
            "No posts found."
        }
    }
}

pub fn Loading() -> Element {
    rsx! {
       h2{
            "Loading..."
        }
    }
}

#[component]
pub fn ErrMsg(msg: String) -> Element {
    rsx! {
       h2{
            "Err... {msg}"
        }
    }
}

pub fn Home() -> Element {
    let posts = use_resource(|| async move {
        PostAPI::list_raw(10).await.unwrap_or(vec![])
    });

    match &*posts.read_unchecked() {
        None => {
            rsx! {
               Loading{}
            }
        }
        Some(posts) => {
            rsx! {
                h1{
                    "Articles22"
                }
                if posts.is_empty(){
                    NoPosts{}
                }else{
                    for post in posts {
                        p{
                            Link{
                                to: Route::Blog {id: post.id},
                                "{post.to::<Post>().unwrap().title}"
                            }
                        }

                    }
                }

            }
        }
    }
}

#[component]
pub fn Blog(id: i64) -> Element {
    let posts = use_resource(move || async move {
        PostAPI::get(id).await
    });
    match &*posts.read_unchecked() {
        None => {rsx!{Loading{}}}
        Some(r) => {
            match r {
                Ok(post) => {
                    rsx! {
                        p{
                            Link{to:Route::Home{}, "Home"}
                        }
                        h2{
                            "{post.title}"
                        }

                        pre{
                            "{post.content}"
                        }
                    }
                }
                Err(e) => {
                    rsx!{
                        ErrMsg{msg: format!("{:?}",e)}
                    }
                }
            }
        }
    }

}

pub fn App() -> Element {
    //MyState usage: let  result_str = use_context::<MyState>().result_str;
    use_context_provider(|| MyState::default());

    //see style: https://jenil.github.io/chota/#buttons
    rsx! {
                //define css here to compatible with ios
         style { {include_str!("../../public/chota.min.css")} }
         style { {include_str!("../../public/main.css")} }

         Router::<Route> {}

    }
}