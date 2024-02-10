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

// region:    --- Article Types
#[derive(Clone, Debug, sqlx::Type, derive_more::Display, Deserialize, Serialize)]
#[sqlx(type_name = "approval_state")]
pub enum ApprovalState {
	Draft,
	RequestApproval,
	ApprovalPending,
	Approved,
}

impl From<ApprovalState> for sea_query::Value {
	fn from(val: ApprovalState) -> Self {
		val.to_string().into()
	}
}

#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct Article {
	pub id: i64,
	pub title: String,
	pub content: String,
	pub category_id: Option<i32>, // Nullable
	pub author_id: i64,
	pub art_version: i16,
	pub approval_state: ApprovalState,
	pub approver_id: Option<i16>,                 // Nullable
	pub approval_time: Option<OffsetDateTime>,    // Nullable
	pub publication_date: Option<OffsetDateTime>, // Nullable
	pub tags: Option<Vec<String>>,                // Nullable
	pub is_featured: bool,
	pub views: i32,
	pub image_url: Option<String>, // Nullable
	pub likes: i32,
}

#[derive(Deserialize)]
pub struct ArticleForCreate {
	pub title: String,
	pub content: String,
	pub category_id: Option<i32>,
	pub author_id: i64,
	pub art_version: i16,
	pub approval_state: ApprovalState,
	pub approver_id: Option<i16>,
	pub approval_time: Option<OffsetDateTime>,
	pub publication_date: Option<OffsetDateTime>,
	pub tags: Option<Vec<String>>,
	pub is_featured: bool,
	pub views: i32,
	pub image_url: Option<String>,
	pub likes: i32,
}

#[derive(Fields)]
pub struct ArticleForInsert {
	pub title: String,
	pub content: String,
	pub category_id: Option<i32>,
	pub author_id: i64,
	pub art_version: i16,
	pub approval_state: ApprovalState,
	pub approver_id: Option<i16>,
	pub approval_time: Option<OffsetDateTime>,
	pub publication_date: Option<OffsetDateTime>,
	pub tags: Option<Vec<String>>,
	pub is_featured: bool,
	pub views: i32,
	pub image_url: Option<String>,
	pub likes: i32,
}

/// Marker trait
pub trait ArticleBy: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl ArticleBy for Article {}

// Note: Since the entity properties Iden will be given by modql
//       ArticleIden does not have to be exhaustive, but just have the columns
//       we use in our specific code.
#[derive(Iden)]
enum ArticleIden {
	Id,
	Title,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct ArticleFilter {
	pub id: Option<OpValsInt64>,
	pub title: Option<OpValsString>,
	pub content: Option<OpValsString>,
	pub category_id: Option<OpValsInt32>,
	pub author_id: Option<OpValsInt64>,
	pub art_version: Option<OpValsInt16>,
	pub approval_state: Option<OpValsString>,
	pub approver_id: Option<OpValsInt16>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	pub approval_time: Option<OpValsValue>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	pub publication_date: Option<OpValsValue>,
	pub tags: Option<OpValsStringArray>,
	pub is_featured: Option<OpValsBool>,
	pub views: Option<OpValsInt32>,
	pub image_url: Option<OpValsString>,
	pub likes: Option<OpValsInt32>,
}

// endregion: --- Article Types

// region:    --- ArticleBmc
pub struct ArticleBmc;

impl DbBmc for ArticleBmc {
	const TABLE: &'static str = "articles";
}

impl ArticleBmc {
	pub async fn create(
		ctx: &Ctx,
		mm: &ModelManager,
		article_c: ArticleForCreate,
	) -> Result<i64> {
		// Start the transaction
		let mm = mm.new_with_txn()?;
		mm.dbx().begin_txn().await?;

		// Create the article row
		let article_fi = ArticleForInsert { ..article_c };

		let article_id = base::create::<Self, _>(ctx, &mm, article_fi)
			.await
			.map_err(|e| Error::from(e))?;

		// Commit the transaction
		mm.dbx().commit_txn().await?;

		Ok(article_id)
	}

	pub async fn get<E>(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
	where
		E: ArticleBy,
	{
		base::get::<Self, _>(ctx, mm, id).await
	}

	pub async fn list(
		ctx: &Ctx,
		mm: &ModelManager,
		filter: Option<Vec<ArticleFilter>>,
		list_options: Option<ListOptions>,
	) -> Result<Vec<Article>> {
		base::list::<Self, _, _>(ctx, mm, filter, list_options).await
	}

	pub async fn update(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		article: ArticleForCreate,
	) -> Result<()> {
		// Start the transaction
		let mm = mm.new_with_txn()?;
		mm.dbx().begin_txn().await?;

		// Update the article
		let article_fi = ArticleForInsert { ..article };
		base::update::<Self, _>(ctx, &mm, id, article_fi).await?;

		// Commit the transaction
		mm.dbx().commit_txn().await?;

		Ok(())
	}

	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		base::delete::<Self>(ctx, mm, id).await
	}
}
// endregion:    --- ArticleBmc

// region:    --- Article Tests
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
        let fx_title = "Test Article";
        let fx_content = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam sed turpis velit. Sed consequat euismod vestibulum.";

        // -- Exec
        let article_id = ArticleBmc::create(
            &ctx,
            &mm,
            ArticleForCreate {
                title: fx_title.to_string(),
                content: fx_content.to_string(),
                category_id: 1000, // Specify the category_id accordingly
                author_id: 1000,   // Specify the author_id accordingly
                art_version: 1,
                approval_state: ApprovalState::Draft,
                approver_id: None,
                approval_time: None,
                publication_date: None,
                tags: vec!["test".to_string(), "example".to_string()],
                is_featured: false,
                views: 0,
                image_url: None,
                likes: 0,
            },
        )
        .await?;

        // -- Check
        let article: Article = ArticleBmc::get(&ctx, &mm, article_id).await?;
        assert_eq!(article.title, fx_title);
        assert_eq!(article.content, fx_content);

        // -- Clean
        ArticleBmc::delete(&ctx, &mm, article_id).await?;

        Ok(())
    }

    // Add more test cases as needed
}
// endregion: --- Article Tests
