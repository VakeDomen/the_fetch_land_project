use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct BulkResponse {
    pub object: String,
    pub has_more: bool,
    pub data: Vec<BulkData>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct BulkData {
    pub object: String,
    pub id: String,
    #[serde(rename="type")]
    pub data_type: String,
    pub updated_at: String,
    pub uri: String,
    pub name: String,
    pub description: String,
    pub compressed_size: i64,
    pub download_uri: String,
    pub content_type: String,
    pub content_encoding: String,
}