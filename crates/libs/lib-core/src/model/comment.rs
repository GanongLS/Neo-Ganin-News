use crate::ctx::Ctx;
use crate::model::base::{self, prep_fields_for_update, DbBmc};
use crate::model::modql_utils::time_to_sea_value;
use crate::model::ModelManager;
use crate::model::{Error, Result};
use lib_auth::pwd::{self, ContentToHash};
use modql::field::{Field, Fields, HasFields};
use modql::filter::{FilterNodes, ListOptions, OpValsInt64, OpValsString, OpValsValue};
use sea_query::{Expr, Iden, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::FromRow;
use uuid::Uuid;

// region:    --- Comment Types
#[derive(Clone, Debug, sqlx::Type, derive_more::Display, Deserialize, Serialize)]
#[sqlx(type_name = "comment_typ")]
pub enum CommentTyp {
	General,
	Reply,
}

impl From<CommentTyp> for sea_query::Value {
	fn from(val: CommentTyp) -> Self {
		val.to_string().into()
	}
}

#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct Comment {
	pub id: i32,
	pub article_id: i32,
	pub user_id: i64,
	pub content: String,
	pub typ: CommentTyp,
}

#[derive(Deserialize)]
pub struct CommentForCreate {
	pub article_id: i32,
	pub user_id: i64,
	pub content: String,
}

#[derive(Fields)]
pub struct CommentForInsert {
	pub article_id: i32,
	pub user_id: i64,
	pub content: String,
}

#[derive(Clone, FromRow, Fields, Debug)]
pub struct CommentForUpdate {
	pub article_id: i32,
	pub user_id: i64,
	pub content: String,
}

/// Marker trait
pub trait CommentBy: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl CommentBy for Comment {}

// Note: Since the entity properties Iden will be given by modql
//       CommentIden does not have to be exhaustive, but just have the columns
//       we use in our specific code.
#[derive(Iden)]
enum CommentIden {
	Id,
	ArticleId,
	UserId,
	Content,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct CommentFilter {
	pub id: Option<OpValsInt32>,
	pub article_id: Option<OpValsInt32>,
	pub user_id: Option<OpValsInt64>,
	pub content: Option<OpValsString>,
}
// endregion: --- Comment Types

// region:    --- CommentBmc
pub struct CommentBmc;

impl DbBmc for CommentBmc {
	const TABLE: &'static str = "comments";
}

impl CommentBmc {
	pub async fn create(
		ctx: &Ctx,
		mm: &ModelManager,
		comment_c: CommentForCreate,
	) -> Result<i64> {
		// Start the transaction
		let mm = mm.new_with_txn()?;
		mm.dbx().begin_txn().await?;

		// Create the comment row
		let comment_fi = CommentForInsert { ..comment_c };

		let comment_id = base::create::<Self, _>(ctx, &mm, comment_fi)
			.await
			.map_err(|e| Error::from(e))?;

		// Commit the transaction
		mm.dbx().commit_txn().await?;

		Ok(comment_id)
	}

	pub async fn get<E>(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
	where
		E: CommentBy,
	{
		base::get::<Self, _>(ctx, mm, id).await
	}

	pub async fn list(
		ctx: &Ctx,
		mm: &ModelManager,
		filter: Option<Vec<CommentFilter>>,
		list_options: Option<ListOptions>,
	) -> Result<Vec<Comment>> {
		base::list::<Self, _, _>(ctx, mm, filter, list_options).await
	}

	pub async fn update(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		comment: CommentForCreate,
	) -> Result<()> {
		// Start the transaction
		let mm = mm.new_with_txn()?;
		mm.dbx().begin_txn().await?;

		// Update the comment
		let comment_fi = CommentForInsert { ..comment };
		base::update::<Self, _>(ctx, &mm, id, comment_fi).await?;

		// Commit the transaction
		mm.dbx().commit_txn().await?;

		Ok(())
	}

	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		base::delete::<Self>(ctx, mm, id).await
	}
}
// endregion:    --- CommentBmc

// region:    --- Comment Tests
#[cfg(test)]
mod tests {
	pub type Result<T> = core::result::Result<T, Error>;
	pub type Error = Box<dyn std::error::Error>; // For tests.

	use super::*;
	use crate::_dev_utils;
	use serial_test::serial;

	#[serial]
	#[tokio::test]
	async fn test_create_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_article_id = 1000; // Specify the article_id accordingly
		let fx_user_id = 1000; // Specify the user_id accordingly
		let fx_content = "Test comment content";

		// -- Exec
		let comment_id = CommentBmc::create(
			&ctx,
			&mm,
			CommentForCreate {
				article_id: fx_article_id,
				user_id: fx_user_id,
				content: fx_content.to_string(),
			},
		)
		.await?;

		// -- Check
		let comment: Comment = CommentBmc::get(&ctx, &mm, comment_id).await?;
		assert_eq!(comment.article_id, fx_article_id);
		assert_eq!(comment.user_id, fx_user_id);
		assert_eq!(comment.content, fx_content);

		// -- Clean
		CommentBmc::delete(&ctx, &mm, comment_id).await?;

		Ok(())
	}

	// Add more test cases as needed
}
// endregion: --- Comment Tests
