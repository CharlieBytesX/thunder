use salvo::{
    http::Mime,
    oapi::{
        Array, BasicType, EndpointArgRegister, KnownFormat, Object, Schema, SchemaFormat, ToSchema,
    },
};

#[derive(Debug)]
pub struct UploadedFile {
    pub path: std::path::PathBuf,
    pub file_name: Option<String>,
    pub content_type: Option<Mime>,
}

impl ToSchema for UploadedFile {
    fn to_schema(_components: &mut salvo::oapi::Components) -> salvo::oapi::RefOr<Schema> {
        // For OpenAPI, a file upload is represented as a string in binary format.
        let schema = Schema::from(
            Object::with_type(BasicType::String)
                .format(SchemaFormat::KnownFormat(KnownFormat::Binary))
                .description("The file to upload."),
        );
        salvo::oapi::RefOr::Type(schema)
    }
}
