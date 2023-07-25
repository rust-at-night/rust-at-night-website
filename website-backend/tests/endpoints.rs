//! Test structure is inspired from `https://github.com/tokio-rs/axum/discussions/555`

mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use itertools::Itertools;
use serde_json::Value;
use serial_test::serial;
use std::net::TcpListener;
use website_backend_lib::data::post::Post;

#[test]
#[serial]
fn test_health_endpoint() {
    common::rt().block_on(async {
        let (app, options) = common::init_app().await;

        let listener = TcpListener::bind(options.addr).unwrap();
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            axum::Server::from_tcp(listener)
                .unwrap()
                .serve(app.router().into_make_service())
                .await
                .unwrap();
        });

        let client = hyper::Client::new();

        let response = client
            .request(
                Request::builder()
                    .uri(format!("http://{addr}/health",))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        // Expect no json body
        assert!(serde_json::from_slice::<Value>(&body).is_err());
    });
}

#[test]
#[serial]
fn test_posts_endpoint() {
    common::rt().block_on(async {
        let (app, options) = common::init_app().await;

        let listener = TcpListener::bind(options.addr).unwrap();
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            axum::Server::from_tcp(listener)
                .unwrap()
                .serve(app.router().into_make_service())
                .await
                .unwrap();
        });

        let client = hyper::Client::new();

        let response = client
            .request(
                Request::builder()
                    .uri(format!("http://{addr}/api/v1/posts",))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let posts = serde_json::from_slice::<Vec<Post>>(&body).unwrap();

        assert!(!posts.is_empty());

        let posts_from_ali = posts
            .into_iter()
            .filter(|post| post.writer == "ali")
            .collect_vec();

        // Also currently checks if the seeding is working correctly.
        assert_eq!(posts_from_ali.len(), 3);
    });
}
