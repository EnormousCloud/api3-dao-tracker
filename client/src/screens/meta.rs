use serde::Serialize;

#[derive(Debug, Clone, Default, Serialize)]
pub struct PageMetaInfo {
    pub title: String,
    pub og_title: String,
    pub description: String,
    pub og_description: String,
}

impl PageMetaInfo {
    pub fn new(title: &str, description: &str) -> Self {
        Self {
            title: title.to_string(),
            og_title: title.to_string(),
            description: description.to_string(),
            og_description: description.to_string(),
        }
    }
}

pub trait MetaProvider {
    fn meta(&self) -> PageMetaInfo {
        PageMetaInfo {
            ..PageMetaInfo::default()
        }
    }
}
