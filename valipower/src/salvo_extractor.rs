use std::fmt::{self, Debug, Display, Formatter};
use std::ops::{Deref, DerefMut};

use salvo::oapi::{Components, Content, EndpointArgRegister, RequestBody, ToRequestBody, ToSchema};
use salvo_core::extract::{Extractible, Metadata};
use salvo_core::{Request, Writer, async_trait};
use serde::{Deserialize, Deserializer};

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
    T: Deserialize<'de> + ToSchema,
{
    fn to_request_body(components: &mut Components) -> RequestBody {
        RequestBody::new()
            .description("Extract form format data from request.")
            .add_content(
                "application/x-www-form-urlencoded",
                Content::new(T::to_schema(components)),
            )
            .add_content("multipart/*", Content::new(T::to_schema(components)))
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
    T: Deserialize<'ex> + Send,
{
    fn metadata() -> &'static Metadata {
        static METADATA: Metadata = Metadata::new("");
        &METADATA
    }
    async fn extract(
        req: &'ex mut Request,
    ) -> Result<Self, impl Writer + Send + fmt::Debug + 'static> {
        req.parse_form().await
    }
    async fn extract_with_arg(
        req: &'ex mut Request,
        _arg: &str,
    ) -> Result<Self, impl Writer + Send + fmt::Debug + 'static> {
        Self::extract(req).await
    }
}

impl<'de, T> Deserialize<'de> for MultipartValidated<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        T::deserialize(deserializer).map(MultipartValidated)
    }
}

#[async_trait]
impl<'de, T> EndpointArgRegister for MultipartValidated<T>
where
    T: Deserialize<'de> + ToSchema,
{
    fn register(components: &mut Components, operation: &mut Operation, _arg: &str) {
        let request_body = Self::to_request_body(components);
        let _ = <T as ToSchema>::to_schema(components);
        operation.request_body = Some(request_body);
    }
}
