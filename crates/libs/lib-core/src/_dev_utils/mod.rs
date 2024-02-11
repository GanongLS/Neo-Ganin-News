// region:    --- Modules

mod agent_test;
mod article_test;
mod conv_test;
mod dev_db;
mod user_test;

use crate::model::ModelManager;

use tokio::sync::OnceCell;
use tracing::info;

// exporting
pub use crate::_dev_utils::agent_test::*;
pub use crate::_dev_utils::article_test::*;
pub use crate::_dev_utils::conv_test::*;
pub use crate::_dev_utils::user_test::*;

// endregion: --- Modules

/// Initialize environment for local development.
/// (for early development, will be called from main()).
pub async fn init_dev() {
	static INIT: OnceCell<()> = OnceCell::const_new();

	INIT
		.get_or_init(|| async {
			info!("{:<12} - init_dev_all()", "FOR-DEV-ONLY");

			dev_db::init_dev_db().await.unwrap();
		})
		.await;
}

/// Initialize test environment.
pub async fn init_test() -> ModelManager {
	static INIT: OnceCell<ModelManager> = OnceCell::const_new();

	let mm = INIT
		.get_or_init(|| async {
			init_dev().await;
			// NOTE: Rare occasion where unwrap is kind of ok.
			ModelManager::new().await.unwrap()
		})
		.await;

	mm.clone()
}
