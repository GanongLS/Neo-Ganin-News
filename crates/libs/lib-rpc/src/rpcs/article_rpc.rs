use crate::rpcs::prelude::*;
use lib_core::model::article::{
	Article, ArticleBmc, ArticleFilter, ArticleForCreate, ArticleForUpdate,
};

pub fn rpc_router() -> RpcRouter {
	rpc_router!(
		// Same as RpcRouter::new().add...
		create_article,
		get_article,
		list_articles,
		update_article,
		delete_article,
	)
}

generate_common_rpc_fns!(
	Bmc: ArticleBmc,
	Entity: Article,
	ForCreate: ArticleForCreate,
	ForUpdate: ArticleForUpdate,
	Filter: ArticleFilter,
	Suffix: article
);
