//use salvo::http::{header, Mime};
use salvo::{Depot, Request, Response, Writer, async_trait};
use serde::Serialize;
use serde_json::json;
use tera::{Context, Tera};
// The main Inertia page object that gets sent to the client.
#[derive(Serialize)]
pub struct Page<T: Serialize> {
    component: String,
    props: T,
    url: String,
    version: String,
}

// Our custom responder struct.
pub struct Inertia<T: Serialize> {
    component: String,
    props: T,
}

impl<T: Serialize> Inertia<T> {
    pub fn new(component: impl Into<String>, props: T) -> Self {
        Self {
            component: component.into(),
            props,
        }
    }
}
// This is where the magic happens. We teach Salvo how to render our `Inertia` struct.
#[async_trait]
impl<T: Serialize> Writer for Inertia<T> {
    async fn write(mut self, req: &mut Request, depot: &mut Depot, res: &mut Response) {
        // 1. Get shared state: the Tera templating engine.
        let tera = depot.obtain::<Tera>().unwrap();

        // 2. Define a static asset version. In a real app, this might come from a file hash or env var.
        let version = "1.0.0".to_string();

        // 3. Construct the full Page object.
        let page = Page {
            component: self.component,
            props: self.props,
            url: req
                .uri()
                .path_and_query()
                .map(|pq| pq.as_str())
                .unwrap_or("/")
                .to_string(),
            version,
        };

        // 4. Check for the `X-Inertia` header to determine the response type.
        if req.headers().get("X-Inertia").is_some() {
            // It's an Inertia visit: respond with JSON.
            res.headers_mut()
                .insert(header::CONTENT_TYPE, Mime::JSON.into());
            res.headers_mut()
                .insert("X-Inertia", "true".parse().unwrap());
            res.render(salvo::prelude::Json(page));
        } else {
            // It's a first-time visit: respond with the full HTML shell.
            let mut context = Context::new();
            // Serialize the page data to a JSON string to embed in the HTML.
            let page_json = json!(page).to_string();
            context.insert("page", &page_json);

            match tera.render("app.html.tera", &context) {
                Ok(html) => res.render(salvo::prelude::Html(html)),
                Err(e) => {
                    res.status_code(salvo::http::StatusCode::INTERNAL_SERVER_ERROR);
                    res.render(format!("Template rendering error: {}", e));
                }
            }
        }
    }
}
