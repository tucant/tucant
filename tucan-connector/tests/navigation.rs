use tucan_connector::{root::root, Tucan};

#[tokio::test]
pub async fn root_page() {
    let tucan = Tucan::new().await.unwrap();
    root(&tucan.client).await.unwrap();
}
