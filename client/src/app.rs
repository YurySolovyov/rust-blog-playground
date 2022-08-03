use yew::prelude::*;
use yew_router::prelude::*;
use yew::{function_component, html, Properties};

use serde::Deserialize;
use reqwasm::http::Request;

#[derive(Clone, PartialEq, Deserialize, Properties)]
pub struct Post {
  pub id: String,
  pub title: String,
  pub body: String,
}

#[derive(Properties, PartialEq)]
pub struct PostProps {
  pub post: Post,
}

#[function_component(ShortPost)]
pub fn shot_post(props: &PostProps) -> Html {
  let post = props.post.clone();

  html! {
    <div key={post.id.clone()} class={classes!("post")}>
      <h2>
        <Link<Route> to={Route::Show { id: post.id }}>{ &post.title }</Link<Route>>
      </h2>
    </div>
  }
}

#[function_component(FullPost)]
pub fn full_post(props: &PostProps) -> Html {
  let post = props.post.clone();

  html! {
    <div key={post.id} class={classes!("post")}>
      <h2>{post.title}</h2>
      <p>{post.body}</p>
    </div>
  }
}

#[function_component(Index)]
pub fn index() -> Html {
  let posts = use_state(|| Vec::default());

  {
    let posts = posts.clone();
    use_effect_with_deps(move |_| {
      wasm_bindgen_futures::spawn_local(async move {
        let response: Vec<Post> = Request::get("/api/posts")
          .send()
          .await
          .unwrap()
          .json()
          .await
          .unwrap();

        posts.set(response);
      });
      || ()
    }, ());
  }

  html! {
    <div class={classes!("posts-layout")}>
      <h1>{ "Latest Posts" }</h1>
      {
        if posts.is_empty() {
          html! { "Loading" }
        } else {
          html! {
            posts.iter().map(|post| {
              html! { <ShortPost post={post.clone()} /> }
            }).collect::<Html>()
          }
        }
      }
    </div>
  }
}

#[derive(Clone, Properties, PartialEq)]
pub struct ShowProps {
  pub id: String,
}

#[function_component(Show)]
pub fn show(props: &ShowProps) -> Html {
  let post = use_state(|| None);
  let props = props.clone();

  {
    let post = post.clone();
    use_effect_with_deps(move |_| {
      wasm_bindgen_futures::spawn_local(async move {
        let response: Post = Request::get(&format!("/api/posts/{}", props.id))
          .send()
          .await
          .unwrap()
          .json()
          .await
          .unwrap();

        post.set(Some(response));
      });
      || ()
    }, ());
  }

  html! {
    <div class={classes!("posts-layout")}>
      <h2>
        <Link<Route> to={Route::Index}>{ "Home" }</Link<Route>>
      </h2>

      {
        if let Some(post) = &*post {
          html! {
            <FullPost post={post.clone()} />
          }
        } else {
          html! { "Loading" }
        }
      }
    </div>
  }
}

#[function_component(NotFound)]
pub fn not_found() -> Html {
  html! { <h1>{ "404" }</h1> }
}


#[derive(Clone, Routable, PartialEq)]
enum Route {
  #[at("/")]
  Index,

  #[at("/post/:id")]
  Show { id: String },

  #[not_found]
  #[at("/404")]
  NotFound
}

fn switch(routes: &Route) -> Html {
  match routes {
    Route::Index => html! { <Index /> },
    Route::Show { id } => html! { <Show id={id.clone()} /> },

    _ => html! { <NotFound /> },
  }
}

#[function_component(App)]
pub fn app() -> Html {
  html! {
    <BrowserRouter>
      <main>
        <Switch<Route> render={Switch::render(switch)} />
      </main>
    </BrowserRouter>
  }
}
