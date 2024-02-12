use modql::filter::OpValString;

use crate::ctx::Ctx;
use crate::model::article::{
	ApprovalState, ArticleBmc, ArticleFilter, ArticleForCreate,
};
use crate::model::{self, ModelManager};

// region:    --- Article seed/clean

pub async fn seed_articles(
	ctx: &Ctx,
	mm: &ModelManager,
	data: &[(&str, &str, Option<i32>, i64, ApprovalState, Option<String>)],
) -> model::Result<Vec<i64>> {
	let mut ids = Vec::new();

	for (title, content, category_id, author_id, approval_state, image_url) in data {
		let id = seed_article(
			ctx,
			mm,
			*title,
			*content,
			*category_id,
			*author_id,
			*approval_state,
			image_url.clone(),
		)
		.await?;
		ids.push(id);
	}

	Ok(ids)
}

pub async fn seed_article(
	ctx: &Ctx,
	mm: &ModelManager,
	title: &str,
	content: &str,
	category_id: Option<i32>,
	author_id: i64,
	approval_state: ApprovalState,
	image_url: Option<String>,
) -> model::Result<i64> {
	ArticleBmc::create(
		ctx,
		mm,
		ArticleForCreate {
			title: title.to_string(),
			content: content.to_string(),
			category_id,
			author_id,
			approval_state,
			image_url,
		},
	)
	.await
}

pub async fn clean_articles(
	ctx: &Ctx,
	mm: &ModelManager,
	contains_title: &str,
) -> model::Result<usize> {
	let articles = ArticleBmc::list(
		ctx,
		mm,
		Some(vec![ArticleFilter {
			title: Some(OpValString::Contains(contains_title.to_string()).into()),
			..Default::default()
		}]),
		None,
	)
	.await?;
	let count = articles.len();

	for article in articles {
		ArticleBmc::delete(ctx, mm, article.id).await?;
	}

	Ok(count)
}

// endregion: --- Article seed/clean
