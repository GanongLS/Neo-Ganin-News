use crate::ctx::Ctx;
use crate::generate_common_bmc_fns;
use crate::model::{
	base::{self, DbBmc},
	modql_utils::time_to_sea_value,
	ModelManager, Result,
};

use modql::{
	field::Fields,
	filter::{
		FilterNodes, ListOptions, OpValsInt32, OpValsInt64, OpValsString, OpValsValue,
	},
};

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use lib_utils::time::Rfc3339;
use sqlx::{types::time::OffsetDateTime, FromRow};

// region:    --- Article Views Types
#[serde_as]
#[derive(Clone, Debug, Fields, FromRow, Serialize)]
pub struct ArticleView {
	pub id: i64,
	pub article_id: i64,
	pub viewer_id: i64,
	pub view_count: i32,
	pub likes: bool,
	pub dislikes: bool,
	pub share: bool,
	#[serde_as(as = "Rfc3339")]
	pub creation_time: OffsetDateTime,
	#[serde_as(as = "Rfc3339")]
	pub updated_time: OffsetDateTime,
}

#[derive(Fields, Deserialize, Default)]
pub struct ArticleViewForCreate {
	pub article_id: i64,
	pub viewer_id: i64,
	pub view_count: Option<i32>,
	pub likes: Option<bool>,
	pub dislikes: Option<bool>,
	pub share: Option<bool>,
}

#[derive(Fields, Deserialize, Default)]
pub struct ArticleViewForUpdate {
	pub view_count: Option<i32>,
	pub likes: Option<bool>,
	pub dislikes: Option<bool>,
	pub share: Option<bool>,
}

#[derive(FilterNodes, Deserialize, Default)]
pub struct ArticleViewFilter {
	pub id: Option<OpValsInt64>,
	pub article_id: Option<OpValsInt64>,
	pub viewer_id: Option<OpValsInt64>,
	pub view_count: Option<OpValsInt32>,
	pub likes: Option<OpValsString>, // Assuming you might want to filter by "true", "false", or "null"
	pub dislikes: Option<OpValsString>, // Assuming you might want to filter by "true", "false", or "null"
	pub share: Option<OpValsString>, // Assuming you might want to filter by "true", "false", or "null"
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	pub creation_time: Option<OpValsValue>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	pub updated_time: Option<OpValsValue>,
}
// endregion: --- Article Views Types

// region:    --- Article Views BMC

pub struct ArticleViewBmc;

impl DbBmc for ArticleViewBmc {
	const TABLE: &'static str = "article_view";
}

generate_common_bmc_fns!(
		Bmc: ArticleViewBmc,
		Entity: ArticleView,
		ForCreate: ArticleViewForCreate,
		ForUpdate: ArticleViewForUpdate,
		Filter: ArticleViewFilter,
);

// endregion: --- Article Views BMC

// region:    --- Tests

#[cfg(test)]
mod tests {
	use super::*;
	use crate::_dev_utils::{self, clean_article_views};

	use serial_test::serial;

	type Error = Box<dyn std::error::Error>;
	type Result<T> = core::result::Result<T, Error>; // For tests.

	#[serial]
	#[tokio::test]
	async fn test_create_article_view_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();

		let article_id = 1; // Example article ID
		let viewer_id = 123; // Example viewer ID
		let view_count = 10;
		let likes = true;
		let dislikes = false;
		let share = true;

		// -- Exec
		let article_view_c = ArticleViewForCreate {
			article_id,
			viewer_id,
			view_count: Some(view_count),
			likes: Some(likes),
			dislikes: Some(dislikes),
			share: Some(share),
		};
		let article_view_id = ArticleViewBmc::create(&ctx, &mm, article_view_c).await?;

		// -- Check
		let article_view = ArticleViewBmc::get(&ctx, &mm, article_view_id).await?;
		assert_eq!(article_view.article_id, article_id);
		assert_eq!(article_view.viewer_id, viewer_id);
		assert_eq!(article_view.view_count, view_count);
		assert_eq!(article_view.likes, likes);
		assert_eq!(article_view.dislikes, dislikes);
		assert_eq!(article_view.share, share);

		// -- Clean
		let count = clean_article_views(&ctx, &mm).await?;
		assert_eq!(count, 1, "Should have cleaned only 1 article view");

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_update_article_view_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();

		let article_id = 1; // Example article ID
		let viewer_id = 123; // Example viewer ID
		let view_count = 10;
		let likes = true;
		let dislikes = false;
		let share = true;

		// Create an initial article view
		let article_view_c = ArticleViewForCreate {
			article_id,
			viewer_id,
			view_count: Some(view_count),
			likes: Some(likes),
			dislikes: Some(dislikes),
			share: Some(share),
		};
		let article_view_id = ArticleViewBmc::create(&ctx, &mm, article_view_c).await?;

		// Updated data
		let updated_view_count = 20;
		let updated_likes = false;
		let updated_dislikes = true;
		let updated_share = false;

		// -- Exec
		let article_view_u = ArticleViewForUpdate {
			view_count: Some(updated_view_count),
			likes: Some(updated_likes),
			dislikes: Some(updated_dislikes),
			share: Some(updated_share),
		};
		ArticleViewBmc::update(&ctx, &mm, article_view_id, article_view_u).await?;

		// -- Check
		let article_view = ArticleViewBmc::get(&ctx, &mm, article_view_id).await?;
		assert_eq!(article_view.view_count, updated_view_count);
		assert_eq!(article_view.likes, updated_likes);
		assert_eq!(article_view.dislikes, updated_dislikes);
		assert_eq!(article_view.share, updated_share);

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_article_view_update_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();

		// Create an initial article view
		let article_id = 1; // Example article ID
		let viewer_id = 123; // Example viewer ID
		let view_count = 10;
		let likes = true;
		let dislikes = false;
		let share = true;

		let article_view_c = ArticleViewForCreate {
			article_id,
			viewer_id,
			view_count: Some(view_count),
			likes: Some(likes),
			dislikes: Some(dislikes),
			share: Some(share),
		};
		let article_view_id = ArticleViewBmc::create(&ctx, &mm, article_view_c).await?;

		// -- Exec: Update the article view
		let updated_view_count = 15;
		let updated_likes = false;

		let article_view_u = ArticleViewForUpdate {
			view_count: Some(updated_view_count),
			likes: Some(updated_likes),
			..Default::default()
		};

		ArticleViewBmc::update(&ctx, &mm, article_view_id, article_view_u).await?;

		// -- Check: Retrieve the updated article view
		let updated_article_view = ArticleViewBmc::get(&ctx, &mm, article_view_id).await?;
		assert_eq!(updated_article_view.view_count, updated_view_count);
		assert_eq!(updated_article_view.likes, updated_likes);

		// -- Clean
		let count = clean_article_views(&ctx, &mm).await?;
		assert_eq!(count, 1, "Should have cleaned only 1 article view");

		Ok(())
	}

	// Add more test cases as needed...
}

// endregion: --- Tests
