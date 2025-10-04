use std::fmt::Debug;

use salvo::oapi::{BasicType, Object, extract::*};

use salvo::{Extractible, prelude::*};
use sea_orm::ActiveModelBehavior;
use serde::Deserialize;
use thunder::{FromMultipart, Inertia, MultipartValidated, UploadedFile, Validate};

#[endpoint]
async fn hello(name: FormBody<String>) -> String {
    todo!()
}
#[endpoint]
async fn bye(name: MultipartValidated<UserProfile>) {

    // format!("Hello, {}!", name.as_deref().unwrap_or("World"))
}
#[endpoint]
async fn file_t(data: FormBody<Waza>, file_name: FormFile) {

    // format!("Hello, {}!", name.as_deref().unwrap_or("World"))
}

#[handler]
async fn inertia_test_endpoint() -> Inertia<()> {
    Inertia::new_no_props("hello")
}

#[derive(ToSchema, Debug, Deserialize)]
struct Waza {
    id: String,
    name: String,
}

#[derive(Debug, ToSchema)]
pub struct UserProfile {
    pub username: String,
    pub email: String,
    pub avatar: UploadedFile, // Para un solo archivo
}

#[async_trait::async_trait]
impl FromMultipart for UserProfile {
    // async fn parse_from_multipart(req: &Request) -> Result<Self, StatusError> {
    // }
    //
    async fn parse_from_multipart(req: &mut Request) -> Result<Self, thunder::ValidationErrors> {
        // Now you can use `.await` safely here
        let username: String = req.form("username").await.unwrap();
        let email: String = req.form("email").await.unwrap();
        let file = req.file("avatar").await.unwrap().clone();

        Ok(UserProfile {
            username: username,
            email: email,
            avatar: UploadedFile {
                file_name: todo!(),
                path: file.path().clone(),
                content_type: file.content_type(),
            },
        })
    }
}

#[derive(Debug, Validate)]
struct Name {
    #[validate(email, min = 34, max = 3)]
    field: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let router = Router::new().push(
        Router::with_path("hello")
            .get(hello)
            .patch(bye)
            .post(file_t),
    );
    let doc = OpenApi::new("test api", "0.0.1").merge_router(&router);

    let router = router
        .unshift(doc.into_router("/api-doc/openapi.json"))
        .unshift(SwaggerUi::new("/api-doc/openapi.json").into_router("/swagger-ui"));

    let acceptor = TcpListener::new("0.0.0.0:5800").bind().await;

    println!("{:?}", router);
    Server::new(acceptor).serve(router).await;
}
