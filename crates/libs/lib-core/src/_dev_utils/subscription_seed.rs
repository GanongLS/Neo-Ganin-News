use time::OffsetDateTime;

// region:    --- Subscription seed/clean

use crate::ctx::Ctx;
use crate::model::subscription::{SubscriptionBmc, SubscriptionForCreate};
use crate::model::{self, ModelManager};

pub async fn seed_subscriptions(
	ctx: &Ctx,
	mm: &ModelManager,
	data: &[(i64, i64)],
) -> model::Result<Vec<i64>> {
	let mut ids = Vec::new();

	for (subscriber_id, author_id) in data {
		let id = seed_subscription(ctx, mm, *subscriber_id, *author_id).await?;
		ids.push(id);
	}

	Ok(ids)
}

pub async fn seed_subscription(
	ctx: &Ctx,
	mm: &ModelManager,
	subscriber_id: i64,
	author_id: i64,
) -> model::Result<i64> {
	let now = OffsetDateTime::now_utc();
	let duration = time::Duration::days(365);
	let end_time = now + duration;

	SubscriptionBmc::create(
		ctx,
		mm,
		SubscriptionForCreate {
			subscriber: subscriber_id,
			author_id,
			subscription_start_time: Some(now),
			subscription_end_time: Some(end_time),
		},
	)
	.await
}

pub async fn clean_subscriptions(ctx: &Ctx, mm: &ModelManager) -> model::Result<usize> {
	let subscriptions = SubscriptionBmc::list(ctx, mm, None, None).await?;
	let count = subscriptions.len();

	for subscription in subscriptions {
		SubscriptionBmc::delete(ctx, mm, subscription.id).await?;
	}

	Ok(count)
}

// endregion: --- Subscription seed/clean
