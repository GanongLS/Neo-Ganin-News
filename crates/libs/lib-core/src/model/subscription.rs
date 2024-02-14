use crate::ctx::Ctx;
use crate::generate_common_bmc_fns;
use crate::model::{
	base::{self, DbBmc},
	ModelManager, Result,
};

use modql::{
	field::Fields,
	filter::{FilterNodes, ListOptions, OpValsInt64, OpValsValue},
};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use lib_utils::time::Rfc3339;
use sqlx::{types::time::OffsetDateTime, FromRow};

// region:    --- Subscription Types
#[serde_as]
#[derive(Clone, Debug, Fields, FromRow, Serialize)]
pub struct Subscription {
	// Main field
	pub id: i64,
	pub subscriber: i64,
	pub author_id: i64,
	#[serde_as(as = "Rfc3339")]
	pub subscription_start_time: OffsetDateTime,
	#[serde_as(as = "Rfc3339")]
	pub subscription_end_time: OffsetDateTime,

	// -- Timestamps
	// (creator and last modified user_id/time)
	pub creator_id: i64,
	#[serde_as(as = "Rfc3339")]
	pub creation_time: OffsetDateTime,
	pub updater_id: i64,
	#[serde_as(as = "Rfc3339")]
	pub updated_time: OffsetDateTime,
}

#[derive(Fields, Deserialize, Default)]
pub struct SubscriptionForCreate {
	pub subscriber: i64,
	pub author_id: i64,
	pub subscription_start_time: Option<OffsetDateTime>,
	pub subscription_end_time: Option<OffsetDateTime>,
}

#[derive(Fields, Deserialize, Default)]
pub struct SubscriptionForUpdate {
	pub subscriber: Option<i64>,
	pub author_id: Option<i64>,
	pub subscription_start_time: Option<OffsetDateTime>,
	pub subscription_end_time: Option<OffsetDateTime>,
}

#[derive(FilterNodes, Deserialize, Default)]
pub struct SubscriptionFilter {
	pub id: Option<OpValsValue>,
	pub subscriber: Option<OpValsValue>,
	pub author_id: Option<OpValsInt64>,
}

// endregion: --- Subscription Types

// region:    --- SubscriptionBmc

pub struct SubscriptionBmc;

impl DbBmc for SubscriptionBmc {
	const TABLE: &'static str = "subscription";
}

// This will generate the `impl SubscriptionBmc {...}` with the default CRUD functions.
generate_common_bmc_fns!(
		Bmc: SubscriptionBmc,
		Entity: Subscription,
		ForCreate: SubscriptionForCreate,
		ForUpdate: SubscriptionForUpdate,
		Filter: SubscriptionFilter,
);

// endregion: --- SubscriptionBmc

// region:    --- Tests

#[cfg(test)]
mod tests {
	type Error = Box<dyn std::error::Error>;
	type Result<T> = core::result::Result<T, Error>; // For tests.

	use super::*;
	use crate::_dev_utils::{self, clean_subscription};
	use serial_test::serial;

	#[tokio::test]
	#[serial]
	async fn test_create_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();

		let subscriber_id = 1000; // Example subscriber ID
		let author_id = 1000; // Example author ID

		// -- Exec
		let subscription_id = SubscriptionBmc::create(
			&ctx,
			&mm,
			SubscriptionForCreate {
				subscriber: subscriber_id,
				author_id,
				subscription_start_time: None,
				subscription_end_time: None,
			},
		)
		.await?;

		// -- Check
		let subscription = SubscriptionBmc::get(&ctx, &mm, subscription_id).await?;
		assert_eq!(subscription.subscriber, subscriber_id);
		assert_eq!(subscription.author_id, author_id);

		// -- Clean
		let count = clean_subscription(&ctx, &mm, subscription_id).await?;
		assert_eq!(count, (), "Should have cleaned only 1 subscription");

		Ok(())
	}

	#[tokio::test]
	#[serial]
	async fn test_update_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();

		let subscriber_id = 1000; // Example subscriber ID
		let author_id = 1000; // Example author ID

		let subscription_id = SubscriptionBmc::create(
			&ctx,
			&mm,
			SubscriptionForCreate {
				subscriber: subscriber_id,
				author_id,
				subscription_start_time: None,
				subscription_end_time: None,
			},
		)
		.await?;

		let updated_author_id = 1002; // Updated author ID

		// -- Exec
		let subscription_u = SubscriptionForUpdate {
			author_id: Some(updated_author_id),
			..Default::default()
		};
		SubscriptionBmc::update(&ctx, &mm, subscription_id, subscription_u).await?;

		// -- Check
		let subscription = SubscriptionBmc::get(&ctx, &mm, subscription_id).await?;
		assert_eq!(subscription.author_id, updated_author_id);

		// -- Clean
		let count = clean_subscription(&ctx, &mm, subscription_id).await?;
		assert_eq!(count, (), "Should have cleaned only 1 subscription");

		Ok(())
	}
}

// endregion: --- Tests
