use crate::ctx::Ctx;
use crate::generate_common_bmc_fns;
use crate::model::{
	base::{self, DbBmc},
	ModelManager, Result,
};
use lib_utils::time::Rfc3339;
use modql::{
	field::Fields,
	filter::{FilterNodes, ListOptions, OpValsInt64, OpValsString},
};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::FromRow;
use time::OffsetDateTime;

// region:    --- Author Types
#[derive(
	Clone, Debug, sqlx::Type, derive_more::Display, Deserialize, Serialize, Default,
)]
#[sqlx(type_name = "AUTHOR_TYPE")]
#[cfg_attr(test, derive(PartialEq))]
pub enum AuthorType {
	Editor,
	Journalist,
	Columnist,
	CopyWriter,
	ContentWriter,
	#[default]
	GhostWriter,
}
impl From<AuthorType> for sea_query::Value {
	fn from(val: AuthorType) -> Self {
		val.to_string().into()
	}
}

#[serde_as]
#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct Author {
	pub id: i64,
	pub user_id: i64,
	pub author_type: AuthorType,
	pub pen_name: String,

	// non mandatory
	pub bio: Option<String>,
	pub website: Option<String>,
	pub avatar_url: Option<String>,

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
pub struct AuthorForCreate {
	pub user_id: i64,
	#[field(cast_as = "AUTHOR_TYPE")]
	pub author_type: AuthorType,
	pub pen_name: String,
	pub bio: Option<String>,
	pub website: Option<String>,
	pub avatar_url: Option<String>,
}

#[derive(Clone, Fields, Default)]
pub struct AuthorForUpdate {
	pub user_id: i64,
	#[field(cast_as = "AUTHOR_TYPE")]
	pub author_type: AuthorType,
	pub pen_name: String,
	pub bio: Option<String>,
	pub website: Option<String>,
	pub avatar_url: Option<String>,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct AuthorFilter {
	pub user_id: Option<OpValsInt64>,
	pub author_type: Option<OpValsString>,
	pub pen_name: Option<OpValsString>,
}
// endregion: --- Author Types

// region:    --- AuthorBmc

pub struct AuthorBmc;

impl DbBmc for AuthorBmc {
	const TABLE: &'static str = "author";

	// fn has_owner_id() -> bool {
	// 	true
	// }
}
// This will generate the `impl AuthorBmc {...}` with the default CRUD functions.
generate_common_bmc_fns!(
	Bmc: AuthorBmc,
	Entity: Author,
	ForCreate: AuthorForCreate,
	ForUpdate: AuthorForUpdate,
	Filter: AuthorFilter,
);

// endregion: --- AuthorBmc

// region:    --- Tests

#[cfg(test)]
mod tests {
	type Error = Box<dyn std::error::Error>;
	type Result<T> = core::result::Result<T, Error>; // For tests.

	use super::*;
	use crate::_dev_utils::{self, clean_authors, seed_author};

	use serial_test::serial;

	#[serial]
	#[tokio::test]
	async fn test_create_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_pen_name = "test_create_ok author 01";
		let fx_user_id = 1003;
		let fx_author_type = AuthorType::Editor;
		let fx_bio = Some("Test bio".to_string());
		let fx_website = Some("https://example.com".to_string());
		let fx_avatar_url = Some("https://example.com/avatar.jpg".to_string());

		// -- Exec
		let fx_author_c = AuthorForCreate {
			user_id: fx_user_id,
			author_type: fx_author_type.clone(),
			pen_name: fx_pen_name.to_string(),
			bio: fx_bio.clone(),
			website: fx_website.clone(),
			avatar_url: fx_avatar_url.clone(),
		};
		let author_id = AuthorBmc::create(&ctx, &mm, fx_author_c).await?;

		// -- Check
		let author = AuthorBmc::get(&ctx, &mm, author_id).await?;
		assert_eq!(author.pen_name, fx_pen_name);
		assert_eq!(author.user_id, fx_user_id);
		assert_eq!(author.author_type, fx_author_type);
		assert_eq!(author.bio, fx_bio);
		assert_eq!(author.website, fx_website);
		assert_eq!(author.avatar_url, fx_avatar_url);

		// -- Clean
		let count = clean_authors(&ctx, &mm, "test_create_ok").await?;
		assert_eq!(count, 1, "Should have cleaned only 1 author");

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_update_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();

		let fx_user_id = 1004;
		let fx_pen_name = "test_update_ok author 01";
		let fx_author_type = AuthorType::Editor;
		let fx_bio = Some("Original Bio".to_string());
		let fx_website = Some("https://example.com".to_string());
		let fx_avatar_url = Some("https://example.com/avatar.jpg".to_string());

		let fx_author_id = seed_author(
			&ctx,
			&mm,
			fx_user_id,
			fx_pen_name,
			fx_author_type,
			fx_bio.clone(),
			fx_website.clone(),
			fx_avatar_url.clone(),
		)
		.await?;

		let fx_pen_name_updated = "test_update_ok author 01 - updated";
		let fx_bio_updated = Some("Updated Bio".to_string());
		let fx_website_updated = Some("https://updated-example.com".to_string());
		let fx_avatar_url_updated =
			Some("https://updated-example.com/avatar.jpg".to_string());

		// -- Exec
		let fx_author_u = AuthorForUpdate {
			pen_name: fx_pen_name_updated.to_string(),
			bio: fx_bio_updated.clone(),
			website: fx_website_updated.clone(),
			avatar_url: fx_avatar_url_updated.clone(),
			..Default::default()
		};
		AuthorBmc::update(&ctx, &mm, fx_author_id, fx_author_u).await?;

		// -- Check
		let author = AuthorBmc::get(&ctx, &mm, fx_author_id).await?;
		assert_eq!(author.pen_name, fx_pen_name_updated);
		assert_eq!(author.bio, fx_bio_updated);
		assert_eq!(author.website, fx_website_updated);
		assert_eq!(author.avatar_url, fx_avatar_url_updated);

		// -- Clean
		let count = clean_authors(&ctx, &mm, "test_update_ok author").await?;
		assert_eq!(count, 1, "Should have cleaned only 1 author");

		Ok(())
	}

	// Add other test cases as needed...
}

// endregion: --- Tests

