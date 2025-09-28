use std::fmt::Debug;

use salvo::oapi::extract::*;
use salvo::{Extractible, prelude::*};
use sea_orm::ActiveModelBehavior;
use valipower::Validate;

#[endpoint]
async fn hello(name: QueryParam<String, false>) -> String {
    format!("Hello, {}!", name.as_deref().unwrap_or("World"))
}
#[endpoint]
async fn bye(name: MultiPartFormDataValidated<UserProfile>) {

    // format!("Hello, {}!", name.as_deref().unwrap_or("World"))
}

#[derive(ToSchema)]
struct Waza {
    id: String,
}

#[derive(Debug)]
pub struct UploadedFile {
    pub path: std::path::PathBuf,
    pub file_name: Option<String>,
    pub content_type: Option<String>,
}

impl ToSchema for UploadedFile {
    fn to_schema(
        components: &mut salvo::oapi::Components,
    ) -> salvo::oapi::RefOr<salvo::oapi::schema::Schema> {
        salvo::oapi::RefOr::Type()
    }
}

#[derive(Debug, ToSchema)]
pub struct UserProfile {
    pub username: String,
    pub email: String,
    pub avatar: UploadedFile, // Para un solo archivo
}

#[derive(Debug, Validate)]
struct Name {
    #[validate(email, min = 34)]
    field: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let router = Router::new().push(Router::with_path("hello").get(hello));
    let doc = OpenApi::new("test api", "0.0.1").merge_router(&router);

    let router = router
        .unshift(doc.into_router("/api-doc/openapi.json"))
        .unshift(SwaggerUi::new("/api-doc/openapi.json").into_router("/swagger-ui"));

    let acceptor = TcpListener::new("0.0.0.0:5800").bind().await;

    println!("{:?}", router);
    Server::new(acceptor).serve(router).await;
}
