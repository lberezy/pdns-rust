use serde::{Deserialize, Serialize};
// use pallet::search::IndexBuilder;
// use tantivy::schema::SchemaBuilder;

#[derive(Serialize, Deserialize, Debug, Default, pallet::DocumentLike)]
#[pallet(tree_name = "pdns")]
pub struct PDNSRecord {
    #[pallet(default_search_field)]
    domain: String,
    #[pallet(default_search_field)]
    address: String,
    #[pallet(index_field_type = "i64")]
    // #[serde(with = "ts_milliseconds")]
    // TODO: Use DateTime<Utc> and serialize to i64 with custom impl of pallet::DocumentLike
    first_seen: i64,
    #[pallet(index_field_type = "i64")]
    // #[serde(with = "ts_milliseconds")]
    last_seen: i64,
    count: Option<u64>,
}

// impl pallet::DocumentLike for PDNSRecord {
//     type IndexFieldsType = Vec<tantivy::schema::Field>;

//     fn as_index_document(
//         &self,
//         index_fields: &Self::IndexFieldsType,
//     ) -> pallet::err::Result<tantivy::Document> {

//         IndexBuilder::with_default_search_fields_builder(self, |x| {

//         })
//         todo!()
//     }

//     fn tree_builder() -> pallet::db::TreeBuilder {
//         // pallet::db::TreeBuilder::default()
//         Some("pdns").into()
//     }

//     fn index_builder() -> pallet::search::IndexBuilder<Self::IndexFieldsType> {
//         // pallet::search::IndexBuilder::default()
//         Ok(vec![])
//     }
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    std::env::set_var("RUST_LOG", "info");
    log::info!("Starting up.");
    const DB_PATH: &'static str = "./database";
    let db = sled::open(DB_PATH)?;

    let store = pallet::Store::builder()
        .with_db(db)
        .with_index_dir(DB_PATH)
        .finish()?;

    let records = vec![PDNSRecord {
        domain: "example.com".into(),
        address: "1.2.3.4".into(),
        count: Some(1),
        first_seen: 1,
        last_seen: 2,
    }];

    let _ = store.create_multi(&records)?;

    let results = store.search("last_seen:>2")?;

    log::info!("{:?}", results);

    Ok(())
}
