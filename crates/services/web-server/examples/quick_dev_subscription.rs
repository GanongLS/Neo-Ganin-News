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

	// -- Create Subscription
	let req_create_subscription = hc.do_post(
		"/api/rpc",
		json!({
				"id": 1,
				"method": "create_subscription",
				"params": {
						"data": {
								"subscriber": 1000,
								"author_id": 1000,
								"subscription_start_time": null,
								"subscription_end_time": null
						}
				}
		}),
	);
	let result = req_create_subscription.await?;
	result.print().await?;
	let subscription_id = result.json_value::<i64>("/result/data/id")?;

	// -- Get Subscription
	let req_get_subscription = hc.do_post(
		"/api/rpc",
		json!({
				"id": 1,
				"method": "get_subscription",
				"params": {
						"id": subscription_id
				}
		}),
	);
	let result = req_get_subscription.await?;
	result.print().await?;

	// -- Update Subscription
	let req_update_subscription = hc.do_post(
		"/api/rpc",
		json!({
				"id": 1,
				"method": "update_subscription",
				"params": {
						"id": subscription_id,
						"data": {
								"author_id": 1002 // Updated author ID
						}
				}
		}),
	);
	let result = req_update_subscription.await?;
	result.print().await?;

	// -- Delete Subscription
	let req_delete_subscription = hc.do_post(
		"/api/rpc",
		json!({
				"id": 1,
				"method": "delete_subscription",
				"params": {
						"id": subscription_id
				}
		}),
	);
	let result = req_delete_subscription.await?;
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
