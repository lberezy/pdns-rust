use std::sync::Arc;

use crate::service::PDNSService;

// use crate::persistence::{SampleRepository, SandboxRepository};

pub struct State {
    pub sample_service: SampleService,
    pub sandbox_service: SandboxService,
}

impl State {
    pub fn new() -> Self {
        const DB_PATH: &'static str = "./database";
        let db = sled::open(DB_PATH)?;

        let store = pallet::Store::builder()
            .with_db(db)
            .with_index_dir(DB_PATH)
            .finish()
            .unwrap();

        let pdns_repository = Arc::new(PDNSRepository::new(store.clone()));

        let pdns_service = PDNSService {
            PDNSRecord_port: pdns_repository
        };

        Self {
            sample_service,
            sandbox_service,
        }
    }
}
