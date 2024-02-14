use crate::ctx::Ctx;
use crate::model::article_view::{ArticleViewBmc, ArticleViewForCreate};
use crate::model::ModelManager;

use std::error::Error;

// region:    --- Article View seed/clean

pub async fn seed_article_views(
	ctx: &Ctx,
	mm: &ModelManager,
	data: &[(&i64, &i64, &i32, &bool, &bool, &bool)],
) -> Result<Vec<i64>, Box<dyn Error>> {
	let mut ids = Vec::new();

	for (article_id, viewer_id, view_count, likes, dislikes, share) in data {
		let id = seed_article_view(
			ctx,
			mm,
			**article_id,
			**viewer_id,
			**view_count,
			**likes,
			**dislikes,
			**share,
		)
		.await?;
		ids.push(id);
	}

	Ok(ids)
}

pub async fn seed_article_view(
	ctx: &Ctx,
	mm: &ModelManager,
	article_id: i64,
	viewer_id: i64,
	view_count: i32,
	likes: bool,
	dislikes: bool,
	share: bool,
) -> Result<i64, Box<dyn Error>> {
	let article_view_c = ArticleViewForCreate {
		article_id,
		viewer_id,
		view_count: Some(view_count),
		likes: Some(likes),
		dislikes: Some(dislikes),
		share: Some(share),
	};
	let id = ArticleViewBmc::create(ctx, mm, article_view_c).await?;
	Ok(id)
}

pub async fn clean_article_views(
	ctx: &Ctx,
	mm: &ModelManager,
) -> Result<usize, Box<dyn Error>> {
	let article_views = ArticleViewBmc::list(ctx, mm, Some(vec![]), None).await?;
	let count = article_views.len();

	for article_view in article_views {
		ArticleViewBmc::delete(ctx, mm, article_view.id).await?;
	}

	Ok(count)
}

pub async fn clean_article_view(
	ctx: &Ctx,
	mm: &ModelManager,
	article_view_id: i64,
) -> Result<(), Box<dyn Error>> {
	// Delete the specified article view
	let deleted_count = ArticleViewBmc::delete(ctx, mm, article_view_id).await?;

	Ok(deleted_count)
}

// endregion: --- Article View seed/clean
