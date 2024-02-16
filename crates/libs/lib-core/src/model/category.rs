use crate::ctx::Ctx;
use crate::generate_common_bmc_fns;
use crate::model::{
	base::{self, DbBmc},
	ModelManager, Result,
};
use lib_utils::time::Rfc3339;
use modql::{
	field::Fields,
	filter::{FilterNodes, ListOptions, OpValsString},
};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::FromRow;
use time::OffsetDateTime;

// region:    --- Category Types

#[serde_as]
#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct Category {
	pub id: i64,
	pub name: String,
	pub description: Option<String>,
	pub parent_id: Option<i64>,
	pub is_featured: bool,
	pub creator_id: i64,
	#[serde_as(as = "Rfc3339")]
	pub creation_time: OffsetDateTime,
	pub updater_id: i64,
	#[serde_as(as = "Rfc3339")]
	pub updated_time: OffsetDateTime,
}

#[derive(Fields, Deserialize, Default)]
pub struct CategoryForCreate {
	pub name: String,
	pub description: Option<String>,
	pub parent_id: Option<i64>,
	pub is_featured: bool,
}

#[derive(Fields, Default, Deserialize)]
pub struct CategoryForUpdate {
	pub name: String,
	pub description: Option<String>,
	pub parent_id: Option<i64>,
	pub is_featured: bool,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct CategoryFilter {
	pub name: Option<OpValsString>,
	pub is_featured: Option<bool>,
}

// endregion: --- Category Types

// region:    --- CategoryBmc

pub struct CategoryBmc;

impl DbBmc for CategoryBmc {
	const TABLE: &'static str = "category";
}

// This will generate the `impl CategoryBmc {...}` with the default CRUD functions.
generate_common_bmc_fns!(
		Bmc: CategoryBmc,
		Entity: Category,
		ForCreate: CategoryForCreate,
		ForUpdate: CategoryForUpdate,
		Filter: CategoryFilter,
);

// endregion: --- CategoryBmc

// region:    --- Tests

#[cfg(test)]
mod tests {
	use super::*;
	use crate::_dev_utils::{self, clean_categories, seed_category};
	use serial_test::serial;

	type Error = Box<dyn std::error::Error>;
	type TestResult<T> = std::result::Result<T, Error>; // For tests.

	#[serial]
	#[tokio::test]
	async fn test_create_category_ok() -> TestResult<()> {
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let name = "Test Category";
		let description = "This is a test category".to_string();
		let parent_id = None;
		let is_featured = false;

		let category_c = CategoryForCreate {
			name: name.to_string(),
			description: Some(description.clone()),
			parent_id,
			is_featured,
		};

		let category_id = CategoryBmc::create(&ctx, &mm, category_c).await?;
		let category = CategoryBmc::get(&ctx, &mm, category_id).await?;

		assert_eq!(category.name, name);
		assert_eq!(category.description, Some(description));
		assert_eq!(category.parent_id, parent_id);
		assert_eq!(category.is_featured, is_featured);

		clean_categories(&ctx, &mm, "Test Category").await?;

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_update_category_ok() -> TestResult<()> {
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let name = "Test Category";
		let description = Some("This is a test category");
		let parent_id = None;
		let is_featured = false;

		let category_id =
			seed_category(&ctx, &mm, name, description, parent_id, is_featured).await?;

		let name_updated = "Updated Test Category";
		let description_updated = Some("This is an updated test category".to_string());
		let is_featured_updated = true;

		let category_u = CategoryForUpdate {
			name: name_updated.to_string(),
			description: description_updated.clone(),
			parent_id,
			is_featured: is_featured_updated,
		};

		CategoryBmc::update(&ctx, &mm, category_id, category_u).await?;

		let category = CategoryBmc::get(&ctx, &mm, category_id).await?;

		assert_eq!(category.name, name_updated);
		assert_eq!(category.description, description_updated);
		assert_eq!(category.parent_id, parent_id);
		assert_eq!(category.is_featured, is_featured_updated);

		clean_categories(&ctx, &mm, "Updated Test Category").await?;

		Ok(())
	}
}

// endregion: --- Tests
