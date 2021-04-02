use async_graphql::Result;
use async_trait::async_trait;
use dataloader::cached::Loader;
#[derive(Clone)]
pub enum BatchFnLoadError {
    NotFound,
    DBError(String),
}

use crate::service::PDNSRecordPort;
use crate::models::PDNSRecord;
// use crate::persistence::BatchFnLoadError;

use super::sample_batcher::{SampleBatchLoadHashMapValue, SampleBatcher};
pub struct PDNSRepository {
    sample_loader: Loader<String, SampleBatchLoadHashMapValue, SampleBatcher>,
}

impl SampleRepository {
    pub fn new(pool: DatabaseConnectionPool) -> Self {
        Self {
            sample_loader: Loader::new(SampleBatcher::new(pool)),
        }
    }
}

#[async_trait]
impl LoadSamplesPort for SampleRepository {
    async fn load_sample(&self, hash: String) -> Result<Option<Sample>> {
        match self.sample_loader.load(hash).await {
            Ok(sample) => Ok(Some(sample)),
            Err(e) => match e {
                BatchFnLoadError::DBError(db_error) => Err(db_error.into()),
                BatchFnLoadError::NotFound => Ok(None),
            },
        }
    }
}
