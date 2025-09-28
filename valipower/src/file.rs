use salvo::{http::Mime, oapi::ToSchema};

#[derive(Debug)]
pub struct UploadedFile {
    pub path: std::path::PathBuf,
    pub file_name: Option<String>,
    pub content_type: Option<Mime>,
}

impl ToSchema for UploadedFile {
    fn to_schema(
        components: &mut salvo::oapi::Components,
    ) -> salvo::oapi::RefOr<salvo::oapi::schema::Schema> {
        todo!()
    }
}
