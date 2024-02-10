use crate::ctx::Ctx;
use crate::model::base::{self, prep_fields_for_update, DbBmc};
use crate::model::modql_utils::time_to_sea_value;
use crate::model::ModelManager;
use crate::model::{Error, Result};
use modql::field::{Field, Fields, HasFields};
use modql::filter::{FilterNodes, ListOptions, OpValsInt64, OpValsString, OpValsValue};
use sea_query::{Expr, Iden, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::FromRow;
use uuid::Uuid;

// region:    --- Subscription Types

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Subscription {
	pub id: i64,
	pub subscriber: i64,
	pub subscription_content: String,
	pub subscription_start_time: chrono::DateTime<chrono::Utc>,
	pub subscription_end_time: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize)]
pub struct SubscriptionForCreate {
	pub subscriber: i64,
	pub subscription_content: String,
	// Add other fields as needed
}

#[derive(Fields)]
pub struct SubscriptionForInsert {
	pub subscriber: i64,
	pub subscription_content: String,
	// Add other fields as needed
}

#[derive(Clone, Debug, FromRow, Fields, Serialize)]
pub struct SubscriptionForList {
	pub id: i64,
	pub subscriber: i64,
	pub subscription_content: String,
	pub subscription_start_time: chrono::DateTime<chrono::Utc>,
	pub subscription_end_time: chrono::DateTime<chrono::Utc>,
	// Add other fields as needed
}

/// Marker trait
pub trait SubscriptionBy:
	HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send
{
}

impl SubscriptionBy for Subscription {}
impl SubscriptionBy for SubscriptionForList {}

// endregion: --- Subscription Types

// region:    --- SubscriptionBmc

pub struct SubscriptionBmc;

impl DbBmc for SubscriptionBmc {
	const TABLE: &'static str = "subscriptions";
}

impl SubscriptionBmc {
	pub async fn create(
		ctx: &Ctx,
		mm: &ModelManager,
		subscription_c: SubscriptionForCreate,
	) -> Result<i64> {
		// Start the transaction
		let mm = mm.new_with_txn()?;
		mm.dbx().begin_txn().await?;

		// Create the subscription row
		let subscription_fi = SubscriptionForInsert {
			subscriber: subscription_c.subscriber,
			subscription_content: subscription_c.subscription_content,
		};

		let subscription_id = base::create::<Self, _>(ctx, &mm, subscription_fi).await?;

		// Commit the transaction
		mm.dbx().commit_txn().await?;

		Ok(subscription_id)
	}

	pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Subscription> {
		base::get::<Self, _>(ctx, mm, id).await
	}

	pub async fn update(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		subscription: SubscriptionForCreate,
	) -> Result<()> {
		// Start the transaction
		let mm = mm.new_with_txn()?;
		mm.dbx().begin_txn().await?;

		// Update the subscription
		let subscription_fi = SubscriptionForInsert {
			subscriber: subscription.subscriber,
			subscription_content: subscription.subscription_content,
		};

		base::update::<Self, _>(ctx, &mm, id, subscription_fi).await?;

		// Commit the transaction
		mm.dbx().commit_txn().await?;

		Ok(())
	}

	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		base::delete::<Self>(ctx, mm, id).await
	}

	pub async fn list(
		ctx: &Ctx,
		mm: &ModelManager,
		filter: Option<Vec<SubscriptionFilter>>,
		list_options: Option<ListOptions>,
	) -> Result<Vec<SubscriptionForList>> {
		base::list::<Self, _, SubscriptionForList>(ctx, mm, filter, list_options).await
	}
}

// endregion:    --- SubscriptionBmc
