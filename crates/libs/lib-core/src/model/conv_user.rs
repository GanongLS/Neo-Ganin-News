use crate::model::base::DbBmc;
use lib_utils::time::Rfc3339;
use modql::field::Fields;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::FromRow;
use time::OffsetDateTime;

// region:    --- Types

#[serde_as]
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct ConvUser {
	pub id: i64,

	// -- FK
	pub conv_id: i64,
	pub user_id: i64,

	// -- Timestamps
	// creator user_id and time
	pub creator_id: i64,
	#[serde_as(as = "Rfc3339")]
	pub creation_time: OffsetDateTime,
	// last modifier user_id and time
	pub updater_id: i64,
	#[serde_as(as = "Rfc3339")]
	pub updated_time: OffsetDateTime,
}

#[derive(Fields, Deserialize)]
pub struct ConvUserForCreate {
	pub conv_id: i64,
	pub user_id: i64,
}

// endregion: --- Types

// region:    --- ConvUser

pub struct ConvUserBmc;

impl DbBmc for ConvUserBmc {
	const TABLE: &'static str = "conv_user";
}

// Note: This is not implemented yet. It will likely be similar to `ConvMsg`, meaning it will be
//       managed by the `ConvBmc` container entity.

// endregion: --- ConvUser
