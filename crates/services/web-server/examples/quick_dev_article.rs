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

	// -- Create Article
	let req_create_article = hc.do_post(
		"/api/rpc",
		json!({
						"id": 1,
						"method": "create_article",
						"params": {
										"data": {
														"title": "test_create_ok article 01",
														"content": "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
														"category_id": 1001,
														"author_id": 1000,
														"approval_state": "Draft",
														"image_url": "https://example.com/image.jpg"
										}
						}
		}),
	);
	let result = req_create_article.await?;
	result.print().await?;
	let article_id = result.json_value::<i64>("/result/data/id")?;

	// -- Get Article
	let req_get_article = hc.do_post(
		"/api/rpc",
		json!({
						"id": 1,
						"method": "get_article",
						"params": {
										"id": article_id
						}
		}),
	);
	let result = req_get_article.await?;
	result.print().await?;

	// -- Update Article
	let req_update_article = hc.do_post(
		"/api/rpc",
		json!({
						"id": 1,
						"method": "update_article",
						"params": {
										"id": article_id,
										"data": {
														"title": "Updated Sample Article",
														"content": "This is the updated content of the sample article.",
														"approval_state": "Draft" // Include the approval_state field
										}
						}
		}),
	);
	let result = req_update_article.await?;
	result.print().await?;

	// -- Delete Article
	let req_delete_article = hc.do_post(
		"/api/rpc",
		json!({
						"id": 1,
						"method": "delete_article",
						"params": {
										"id": article_id
						}
		}),
	);
	let result = req_delete_article.await?;
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
