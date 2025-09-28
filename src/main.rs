use salvo::oapi::extract::*;
use salvo::{Extractible, prelude::*};
use sea_orm::ActiveModelBehavior;
use valipower::Validate;

#[endpoint]
async fn hello(name: QueryParam<String, false>) -> String {
    format!("Hello, {}!", name.as_deref().unwrap_or("World"))
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

#[derive(Debug)]
pub struct UserProfile {
    pub username: String,
    pub email: String,
    pub avatar: UploadedFile, // Para un solo archivo
                              // pub documents: Vec<UploadedFile>, // Si esperas múltiples archivos
}

impl salvo::Extractible for UserProfile {
    async fn extract(
        req: &'ex mut Request,
    ) -> Result<Self, impl Writer + Send + std::fmt::Debug + 'static>
    where
        Self: Sized,
    {
        // Obtenemos el stream multipart o devolvemos un error 400
        let multipart = req.multipart().await.ok_or_else(|| {
            todo!()
            // salvo::Error::new(
            //     StatusCode::BAD_REQUEST,
            //     "The request is not multipart",
            //     None,
            // )
        })?;

        // Usamos una función auxiliar para procesar los datos
        let (mut fields, mut files) = process_multipart(multipart).await?;

        // Verificamos y extraemos los campos de texto requeridos
        let username = fields.remove("username").ok_or_else(|| {
            salvo::Error::new(StatusCode::BAD_REQUEST, "Missing field: username", None)
        })?;

        let email = fields.remove("email").ok_or_else(|| {
            salvo::Error::new(StatusCode::BAD_REQUEST, "Missing field: email", None)
        })?;

        // Verificamos y extraemos el archivo requerido
        let avatar = files.remove("avatar").ok_or_else(|| {
            salvo::Error::new(StatusCode::BAD_REQUEST, "Missing file: avatar", None)
        })?;

        // Construimos y devolvemos el struct final
        Ok(UserProfile {
            username,
            email,
            avatar,
        })
    }
}
// impl Extractible for Waza {
//     // fn extract(
//     //     req: &'ex mut Request,
//     // ) -> impl Future<Output = Result<Self, impl Writer + Send + std::fmt::Debug + 'static>> + Send
//     // where
//     //     Self: Sized,
//     // {
//     //     todo!()
//     // }
//     // fn metadata() -> &'static salvo::extract::Metadata {
//     //     todo!()
//     // }
// }
#[endpoint]
async fn waza(w: FormBody<Waza>) {
    todo!()
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
