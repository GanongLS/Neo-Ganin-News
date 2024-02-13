use crate::ctx::Ctx;
use crate::generate_common_bmc_fns;
use crate::model::{
	base::{self, DbBmc},
	modql_utils::time_to_sea_value,
	ModelManager, Result,
};

use modql::{
	field::Fields,
	filter::{FilterNodes, ListOptions, OpValsInt64, OpValsString, OpValsValue},
};

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use lib_utils::time::Rfc3339;
use sqlx::{types::time::OffsetDateTime, FromRow};

// region:    --- Comment Types

#[serde_as]
#[derive(Clone, Debug, Fields, FromRow, Serialize)]
pub struct Comment {
	// Main field
	pub id: i64,
	pub article_id: i64,
	pub user_id: i64,
	pub content: String,
  // if null then the comment are article root comment, if there then it is a replay to a comment, 
	pub replay_to: Option<i64>, // Nullable

	// -- Timestamps
	// (creator and last modified user_id/time)
	#[serde_as(as = "Rfc3339")]
	pub creation_time: OffsetDateTime,
	#[serde_as(as = "Rfc3339")]
	pub updated_time: OffsetDateTime,
}

#[derive(Fields, Deserialize, Default)]
pub struct CommentForCreate {
	pub article_id: i64,
	pub user_id: i64,
	pub content: String,
	pub replay_to: Option<i64>,
}

#[derive(Fields, Deserialize, Default)]
pub struct CommentForUpdate {
	pub content: Option<String>,
	// Add other fields for update if needed
}

#[derive(FilterNodes, Deserialize, Default)]
pub struct CommentFilter {
	pub id: Option<OpValsInt64>,
	pub article_id: Option<OpValsInt64>,
	pub user_id: Option<OpValsInt64>,
	pub content: Option<OpValsString>,

	pub replay_to: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	pub creation_time: Option<OpValsValue>,
}

// endregion: --- Comment Types

// region:    --- CommentBmc

pub struct CommentBmc;

impl DbBmc for CommentBmc {
	const TABLE: &'static str = "comment";
}

// This will generate the `impl CommentBmc {...}` with the default CRUD functions.
generate_common_bmc_fns!(
		Bmc: CommentBmc,
		Entity: Comment,
		ForCreate: CommentForCreate,
		ForUpdate: CommentForUpdate,
		Filter: CommentFilter,
);

// endregion: --- CommentBmc

// region:    --- Tests
#[cfg(test)]
mod tests {

	type Error = Box<dyn std::error::Error>;
	type Result<T> = core::result::Result<T, Error>;

	use super::*;
	use crate::_dev_utils::{self, clean_comments, seed_comment};

	use serial_test::serial;

	#[tokio::test]
	#[serial]
	async fn test_create_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_article_id = 1; // Example article ID
		let fx_user_id = 1000; // Example user ID
		let fx_content = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.";
		let fx_replay_to = None; // Example replay_to ID

		// -- Exec
		let fx_comment_c = CommentForCreate {
			article_id: fx_article_id,
			user_id: fx_user_id,
			content: fx_content.to_string(),
			replay_to: fx_replay_to,
		};
		let comment_id = CommentBmc::create(&ctx, &mm, fx_comment_c).await?;

		// -- Check
		let comment = CommentBmc::get(&ctx, &mm, comment_id).await?;
		assert_eq!(comment.article_id, fx_article_id);
		assert_eq!(comment.user_id, fx_user_id);
		assert_eq!(comment.content, fx_content);
		assert_eq!(comment.replay_to, fx_replay_to);

		// -- Clean
		let count = clean_comments(&ctx, &mm, "test_create_ok").await?;
		assert_eq!(count, 1, "Should have cleaned only 1 comment");

		Ok(())
	}

	#[tokio::test]
	#[serial]
	async fn test_update_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_article_id = 1; // Example article ID
		let fx_user_id = 1000; // Example user ID
		let fx_content = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.";
		let fx_replay_to = None; // Example replay_to ID

		let fx_comment_id = seed_comment(
			&ctx,
			&mm,
			fx_article_id,
			fx_user_id,
			fx_content,
			fx_replay_to,
		)
		.await?;

		let fx_content_updated = "Updated content.";
		// Example updated replay_to ID

		// -- Exec
		let fx_comment_u = CommentForUpdate {
			content: Some(fx_content_updated.to_string()),
			..Default::default()
		};
		CommentBmc::update(&ctx, &mm, fx_comment_id, fx_comment_u).await?;

		// -- Check
		let comment = CommentBmc::get(&ctx, &mm, fx_comment_id).await?;
		assert_eq!(comment.content, fx_content_updated);

		// -- Clean
		let count = clean_comments(&ctx, &mm, "test_update_ok").await?;
		assert_eq!(count, 1, "Should have cleaned only 1 comment");

		Ok(())
	}

	// Add other test cases as needed...
}

// endregion:    --- Tests
