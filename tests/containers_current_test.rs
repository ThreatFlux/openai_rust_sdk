//! Current official container-file wire fixtures and pagination errors.

use httpmock::prelude::*;
use openai_rust_sdk::OpenAIError;
use openai_rust_sdk::api::common::{ApiClientConstructors, StandardListParams};
use openai_rust_sdk::api::containers::ContainersApi;
use openai_rust_sdk::models::containers::ContainerFileMetadata;
use serde_json::{Value, json};

fn metadata() -> Value {
    json!({"id":"file_fixture","bytes":42,"container_id":"cntr_fixture",
        "created_at":1,"object":"container.file","path":"/mnt/data/result.txt",
        "source":"assistant","future_metadata":{"available":true}})
}

#[test]
fn official_file_metadata_has_no_invented_filename_or_size_requirements() {
    let original = metadata();
    let file: ContainerFileMetadata = serde_json::from_value(original.clone()).unwrap();
    assert_eq!(file.bytes, 42);
    assert_eq!(file.extra["future_metadata"]["available"], true);
    assert_eq!(serde_json::to_value(file).unwrap(), original);
    let mut malformed = metadata();
    malformed.as_object_mut().unwrap().remove("bytes");
    assert!(serde_json::from_value::<ContainerFileMetadata>(malformed).is_err());
}

#[tokio::test]
async fn retrieve_and_list_use_current_endpoints_and_cursor_query() {
    let server = MockServer::start_async().await;
    let retrieve = server
        .mock_async(|when, then| {
            when.method(GET)
                .path("/v1/containers/cntr_fixture/files/file_fixture")
                .header("authorization", "Bearer fixture-key");
            then.status(200).json_body(metadata());
        })
        .await;
    let list = server
        .mock_async(|when, then| {
            when.method(GET)
                .path("/v1/containers/cntr_fixture/files")
                .query_param("after", "file_previous")
                .query_param("limit", "2")
                .query_param("order", "asc");
            then.status(200)
                .json_body(json!({"object":"list","data":[metadata()],
            "first_id":"file_fixture","last_id":"file_fixture","has_more":true,"future":1}));
        })
        .await;
    let api =
        ContainersApi::new_with_base_url("fixture-key".to_owned(), server.base_url()).unwrap();
    assert_eq!(
        api.retrieve_file("cntr_fixture", "file_fixture")
            .await
            .unwrap()
            .bytes,
        42
    );
    let page = api
        .list_files_with_params(
            "cntr_fixture",
            &StandardListParams {
                limit: Some(2),
                order: Some("asc".into()),
                after: Some("file_previous".into()),
                before: None,
            },
        )
        .await
        .unwrap();
    assert!(page.has_more);
    assert_eq!(page.last_id.as_deref(), Some("file_fixture"));
    assert_eq!(page.extra["future"], 1);
    retrieve.assert_async().await;
    list.assert_async().await;
}

#[tokio::test]
async fn invalid_paths_and_unsupported_list_controls_fail_before_http() {
    let server = MockServer::start_async().await;
    let any = server
        .mock_async(|_, then| {
            then.status(500);
        })
        .await;
    let api =
        ContainersApi::new_with_base_url("fixture-key".to_owned(), server.base_url()).unwrap();
    assert!(matches!(
        api.retrieve_file("../escape", "file_fixture").await,
        Err(OpenAIError::InvalidRequest(_))
    ));
    assert!(
        api.retrieve_file("cntr_fixture", "file?query=bad")
            .await
            .is_err()
    );
    for params in [
        StandardListParams {
            before: Some("file_fixture".into()),
            ..Default::default()
        },
        StandardListParams {
            limit: Some(0),
            ..Default::default()
        },
        StandardListParams {
            limit: Some(101),
            ..Default::default()
        },
        StandardListParams {
            order: Some("unexpected".into()),
            ..Default::default()
        },
    ] {
        assert!(
            api.list_files_with_params("cntr_fixture", &params)
                .await
                .is_err()
        );
    }
    assert_eq!(any.calls_async().await, 0);
}

#[tokio::test]
async fn metadata_retrieval_preserves_api_errors() {
    let server = MockServer::start_async().await;
    server
        .mock_async(|when, then| {
            when.method(GET)
                .path("/v1/containers/cntr_fixture/files/file_fixture");
            then.status(404)
                .json_body(json!({"error":{"message":"not found","type":"invalid_request_error"}}));
        })
        .await;
    let api =
        ContainersApi::new_with_base_url("fixture-key".to_owned(), server.base_url()).unwrap();
    assert!(matches!(
        api.retrieve_file("cntr_fixture", "file_fixture").await,
        Err(OpenAIError::Api {
            status_code: 404,
            ..
        })
    ));
}
