use crate::rpcs::prelude::*;
use lib_core::model::article_view::{
	ArticleView, ArticleViewBmc, ArticleViewFilter, ArticleViewForCreate, ArticleViewForUpdate,
};

pub fn rpc_router() -> RpcRouter {
	rpc_router!(
		// Same as RpcRouter::new().add...
		create_article_view,
		get_article_view,
		list_article_views,
		update_article_view,
		delete_article_view,
	)
}

generate_common_rpc_fns!(
	Bmc: ArticleViewBmc,
	Entity: ArticleView,
	ForCreate: ArticleViewForCreate,
	ForUpdate: ArticleViewForUpdate,
	Filter: ArticleViewFilter,
	Suffix: article_view
);
