use animethemes_server_rust::app::App;
use loco_rs::boot::run_task;
use loco_rs::task;
use loco_rs::testing::prelude::*;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_can_run_sync_favorites_count() {
    let boot = boot_test::<App>().await.unwrap();

    assert!(
        run_task::<App>(
            &boot.app_context,
            Some(&"sync:favorites-count".to_string()),
            &task::Vars::default()
        )
        .await
        .is_ok()
    );
}
