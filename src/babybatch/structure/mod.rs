pub mod resource;
pub mod thread_pool;
pub mod worker;

#[derive(Debug)]
pub struct BabyBatchResponse {
    pub body: serde_json::Value,
    pub page_token: String,
}
