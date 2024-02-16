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

	// -- Create Comment
	let req_create_comment = hc.do_post(
		"/api/rpc",
		json!({
				"id": 1,
				"method": "create_comment",
				"params": {
						"data": {
								"article_id": 1001,
								"user_id": 1000,
								"content": "Test comment content.",
								"replay_to": null // or specify the comment ID to reply to
						}
				}
		}),
	);
	let result = req_create_comment.await?;
	result.print().await?;
	let comment_id = result.json_value::<i64>("/result/data/id")?;

	// -- Get Comment
	let req_get_comment = hc.do_post(
		"/api/rpc",
		json!({
				"id": 1,
				"method": "get_comment",
				"params": {
						"id": comment_id
				}
		}),
	);
	let result = req_get_comment.await?;
	result.print().await?;

	// -- Update Comment
	let req_update_comment = hc.do_post(
		"/api/rpc",
		json!({
				"id": 1,
				"method": "update_comment",
				"params": {
						"id": comment_id,
						"data": {
								"content": "Updated test comment content."
						}
				}
		}),
	);
	let result = req_update_comment.await?;
	result.print().await?;

	// -- Delete Comment
	let req_delete_comment = hc.do_post(
		"/api/rpc",
		json!({
				"id": 1,
				"method": "delete_comment",
				"params": {
						"id": comment_id
				}
		}),
	);
	let result = req_delete_comment.await?;
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
