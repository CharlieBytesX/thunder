use std::fmt::{self, Debug, Display, Formatter};
use std::ops::{Deref, DerefMut};

use salvo::http::{StatusCode, StatusError};
use salvo::oapi::{
    Components, Content, EndpointArgRegister, Operation, RequestBody, ToRequestBody, ToSchema,
};
use salvo::writing::Json;
use salvo::{Depot, Response};
use salvo_core::extract::{Extractible, Metadata};
use salvo_core::{Request, Writer, async_trait};

use crate::ValidationErrors;

#[async_trait]
pub trait FromMultipart: Sized {
    /// The error type returned when parsing fails.

    /// The method that performs the custom parsing logic.
    async fn parse_from_multipart(req: &mut Request) -> Result<Self, ValidationErrors>;
}

pub struct MultipartValidated<T>(pub T);
impl<T> MultipartValidated<T> {
    /// Consumes self and returns the value of the parameter.
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> Deref for MultipartValidated<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for MultipartValidated<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'de, T> ToRequestBody for MultipartValidated<T>
where
    T: ToSchema,
{
    fn to_request_body(components: &mut Components) -> RequestBody {
        RequestBody::new()
            .description("Extract form format data from request.")
            // .add_content(
            //     "application/x-www-form-urlencoded",
            //     Content::new(T::to_schema(components)),
            // )
            .add_content(
                "multipart/form-data",
                Content::new(T::to_schema(components)),
            )
    }
}

impl<T> fmt::Debug for MultipartValidated<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: Display> Display for MultipartValidated<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<'ex, T> Extractible<'ex> for MultipartValidated<T>
where
    T: Send + FromMultipart + Debug,
{
    fn metadata() -> &'static Metadata {
        static METADATA: Metadata = Metadata::new("");
        &METADATA
    }
    async fn extract(
        req: &'ex mut Request,
    ) -> Result<Self, impl Writer + Send + fmt::Debug + 'static> {
        let value = match T::parse_from_multipart(req).await {
            Ok(v) => v,
            Err(e) => {
                //TODO: verify if its ok
                return Err(e);
            }
        };
        return Ok(MultipartValidated(value));
    }
}

#[async_trait]
impl Writer for ValidationErrors {
    async fn write(mut self, _: &mut Request, _: &mut Depot, res: &mut Response) {
        res.status_code(
            self.status_code
                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
        );
        res.render(Json(self));
    }
}

#[async_trait]
impl<'de, T> EndpointArgRegister for MultipartValidated<T>
where
    T: ToSchema,
{
    fn register(components: &mut Components, operation: &mut Operation, _arg: &str) {
        let request_body = Self::to_request_body(components);
        let _ = <T as ToSchema>::to_schema(components);
        operation.request_body = Some(request_body);
    }
}
