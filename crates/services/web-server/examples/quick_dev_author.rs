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

	// region: AUTHOR VIEWS

	// -- Create Author
	let req_create_author = hc.do_post(
		"/api/rpc",
		json!({
						"id": 1,
						"method": "create_author",
						"params": {
										"data": {
														"user_id": 1003,
														"pen_name": "Test Author",
														"author_type": "Editor",
														"bio": "Test bio",
														"website": "https://example.com",
														"avatar_url": "https://example.com/avatar.jpg"
										}
						}
		}),
	);
	let result = req_create_author.await?;
	result.print().await?;
	let author_id = result.json_value::<i64>("/result/data/id")?;

	// -- Get Author
	let req_get_author = hc.do_post(
		"/api/rpc",
		json!({
						"id": 1,
						"method": "get_author",
						"params": {
										"id": author_id
						}
		}),
	);
	let result = req_get_author.await?;
	result.print().await?;

	// -- Update Author
	let req_update_author = hc.do_post(
		"/api/rpc",
		json!({
						"id": 1,
						"method": "update_author",
						"params": {
										"id": author_id,
										"data": {
														"pen_name": "Updated Test Author",
														"bio": "Updated bio",
														"website": "https://updated-example.com",
														"avatar_url": "https://updated-example.com/avatar.jpg"
										}
						}
		}),
	);
	let result = req_update_author.await?;
	result.print().await?;

	// -- Delete Author
	let req_delete_author = hc.do_post(
		"/api/rpc",
		json!({
						"id": 1,
						"method": "delete_author",
						"params": {
										"id": author_id
						}
		}),
	);
	let result = req_delete_author.await?;
	result.print().await?;

	// endregion: AUTHOR VIEWS

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
