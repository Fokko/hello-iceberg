use std::collections::HashMap;
use iceberg_catalog_rest::{RestCatalog, RestCatalogConfig};
use iceberg::Catalog;
use iceberg::TableIdent;
use std::env;

#[tokio::main]
async fn main() {
    let config = RestCatalogConfig::builder()
        .uri("https://api.tabular.io/ws".to_string())
        .warehouse("Fokko".to_string())
        .props(HashMap::from([
            ("credential".to_string(), env::var("credential").unwrap())
        ])).build();

    let catalog = RestCatalog::new(config);

    let all_namespaces = catalog.list_namespaces(None).await.unwrap();
    println!("Namespaces in current catalog: {:?}", all_namespaces);

    let wrapped = catalog
        .load_table(&TableIdent::from_strs(["examples", "nyc_taxi_locations"]).unwrap())
        .await;

    let table2 = wrapped.unwrap() ;

    println!("{:?}", table2.metadata());
    println!("{:?}", table2.scan().build().unwrap());
}
