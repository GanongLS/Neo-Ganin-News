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
use sea_query::Nullable;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use lib_utils::time::Rfc3339;
use sqlx::{types::time::OffsetDateTime, FromRow};

// region:    --- Article Types
#[derive(
	Copy, Clone, Debug, sqlx::Type, derive_more::Display, Deserialize, Serialize, Default,
)]
#[sqlx(type_name = "approval_state")]
#[cfg_attr(test, derive(PartialEq))]
pub enum ApprovalState {
	#[default]
	Draft,
	RequestApproval,
	ApprovalPending,
	Approved,
	NeedCorrection,
	Reject,
}

impl From<ApprovalState> for sea_query::Value {
	fn from(val: ApprovalState) -> Self {
		val.to_string().into()
	}
}

/// Note: This is required for sea::query in case of None.
///       However, in this codebase, we utilize the modql not_none_field,
///       so this will be disregarded anyway.
///       Nonetheless, it's still necessary for compilation.
impl Nullable for ApprovalState {
	fn null() -> sea_query::Value {
		ApprovalState::Draft.into()
	}
}

#[serde_as]
#[derive(Clone, Debug, Fields, FromRow, Serialize)]
pub struct Article {
	// Main field
	pub id: i64,
	pub title: String,
	pub content: String,
	pub category_id: i32,
	pub author_id: i64,
	pub approval_state: ApprovalState,

	// Approval
	pub approver_id: Option<i64>,              // Nullable
	pub approval_time: Option<OffsetDateTime>, // Nullable

	// non mandatory
	pub views: i32,
	pub image_url: Option<String>, // Nullable
	pub likes: i32,

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
pub struct ArticleForCreate {
	pub title: String,
	pub content: String,
	pub category_id: Option<i32>,
	pub author_id: i64,
	#[field(cast_as = "approval_state")]
	pub approval_state: ApprovalState,
	pub image_url: Option<String>,
}

#[derive(Fields, Deserialize, Default)]
pub struct ArticleForUpdate {
	pub title: Option<String>,
	pub content: Option<String>,
	pub category_id: Option<i32>,
	pub views: Option<i32>,
	#[field(cast_as = "approval_state")]
	pub approval_state: ApprovalState,
	pub image_url: Option<String>,
	// article views, likes, reaction, comments itu masuknya article accessories
	pub likes: Option<i32>,
}

#[derive(FilterNodes, Deserialize, Default)]
pub struct ArticleFilter {
	pub id: Option<OpValsInt64>,
	pub title: Option<OpValsString>,
	pub content: Option<OpValsString>,
	pub category_id: Option<OpValsInt32>,
	pub author_id: Option<OpValsInt64>,
	pub approval_state: Option<OpValsString>,
	pub approver_id: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	pub approval_time: Option<OpValsValue>,
}

// endregion: --- Article Types

// region:    --- ArticleBmc

pub struct ArticleBmc;

impl DbBmc for ArticleBmc {
	const TABLE: &'static str = "article";

	// fn has_owner_id() -> bool {
	// 	true
	// }
}
// This will generate the `impl ArticleBmc {...}` with the default CRUD functions.
generate_common_bmc_fns!(
	Bmc: ArticleBmc,
	Entity: Article,
	ForCreate: ArticleForCreate,
	ForUpdate: ArticleForUpdate,
	Filter: ArticleFilter,
);

// endregion: --- ArticleBmc

// region:    --- Tests

#[cfg(test)]
mod tests {
	type Error = Box<dyn std::error::Error>;
	type Result<T> = core::result::Result<T, Error>; // For tests.

	use super::*;
	use crate::_dev_utils::{self, clean_articles, seed_article};
	use crate::model;
	use serde_json::json;

	use serial_test::serial;

	#[serial]
	#[tokio::test]
	async fn test_create_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_title = "test_create_ok article 01";
		let fx_content = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.";
		let fx_author_id = 1000;
		let fx_category_id = Some(1001);
		let fx_approval_state = ApprovalState::Draft;
		let fx_image_url = Some("https://example.com/image.jpg".to_string());

		// -- Exec
		let fx_article_c = ArticleForCreate {
			title: fx_title.to_string(),
			content: fx_content.to_string(),
			category_id: fx_category_id,
			author_id: fx_author_id,
			approval_state: fx_approval_state.clone(),
			image_url: fx_image_url.clone(),
		};
		let article_id = ArticleBmc::create(&ctx, &mm, fx_article_c).await?;

		// -- Check
		let article = ArticleBmc::get(&ctx, &mm, article_id).await?;
		assert_eq!(article.title, fx_title);
		assert_eq!(article.content, fx_content);
		assert_eq!(article.category_id, fx_category_id.unwrap());
		assert_eq!(article.author_id, fx_author_id);
		assert_eq!(article.approval_state, fx_approval_state);
		assert_eq!(article.image_url, fx_image_url);

		// -- Clean
		let count = clean_articles(&ctx, &mm, "test_create_ok").await?;
		assert_eq!(count, 1, "Should have cleaned only 1 article");

		Ok(())
	}

	#[tokio::test]
	#[serial]
	async fn test_update_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();

		let fx_title = "test_update_ok article 01";
		let fx_content = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.";
		let fx_author_id = 1000;
		let fx_approval_state = ApprovalState::Draft;
		let fx_image_url = Some("https://example.com/image.jpg".to_string());

		let fx_article_id = seed_article(
			&ctx,
			&mm,
			fx_title,
			fx_content,
			None,
			fx_author_id,
			fx_approval_state,
			fx_image_url,
		)
		.await?;

		let fx_title_updated = "test_update_ok article 01 - updated";
		let fx_content_updated = "Updated content.";
		let fx_image_url_updated =
			Some("https://example.com/updated_image.jpg".to_string());

		// -- Exec
		let fx_article_u = ArticleForUpdate {
			title: Some(fx_title_updated.to_string()),
			content: Some(fx_content_updated.to_string()),
			image_url: Some(fx_image_url_updated.clone().unwrap().to_string()),
			..Default::default()
		};
		ArticleBmc::update(&ctx, &mm, fx_article_id, fx_article_u).await?;

		// -- Check
		let article = ArticleBmc::get(&ctx, &mm, fx_article_id).await?;
		assert_eq!(article.title, fx_title_updated);
		assert_eq!(article.content, fx_content_updated);
		assert_eq!(article.image_url, fx_image_url_updated);

		// -- Clean
		let count = clean_articles(&ctx, &mm, "test_update_ok").await?;
		assert_eq!(count, 1, "Should have cleaned only 1 article");

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_delete_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();

		let fx_title = "test_delete_ok article 01";
		let fx_content = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.";
		let fx_category_id = Some(1001); // Example category ID
		let fx_author_id = 1002; // Example author ID
		let fx_approval_state = ApprovalState::Draft;
		let fx_image_url = Some("https://example.com/image.jpg".to_string());

		let fx_article_id = seed_article(
			&ctx,
			&mm,
			fx_title,
			fx_content,
			fx_category_id,
			fx_author_id,
			fx_approval_state,
			fx_image_url,
		)
		.await?;

		// -- Exec
		// Check it's there
		ArticleBmc::get(&ctx, &mm, fx_article_id).await?;
		// Delete
		ArticleBmc::delete(&ctx, &mm, fx_article_id).await?;

		// -- Check
		let res = ArticleBmc::get(&ctx, &mm, fx_article_id).await;
		assert!(
			matches!(&res, Err(model::Error::EntityNotFound { .. })),
			"Should return EntityNotFound"
		);

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_first_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();

		let fx_titles = &["test_first_ok article 01", "test_first_ok article 02"];
		let fx_content = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.";
		let fx_category_id = Some(123); // Example category ID
		let fx_author_id = 456; // Example author ID
		let fx_approval_state = ApprovalState::Draft;
		let fx_image_url = Some("https://example.com/image.jpg".to_string());

		for title in fx_titles {
			seed_article(
				&ctx,
				&mm,
				title,
				fx_content,
				fx_category_id,
				fx_author_id,
				fx_approval_state,
				fx_image_url.clone(),
			)
			.await?;
		}

		// -- Exec
		let article_filter: ArticleFilter = serde_json::from_value(json!(
				{
						"title": {"$startsWith": "test_first_ok article"}
				}
		))?;
		let article =
			ArticleBmc::first(&ctx, &mm, Some(vec![article_filter]), None).await?;

		// -- Check
		let article = article.ok_or("No Article Returned (should have returned one")?;
		assert_eq!(article.title, fx_titles[0]);

		// -- Clean
		let count = clean_articles(&ctx, &mm, "test_first_ok article").await?;
		assert_eq!(count, 2, "Should have cleaned 2 articles");

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_list_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();

		let fx_titles = &[
			"test_list_ok article 01",
			"test_list_ok article 02",
			"test_list_ok article 03",
		];
		let fx_content = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.";
		let fx_category_id = Some(123); // Example category ID
		let fx_author_id = 456; // Example author ID
		let fx_approval_state = ApprovalState::Draft;
		let fx_image_url = Some("https://example.com/image.jpg".to_string());

		for title in fx_titles {
			seed_article(
				&ctx,
				&mm,
				title,
				fx_content,
				fx_category_id,
				fx_author_id,
				fx_approval_state,
				fx_image_url.clone(),
			)
			.await?;
		}

		// -- Exec
		let article_filter: ArticleFilter = serde_json::from_value(json!(
				{
						"title": {"$contains": "test_list_ok article"}
				}
		))?;
		let articles =
			ArticleBmc::list(&ctx, &mm, Some(vec![article_filter]), None).await?;

		// -- Check
		assert_eq!(articles.len(), 3);
		let titles = articles.iter().map(|a| &a.title).collect::<Vec<_>>();
		assert_eq!(titles, fx_titles);

		// -- Clean
		let count = clean_articles(&ctx, &mm, "test_list_ok article").await?;
		assert_eq!(count, 3, "Should have cleaned 3 articles");

		Ok(())
	}

	// Add other test cases as needed...
}

// endregion: --- Tests
