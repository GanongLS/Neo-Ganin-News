use modql::filter::OpValString;

use crate::ctx::Ctx;
use crate::model::comment::{CommentBmc, CommentFilter, CommentForCreate, CommentType};
use crate::model::{self, ModelManager};

// region:    --- Comment seed/clean

pub async fn seed_comments(
	ctx: &Ctx,
	mm: &ModelManager,
	data: &[(i64, i64, &str, CommentType, Option<i64>)],
) -> model::Result<Vec<i64>> {
	let mut ids = Vec::new();

	for (article_id, user_id, content, comment_type, replay_to) in data {
		let id = seed_comment(
			ctx,
			mm,
			*article_id,
			*user_id,
			*content,
			*comment_type,
			*replay_to,
		)
		.await?;
		ids.push(id);
	}

	Ok(ids)
}

pub async fn seed_comment(
	ctx: &Ctx,
	mm: &ModelManager,
	article_id: i64,
	user_id: i64,
	content: &str,
	comment_type: CommentType,
	replay_to: Option<i64>,
) -> model::Result<i64> {
	CommentBmc::create(
		ctx,
		mm,
		CommentForCreate {
			article_id,
			user_id,
			content: content.to_string(),
			comment_type,
			replay_to,
		},
	)
	.await
}

pub async fn clean_comments(
	ctx: &Ctx,
	mm: &ModelManager,
	contains_content: &str,
) -> model::Result<usize> {
	// Assuming you have a list function in CommentBmc that accepts a filter
	let comments = CommentBmc::list(
		ctx,
		mm,
		// Construct a filter to find comments containing the specified content
		Some(vec![CommentFilter {
			content: Some(OpValString::Contains(contains_content.to_string()).into()),
			..Default::default()
		}]),
		None,
	)
	.await?;
	let count = comments.len();

	// Delete each comment found
	for comment in comments {
		CommentBmc::delete(ctx, mm, comment.id).await?;
	}

	Ok(count)
}

// endregion: --- Comment seed/clean
