use gloo_net::http::Request;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Clone, Deserialize, Debug)]
pub struct Post {
    writer: String,
    content: String,
}

#[function_component(Posts)]
pub fn posts() -> Html {
    let data = use_state(|| None);

    // Request `/api/v1/posts` once
    {
        let data = data.clone();
        use_effect(move || {
            if data.is_none() {
                spawn_local(async move {
                    let resp = Request::get("/api/v1/posts").send().await.unwrap();
                    let result = {
                        if !resp.ok() {
                            Err(format!(
                                "Error fetching data {} ({})",
                                resp.status(),
                                resp.status_text()
                            ))
                        } else {
                            let response =
                                resp.text().await.map_err(|err| err.to_string()).unwrap();
                            Ok(serde_json::from_str::<Vec<Post>>(&response).unwrap())
                        }
                    };
                    data.set(Some(result));
                });
            }

            || {}
        });
    }

    match data.as_ref() {
        None => {
            html! {
                <div>{"No server response"}</div>
            }
        }
        Some(Ok(data)) => {
            html! {
                <ul class="posts">
                    <p>{"Posts"}</p>
                    { data.iter().map(|Post {writer, content}| {
                        html!{
                            <div class="post">
                                <div>{format!("Writer: {writer}")}</div>
                                <div>{format!("Content: {content}")}</div>
                            </div>
                        }
                    }).collect::<Html>() }
                </ul>
            }
        }
        Some(Err(err)) => {
            html! {
                <div>{"Error requesting data from server: "}{err}</div>
            }
        }
    }
}
