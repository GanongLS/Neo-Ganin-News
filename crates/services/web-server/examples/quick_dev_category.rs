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

	// -- Create Category
	let req_create_category = hc.do_post(
		"/api/rpc",
		json!({
				"id": 1,
				"method": "create_category",
				"params": {
						"data": {
								"name": "Test Category",
								"description": "This is a test category",
								"parent_id": null,
								"is_featured": false
						}
				}
		}),
	);
	let result = req_create_category.await?;
	result.print().await?;
	let category_id = result.json_value::<i64>("/result/data/id")?;

	// -- Get Category
	let req_get_category = hc.do_post(
		"/api/rpc",
		json!({
				"id": 1,
				"method": "get_category",
				"params": {
						"id": category_id
				}
		}),
	);
	let result = req_get_category.await?;
	result.print().await?;

	// -- Update Category
	let req_update_category = hc.do_post(
		"/api/rpc",
		json!({
				"id": 1,
				"method": "update_category",
				"params": {
						"id": category_id,
						"data": {
								"name": "Updated Test Category",
								"description": "This is an updated test category",
								"is_featured": true
						}
				}
		}),
	);
	let result = req_update_category.await?;
	result.print().await?;

	// -- Delete Category
	let req_delete_category = hc.do_post(
		"/api/rpc",
		json!({
				"id": 1,
				"method": "delete_category",
				"params": {
						"id": category_id
				}
		}),
	);
	let result = req_delete_category.await?;
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
