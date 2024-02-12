use crate::ctx::Ctx;
use crate::model::conv::{ConvBmc, ConvForCreate};
use crate::model::{self, ModelManager};
use modql::filter::OpValString;

// region:    --- Conv seed/clean

pub async fn seed_convs(
	ctx: &Ctx,
	mm: &ModelManager,
	agent_id: i64,
	titles: &[&str],
) -> model::Result<Vec<i64>> {
	let mut ids = Vec::new();

	for title in titles {
		let id = seed_conv(ctx, mm, agent_id, title).await?;
		ids.push(id);
	}

	Ok(ids)
}

pub async fn seed_conv(
	ctx: &Ctx,
	mm: &ModelManager,
	agent_id: i64,
	title: &str,
) -> model::Result<i64> {
	ConvBmc::create(
		ctx,
		mm,
		ConvForCreate {
			agent_id,
			title: Some(title.to_string()),
			..Default::default()
		},
	)
	.await
}

pub async fn clean_convs(
	ctx: &Ctx,
	mm: &ModelManager,
	contains_title: &str,
) -> model::Result<usize> {
	let convs = ConvBmc::list(
		ctx,
		mm,
		Some(vec![model::conv::ConvFilter {
			title: Some(OpValString::Contains(contains_title.to_string()).into()),
			..Default::default()
		}]),
		None,
	)
	.await?;

	let count = convs.len();

	for conv in convs {
		ConvBmc::delete(ctx, mm, conv.id).await?;
	}

	Ok(count)
}

// endregion: --- Conv seed/clean
