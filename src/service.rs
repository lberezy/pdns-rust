use async_graphql::Result;
use async_trait::async_trait;
use std::sync::Arc;

use crate::models::PDNSRecord;
#[async_trait]
pub trait PDNSRecordPort {
    async fn query_record(&self, query: String) -> Result<Vec<PDNSRecord>>;
}

pub struct PDNSService {
    pub PDNSRecord_port: Arc<dyn PDNSRecordPort + Send + Sync>,
}

impl PDNSService {
    pub async fn query_records(&self, query: String) -> Result<Vec<PDNSRecord>> {
        self.PDNSRecord_port.query_record(query).await
    }
}
