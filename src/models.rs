use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
// use pallet::search::IndexBuilder;
// use tantivy::schema::SchemaBuilder;

#[derive(Serialize, Deserialize, Debug, Default, pallet::DocumentLike, SimpleObject)]
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
