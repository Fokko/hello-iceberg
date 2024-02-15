use iceberg_catalog_rest::{RestCatalog, RestCatalogConfig};
use iceberg::Catalog;
use iceberg::TableIdent;

#[tokio::main]
async fn main() {
    let config = RestCatalogConfig::builder().uri("https://api.dev.tabulardata.io/ws/".to_string()).build();

    let catalog = RestCatalog::new(config).await.unwrap();

    let all_namespaces = catalog.list_namespaces(None).await.unwrap();
    println!("Namespaces in current catalog: {:?}", all_namespaces);

    let table2 = catalog
        .load_table(&TableIdent::from_strs(["nyc", "taxis"]).unwrap())
        .await
        .unwrap();
    println!("{:?}", table2.metadata());

    println!("Hello, world!");
}
