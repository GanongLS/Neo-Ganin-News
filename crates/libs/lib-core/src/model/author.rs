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

// region:    --- Author Types
#[derive(Clone, Debug, sqlx::Type, derive_more::Display, Deserialize, Serialize)]
#[sqlx(type_name = "AUTHOR_TYPE")]
pub enum AuthorType {
	Journalist,
	Editor,
	Columnist,
	CopyWriter,
	ContentWriter,
	GhostWriter,
}
impl From<AuthorType> for sea_query::Value {
	fn from(val: AuthorType) -> Self {
		val.to_string().into()
	}
}

#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct Author {
	pub user_id: i64,
	pub typ: AuthorType,
	pub full_name: Option<String>,
	pub pen_name: String,
	pub bio: Option<String>,
	pub website: Option<String>,
	pub avatar_url: Option<String>,
}

#[derive(Deserialize)]
pub struct AuthorForCreate {
	pub user_id: i64,
	pub pen_name: String,
	pub full_name: Option<String>,
	pub bio: Option<String>,
	pub website: Option<String>,
	pub avatar_url: Option<String>,
}

#[derive(Fields)]
pub struct AuthorForInsert {
	pub user_id: i64,
	pub pen_name: String,
}

#[derive(Clone, FromRow, Fields, Debug)]
pub struct AuthorForUpdate {
	pub user_id: i64,
	pub full_name: Option<String>,
	pub pen_name: String,
	pub bio: Option<String>,
	pub website: Option<String>,
	pub avatar_url: Option<String>,
}

/// Marker trait
pub trait AuthorBy: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl AuthorBy for Author {}
impl AuthorBy for AuthorForCreate {}
impl AuthorBy for AuthorForCreate {}

// Note: Since the entity properties Iden will be given by modql
//       AuthorIden does not have to be exhaustive, but just have the columns
//       we use in our specific code.
#[derive(Iden)]
enum AuthorIden {
	UserId,
	Typ,
	FullName,
	PenName,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct AuthorFilter {
	pub user_id: Option<OpValsInt64>,
	pub typ: Option<OpValsString>,
	pub full_name: Option<OpValsString>,
	pub pen_name: Option<OpValsString>,
}
// endregion: --- Author Types

// region:    --- AuthorBmc

pub struct AuthorBmc;

impl DbBmc for AuthorBmc {
	const TABLE: &'static str = "author";
}

impl AuthorBmc {
	pub async fn create(
		ctx: &Ctx,
		mm: &ModelManager,
		author_c: AuthorForCreate,
	) -> Result<u64> {
		let AuthorForCreate {
			user_id,
			pen_name,
			full_name,
			bio,
			website,
			avatar_url,
		} = author_c;

		// -- Create the author row
		let author_fi = AuthorForInsert {
			user_id,
			pen_name: pen_name.to_string(),
		};

		// Start the transaction
		let mm = mm.new_with_txn()?;

		mm.dbx().begin_txn().await?;

		let author_id = base::create::<Self, _>(ctx, &mm, author_fi).await?;

		// Commit the transaction
		mm.dbx().commit_txn().await?;

		Ok(author_id)
	}

	pub async fn get<E>(ctx: &Ctx, mm: &ModelManager, user_id: i64) -> Result<E>
	where
		E: AuthorBy,
	{
		base::get::<Self, _>(ctx, mm, user_id).await
	}

	pub async fn first_by_user_id<E>(
		_ctx: &Ctx,
		mm: &ModelManager,
		user_id: i64,
	) -> Result<Option<E>>
	where
		E: AuthorBy,
	{
		// -- Build query
		let mut query = Query::select();
		query
			.from(Self::table_ref())
			.columns(E::field_idens())
			.and_where(Expr::col(AuthorIden::UserId).eq(user_id));

		// -- Execute query
		let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

		let sqlx_query = sqlx::query_as_with::<_, E, _>(&sql, values);
		let entity = mm.dbx().fetch_optional(sqlx_query).await?;

		Ok(entity)
	}

	pub async fn list(
		ctx: &Ctx,
		mm: &ModelManager,
		filter: Option<Vec<AuthorFilter>>,
		list_options: Option<ListOptions>,
	) -> Result<Vec<Author>> {
		base::list::<Self, _, _>(ctx, mm, filter, list_options).await
	}
}
