use lb_rs::service::activity::RankingWeights;
use lb_rs::Lb;
use test_utils::*;
use tokio::time;
use tokio::time::Duration;
use uuid::Uuid;

#[tokio::test]
async fn suggest_docs() {
    let lb: Lb = test_core().await;
    lb.create_account(&random_name(), &url(), false)
        .await
        .unwrap();

    let document = lb.create_at_path("hello.md").await.unwrap();
    lb.write_document(document.id, "hello world".as_bytes())
        .await
        .unwrap();
    time::sleep(Duration::from_millis(100)).await;

    let expected_suggestions = lb.suggested_docs(RankingWeights::default()).await.unwrap();

    assert_eq!(vec![document.id], expected_suggestions);
}

#[tokio::test]
async fn suggest_docs_empty() {
    let lb: Lb = test_core().await;
    lb.create_account(&random_name(), &url(), false)
        .await
        .unwrap();

    let expected = lb.suggested_docs(RankingWeights::default()).await.unwrap();
    let actual: Vec<Uuid> = vec![];

    assert_eq!(actual, expected);
}

#[tokio::test]
async fn write_count() {
    let lb: Lb = test_core().await;
    lb.create_account(&random_name(), &url(), false)
        .await
        .unwrap();

    let document1 = lb.create_at_path("hello1.md").await.unwrap();
    for _ in 0..10 {
        lb.write_document(document1.id, "hello world".as_bytes())
            .await
            .unwrap();
    }

    let document2 = lb.create_at_path("hello2.md").await.unwrap();
    for _ in 0..20 {
        lb.write_document(document2.id, "hello world".as_bytes())
            .await
            .unwrap();
    }

    time::sleep(Duration::from_millis(100)).await;
    let actual_suggestions = lb
        .suggested_docs(RankingWeights { temporality: 0, io: 100 })
        .await
        .unwrap();
    let expected_suggestions = vec![document2.id, document1.id];
    assert_eq!(actual_suggestions, expected_suggestions);
}

#[tokio::test]
async fn write_count_multiple_docs() {
    let lb: Lb = test_core().await;
    lb.create_account(&random_name(), &url(), false)
        .await
        .unwrap();

    let document1 = lb.create_at_path("hello.md").await.unwrap();
    for _ in 0..10 {
        lb.write_document(document1.id, "hello world".as_bytes())
            .await
            .unwrap();
    }

    let document2 = lb.create_at_path("hello2.md").await.unwrap();
    for _ in 0..50 {
        lb.write_document(document2.id, "hello world".as_bytes())
            .await
            .unwrap();
    }

    let document3 = lb.create_at_path("hello3.md").await.unwrap();
    for _ in 0..55 {
        lb.write_document(document3.id, "hello world".as_bytes())
            .await
            .unwrap();
    }

    time::sleep(Duration::from_millis(100)).await;
    let actual_suggestions = lb
        .suggested_docs(RankingWeights { temporality: 0, io: 100 })
        .await
        .unwrap();

    let expected_suggestions = vec![document3.id, document2.id, document1.id];

    assert_eq!(actual_suggestions, expected_suggestions);
}

#[tokio::test]
async fn read_count() {
    let lb: Lb = test_core().await;
    lb.create_account(&random_name(), &url(), false)
        .await
        .unwrap();

    let document1 = lb.create_at_path("hello1.md").await.unwrap();
    for _ in 0..10 {
        lb.read_document(document1.id, true).await.unwrap();
    }

    let document2 = lb.create_at_path("hello2.md").await.unwrap();
    for _ in 0..20 {
        lb.read_document(document2.id, true).await.unwrap();
    }

    time::sleep(Duration::from_millis(100)).await;
    let actual_suggestions = lb
        .suggested_docs(RankingWeights { temporality: 0, io: 100 })
        .await
        .unwrap();
    let expected_suggestions = vec![document2.id, document1.id];
    assert_eq!(actual_suggestions, expected_suggestions);
}

#[tokio::test]
async fn read_count_multiple_docs() {
    let lb: Lb = test_core().await;
    lb.create_account(&random_name(), &url(), false)
        .await
        .unwrap();

    let document1 = lb.create_at_path("hello.md").await.unwrap();
    for _ in 0..10 {
        lb.read_document(document1.id, true).await.unwrap();
    }

    let document2 = lb.create_at_path("hello2.md").await.unwrap();
    for _ in 0..20 {
        lb.read_document(document2.id, true).await.unwrap();
    }

    let document3 = lb.create_at_path("hello3.md").await.unwrap();
    for _ in 0..100 {
        lb.read_document(document3.id, true).await.unwrap();
    }

    time::sleep(Duration::from_millis(100)).await;
    let actual_suggestions = lb
        .suggested_docs(RankingWeights { temporality: 0, io: 100 })
        .await
        .unwrap();

    let expected_suggestions = vec![document3.id, document2.id, document1.id];

    assert_eq!(actual_suggestions, expected_suggestions);
}

#[tokio::test]
async fn last_read() {
    let lb: Lb = test_core().await;
    lb.create_account(&random_name(), &url(), false)
        .await
        .unwrap();

    let document1 = lb.create_at_path("hello.md").await.unwrap();
    lb.read_document(document1.id, true).await.unwrap();

    time::sleep(Duration::from_millis(100)).await;

    let document2 = lb.create_at_path("hello2.md").await.unwrap();
    lb.read_document(document2.id, true).await.unwrap();

    time::sleep(Duration::from_millis(100)).await;
    let actual_suggestions = lb
        .suggested_docs(RankingWeights { temporality: 100, io: 0 })
        .await
        .unwrap();

    let expected_suggestions = vec![document2.id, document1.id];

    assert_eq!(actual_suggestions, expected_suggestions);
}

#[tokio::test]
async fn last_write() {
    let lb: Lb = test_core().await;
    lb.create_account(&random_name(), &url(), false)
        .await
        .unwrap();

    let document1 = lb.create_at_path("hello.md").await.unwrap();
    lb.write_document(document1.id, "hello world".as_bytes())
        .await
        .unwrap();

    time::sleep(Duration::from_millis(100)).await;

    let document2 = lb.create_at_path("hello2.md").await.unwrap();
    lb.write_document(document2.id, "hello world".as_bytes())
        .await
        .unwrap();

    time::sleep(Duration::from_millis(100)).await;
    let actual_suggestions = lb
        .suggested_docs(RankingWeights { temporality: 100, io: 0 })
        .await
        .unwrap();

    let expected_suggestions = vec![document2.id, document1.id];

    assert_eq!(actual_suggestions, expected_suggestions);
}
