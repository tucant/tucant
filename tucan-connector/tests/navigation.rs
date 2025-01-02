use tucan_connector::{
    externalpages::welcome::welcome, root::root, startpage_dispatch::one::startpage_dispatch_1,
    Tucan,
};

#[tokio::test]
pub async fn test_root_page() {
    let tucan = Tucan::new().await.unwrap();
    root(&tucan.client).await.unwrap();
}

/// /
/// redirects to
/// /scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=STARTPAGE_DISPATCH&ARGUMENTS=-N000000000000001
#[tokio::test]
pub async fn test_startpage_dispatch_1() {
    let tucan = Tucan::new().await.unwrap();
    startpage_dispatch_1(&tucan.client).await.unwrap();
}

/// /scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=STARTPAGE_DISPATCH&ARGUMENTS=-N000000000000001
/// redirects to
/// /scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N000000000000001,-N000344,-Awelcome
#[tokio::test]
pub async fn test_welcome() {
    let tucan = Tucan::new().await.unwrap();
    welcome(&tucan.client).await.unwrap();
}
