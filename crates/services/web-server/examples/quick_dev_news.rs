#![allow(unused)] // For example code.

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For examples.

use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    // -- Login
    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd": "welcome"
        }),
    );
    req_login.await?.print().await?;

    // -- Create ArticleView
    let req_create_article_view = hc.do_post(
        "/api/rpc",
        json!({
            "id": 1,
            "method": "create_article_view",
            "params": {
                "data": {
                    "article_id": 1001,
                    "viewer_id": 1001,
                    "view_count": 10,
                    "likes": false,
                    "dislikes": false,
                    "SHARE": false
                }
            }
        }),
    );
    let result = req_create_article_view.await?;
    result.print().await?;
    let article_view_id = result.json_value::<i64>("/result/data/id")?;

    // -- Get ArticleView
    let req_get_article_view = hc.do_post(
        "/api/rpc",
        json!({
            "id": 1,
            "method": "get_article_view",
            "params": {
                "id": article_view_id
            }
        }),
    );
    let result = req_get_article_view.await?;
    result.print().await?;

    // -- Update ArticleView
    let req_update_article_view = hc.do_post(
        "/api/rpc",
        json!({
            "id": 1,
            "method": "update_article_view",
            "params": {
                "id": article_view_id,
                "data": {
                    "view_count": 2,
                    "likes": true,
                    "dislikes": false,
                    "SHARE": true
                }
            }
        }),
    );
    let result = req_update_article_view.await?;
    result.print().await?;

    // -- Delete ArticleView
    let req_delete_article_view = hc.do_post(
        "/api/rpc",
        json!({
            "id": 1,
            "method": "delete_article_view",
            "params": {
                "id": article_view_id
            }
        }),
    );
    let result = req_delete_article_view.await?;
    result.print().await?;

    // -- Logoff
    let req_logoff = hc.do_post(
        "/api/logoff",
        json!({
            "logoff": true
        }),
    );
    req_logoff.await?.print().await?;

    Ok(())
}
