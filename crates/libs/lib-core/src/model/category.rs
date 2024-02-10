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

// region:    --- Category Types
#[derive(Clone, Debug, sqlx::Type, derive_more::Display, Deserialize, Serialize)]
#[sqlx(type_name = "category_typ")]
pub enum CategoryTyp {
	General,
	Featured,
}

impl From<CategoryTyp> for sea_query::Value {
	fn from(val: CategoryTyp) -> Self {
		val.to_string().into()
	}
}

#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct Category {
	pub id: i32,
	pub name: String,
	pub description: Option<String>,
	pub parent_id: Option<i32>,
	pub is_featured: bool,
	pub creator_id: i64,
	#[serde_as(as = "Rfc3339")]
	pub creation_time: OffsetDateTime,
	pub updater_id: i64,
	#[serde_as(as = "Rfc3339")]
	pub updated_time: OffsetDateTime,
}

#[derive(Deserialize)]
pub struct CategoryForCreate {
	pub name: String,
	pub description: Option<String>,
	pub parent_id: Option<i32>,
	pub is_featured: bool,
}

#[derive(Fields)]
pub struct CategoryForInsert {
	pub name: String,
	pub description: Option<String>,
	pub parent_id: Option<i32>,
	pub is_featured: bool,
}

/// Marker trait
pub trait CategoryBy: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl CategoryBy for Category {}

// Note: Since the entity properties Iden will be given by modql
//       CategoryIden does not have to be exhaustive, but just have the columns
//       we use in our specific code.
#[derive(Iden)]
enum CategoryIden {
	Id,
	Name,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct CategoryFilter {
	pub id: Option<OpValsInt32>,
	pub name: Option<OpValsString>,
	pub description: Option<OpValsString>,
	pub parent_id: Option<OpValsInt32>,
	pub is_featured: Option<OpValsBool>,
	pub creator_id: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	pub creation_time: Option<OpValsValue>,
	pub updater_id: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	pub updated_time: Option<OpValsValue>,
}

// endregion: --- Category Types

// region:    --- CategoryBmc

pub struct CategoryBmc;

impl DbBmc for CategoryBmc {
	const TABLE: &'static str = "categories";
}

impl CategoryBmc {
	pub async fn create(
		ctx: &Ctx,
		mm: &ModelManager,
		category_c: CategoryForCreate,
	) -> Result<i32> {
		let CategoryForCreate {
			name,
			description,
			parent_id,
			is_featured,
		} = category_c;

		// -- Create the category row
		let category_fi = CategoryForInsert {
			name: name.to_string(),
			description,
			parent_id,
			is_featured,
		};

		// Start the transaction
		let mm = mm.new_with_txn()?;

		mm.dbx().begin_txn().await?;

		let category_id = base::create::<Self, _>(ctx, &mm, category_fi)
			.await
			.map_err(|model_error| {
				Error::resolve_unique_violation(
					model_error,
					None, // No unique constraint handling for category creation
				)
			})?;

		// Commit the transaction
		mm.dbx().commit_txn().await?;

		Ok(category_id)
	}

	pub async fn get<E>(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<E>
	where
		E: CategoryBy,
	{
		base::get::<Self, _>(ctx, mm, id).await
	}

	pub async fn first_by_name<E>(
		_ctx: &Ctx,
		mm: &ModelManager,
		name: &str,
	) -> Result<Option<E>>
	where
		E: CategoryBy,
	{
		// -- Build query
		let mut query = Query::select();
		query
			.from(Self::table_ref())
			.columns(E::field_idens())
			.and_where(Expr::col(CategoryIden::Name).eq(name));

		// -- Execute query
		let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

		let sqlx_query = sqlx::query_as_with::<_, E, _>(&sql, values);
		let entity = mm.dbx().fetch_optional(sqlx_query).await?;

		Ok(entity)
	}

	pub async fn list(
		ctx: &Ctx,
		mm: &ModelManager,
		filter: Option<Vec<CategoryFilter>>,
		list_options: Option<ListOptions>,
	) -> Result<Vec<Category>> {
		base::list::<Self, _, _>(ctx, mm, filter, list_options).await
	}

	pub async fn update(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i32,
		category_update: CategoryForInsert,
	) -> Result<()> {
		// -- Build query
		let mut query = Query::update();
		query
			.table(Self::table_ref())
			.values(category_update)
			.and_where(Expr::col(CategoryIden::Id).eq(id));

		// -- Exec query
		let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
		let sqlx_query = sqlx::query_with(&sql, values);
		let _count = mm.dbx().execute(sqlx_query).await?;

		Ok(())
	}

	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<()> {
		base::delete::<Self>(ctx, mm, id).await
	}
}

// endregion: --- CategoryBmc

// region:    --- Tests

#[cfg(test)]
mod tests {
	use super::*;
	use crate::_dev_utils;
	use serial_test::serial;

	#[serial]
	#[tokio::test]
	async fn test_create_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_name = "test_category_create_ok";
		let fx_description = "Test category description";
		let fx_parent_id = None;
		let fx_is_featured = false;

		// -- Exec
		let category_id = CategoryBmc::create(
			&ctx,
			&mm,
			CategoryForCreate {
				name: fx_name.to_string(),
				description: Some(fx_description.to_string()),
				parent_id: fx_parent_id,
				is_featured: fx_is_featured,
			},
		)
		.await?;

		// -- Check
		let category: Category = CategoryBmc::get(&ctx, &mm, category_id).await?;
		assert_eq!(category.name, fx_name);
		assert_eq!(category.description, Some(fx_description.to_string()));
		assert_eq!(category.parent_id, fx_parent_id);
		assert_eq!(category.is_featured, fx_is_featured);

		// -- Clean
		CategoryBmc::delete(&ctx, &mm, category_id).await?;

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_get_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_name = "test_category_get_ok";
		let fx_description = "Test category description";
		let fx_parent_id = None;
		let fx_is_featured = true;

		// -- Create category
		let category_id = CategoryBmc::create(
			&ctx,
			&mm,
			CategoryForCreate {
				name: fx_name.to_string(),
				description: Some(fx_description.to_string()),
				parent_id: fx_parent_id,
				is_featured: fx_is_featured,
			},
		)
		.await?;

		// -- Exec
		let category: Category = CategoryBmc::get(&ctx, &mm, category_id).await?;

		// -- Check
		assert_eq!(category.name, fx_name);
		assert_eq!(category.description, Some(fx_description.to_string()));
		assert_eq!(category.parent_id, fx_parent_id);
		assert_eq!(category.is_featured, fx_is_featured);

		// -- Clean
		CategoryBmc::delete(&ctx, &mm, category_id).await?;

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_list_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();

		// -- Create categories
		let category_ids: Vec<i32> = vec![
			CategoryBmc::create(
				&ctx,
				&mm,
				CategoryForCreate {
					name: "test_category_list_ok_1".to_string(),
					description: None,
					parent_id: None,
					is_featured: false,
				},
			)
			.await?,
			CategoryBmc::create(
				&ctx,
				&mm,
				CategoryForCreate {
					name: "test_category_list_ok_2".to_string(),
					description: None,
					parent_id: None,
					is_featured: false,
				},
			)
			.await?,
		];

		// -- Exec
		let categories: Vec<Category> = CategoryBmc::list(&ctx, &mm, None, None).await?;

		// -- Check
		assert_eq!(categories.len(), 2);
		assert!(category_ids.contains(&categories[0].id));
		assert!(category_ids.contains(&categories[1].id));

		// -- Clean
		for &category_id in category_ids.iter() {
			CategoryBmc::delete(&ctx, &mm, category_id).await?;
		}

		Ok(())
	}
}

// endregion: --- Tests
