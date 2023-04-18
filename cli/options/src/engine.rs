use crate::*;

#[derive(JsonSchema, Debug, Clone, Serialize, Deserialize)]
pub struct Options {
    pub backend: Backend,
    pub input: Vec<circus_frontend_exporter::Item>,
}

#[derive(JsonSchema, Debug, Clone, Serialize, Deserialize)]
pub struct File {
    pub path: String,
    pub contents: String,
}

#[derive(JsonSchema, Debug, Clone, Serialize, Deserialize)]
pub struct Output {
    pub diagnostics: Vec<circus_diagnostics::Diagnostics<circus_frontend_exporter::Span>>,
    pub files: Vec<File>,
}
