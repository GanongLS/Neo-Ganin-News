use crate::ctx::Ctx;
use crate::model::category::{CategoryBmc, CategoryFilter, CategoryForCreate};
use crate::model::{self, ModelManager};
use modql::filter::OpValString;

// region:    --- Category seed/clean

pub async fn seed_categories(
	ctx: &Ctx,
	mm: &ModelManager,
	data: &[(&str, Option<&str>, Option<i64>, bool)],
) -> model::Result<Vec<i64>> {
	let mut ids = Vec::new();

	for (name, description, parent_id, is_featured) in data {
		let id =
			seed_category(ctx, mm, *name, *description, *parent_id, *is_featured).await?;
		ids.push(id);
	}

	Ok(ids)
}

pub async fn seed_category(
	ctx: &Ctx,
	mm: &ModelManager,
	name: &str,
	description: Option<&str>,
	parent_id: Option<i64>,
	is_featured: bool,
) -> model::Result<i64> {
	CategoryBmc::create(
		ctx,
		mm,
		CategoryForCreate {
			name: name.to_string(),
			description: description.map(|desc| desc.to_string()),
			parent_id,
			is_featured,
		},
	)
	.await
}

pub async fn clean_categories(
	ctx: &Ctx,
	mm: &ModelManager,
	contains_name: &str,
) -> model::Result<usize> {
	let categories = CategoryBmc::list(
		ctx,
		mm,
		Some(vec![CategoryFilter {
			name: Some(OpValString::Contains(contains_name.to_string()).into()),
			..Default::default()
		}]),
		None,
	)
	.await?;
	let count = categories.len();

	for category in categories {
		CategoryBmc::delete(ctx, mm, category.id).await?;
	}

	Ok(count)
}

// endregion: --- Category seed/clean
