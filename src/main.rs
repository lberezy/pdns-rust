extern crate env_logger;
use actix_web::{web, App, HttpResponse, HttpServer};
use async_graphql::extensions::ApolloTracing;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{Request, Response};

mod state;
mod graphql;
mod models;
mod repository;
mod service;

use graphql::QueryRoot;
use state::State;

pub type PDNSSchema = Schema<QueryRoot, MutationRoot, SubscriptionRoot>;

async fn index(schema: web::Data<PDNSSchema>, req: Request) -> Response {
    schema.execute(req.into_inner()).await.into()
}

async fn index_playground() -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql"),
        )))
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // // Connect to database and generates collections and indexes
    let db_pool = state::get_pool().await.unwrap();
    // db_pool.truncate().await;

    state::populate_test_data(&db_pool).await;
    log::info!("Finished setting up database.");

    let state = State::new(db_pool);

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(state)
        .extension(ApolloTracing)
        .finish();

    let listen_addr = std::env::var("LISTEN_ADDR").unwrap_or_else(|_| "0.0.0.0:8000".to_string());
    log::info!("Playground: http://localhost:8000");

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .service(
                web::resource("/graphql")
                    .guard(actix_web::guard::Post())
                    .to(index),
            )
            .service(
                web::resource("/")
                    .guard(actix_web::guard::Get())
                    .to(index_playground),
            )
    })
    .bind(listen_addr)?
    .run()
    .await?;

    Ok(())
}

// use serde::{Deserialize, Serialize};
// // use pallet::search::IndexBuilder;
// // use tantivy::schema::SchemaBuilder;

// #[derive(Serialize, Deserialize, Debug, Default, pallet::DocumentLike)]
// #[pallet(tree_name = "pdns")]
// pub struct PDNSRecord {
//     #[pallet(default_search_field)]
//     domain: String,
//     #[pallet(default_search_field)]
//     address: String,
//     #[pallet(index_field_type = "i64")]
//     // #[serde(with = "ts_milliseconds")]
//     // TODO: Use DateTime<Utc> and serialize to i64 with custom impl of pallet::DocumentLike
//     first_seen: i64,
//     #[pallet(index_field_type = "i64")]
//     // #[serde(with = "ts_milliseconds")]
//     last_seen: i64,
//     count: Option<u64>,
// }

// // impl pallet::DocumentLike for PDNSRecord {
// //     type IndexFieldsType = Vec<tantivy::schema::Field>;

// //     fn as_index_document(
// //         &self,
// //         index_fields: &Self::IndexFieldsType,
// //     ) -> pallet::err::Result<tantivy::Document> {

// //         IndexBuilder::with_default_search_fields_builder(self, |x| {

// //         })
// //         todo!()
// //     }

// //     fn tree_builder() -> pallet::db::TreeBuilder {
// //         // pallet::db::TreeBuilder::default()
// //         Some("pdns").into()
// //     }

// //     fn index_builder() -> pallet::search::IndexBuilder<Self::IndexFieldsType> {
// //         // pallet::search::IndexBuilder::default()
// //         Ok(vec![])
// //     }
// // }

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     env_logger::init();
//     std::env::set_var("RUST_LOG", "info");
//     log::info!("Starting up.");
//     const DB_PATH: &'static str = "./database";
//     let db = sled::open(DB_PATH)?;

//     let store = pallet::Store::builder()
//         .with_db(db)
//         .with_index_dir(DB_PATH)
//         .finish()?;

//     let records = vec![PDNSRecord {
//         domain: "example.com".into(),
//         address: "1.2.3.4".into(),
//         count: Some(1),
//         first_seen: 1,
//         last_seen: 2,
//     }];

//     let _ = store.create_multi(&records)?;

//     let results = store.search("last_seen:>2")?;

//     log::info!("{:?}", results);

//     Ok(())
// }
