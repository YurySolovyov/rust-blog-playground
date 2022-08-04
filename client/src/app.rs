use yew::prelude::*;
use yew_router::prelude::*;
use yew::{function_component, html, Properties, Children};

use serde::Deserialize;
use reqwasm::http::Request;

#[derive(Clone, PartialEq, Deserialize, Properties)]
pub struct Post {
  pub id: String,
  pub title: String,
  pub body: Vec<String>,
}


#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    #[prop_or_default]
    pub children: Children,
}

#[derive(Properties, PartialEq)]
pub struct PostProps {
  pub post: Post,
}

#[function_component(ShortPost)]
pub fn shot_post(props: &PostProps) -> Html {
  let post = props.post.clone();

  html! {
    <div key={post.id.clone()} class={classes!("post", "short")}>
      <h3>
        <Link<Route> to={Route::Show { id: post.id }}>{ &post.title }</Link<Route>>
      </h3>
    </div>
  }
}

#[function_component(FullPost)]
pub fn full_post(props: &PostProps) -> Html {
  let post = props.post.clone();

  html! {
    <div key={post.id} class={classes!("post", "full")}>
      <h3>{post.title}</h3>
      <div class={classes!("post-body")}>
        {
          post.body.iter().map(|paragraph| {
            html! { <p>{paragraph}</p> }
          }).collect::<Html>()
        }
      </div>
    </div>
  }
}

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
  html! {
    <main>
      <nav>
        <Link<Route> to={Route::Index}>{ "Home" }</Link<Route>>
        <Link<Route> to={Route::Create}>{ "Write" }</Link<Route>>
      </nav>

      <div class={classes!("layout")}>
        {for props.children.iter()}
      </div>
    </main>
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
    <Layout>
      <h2>{ "Latest Posts" }</h2>
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
    </Layout>
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
    <Layout>


      {
        if let Some(post) = &*post {
          html! {
            <FullPost post={post.clone()} />
          }
        } else {
          html! { "Loading" }
        }
      }
    </Layout>
  }
}

#[function_component(Create)]
pub fn create() -> Html {
  html! {
    <Layout>
      <h2>{ "New post" }</h2>
      <form>
        <div class={classes!("input")}>
          <span class={classes!("label")}>
            { "Title" }
          </span>

          <input type="text" />
        </div>

        <div class={classes!("input")}>
          <span class={classes!("label")}>
            { "Body" }
          </span>

          <textarea rows={10}></textarea>
        </div>

        <div class={classes!("input")}>
          <button>{ "Submit" }</button>
        </div>
      </form>
    </Layout>
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

  #[at("/post/new")]
  Create,

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
    Route::Create => html! { <Create /> },

    _ => html! { <NotFound /> },
  }
}

#[function_component(App)]
pub fn app() -> Html {
  html! {
    <BrowserRouter>
      <Switch<Route> render={Switch::render(switch)} />
    </BrowserRouter>
  }
}
