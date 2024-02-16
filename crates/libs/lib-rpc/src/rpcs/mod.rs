use crate::router::RpcRouter;

pub mod agent_rpc;
pub mod article_rpc;
pub mod article_view_rpc;
pub mod author_rpc;
pub mod category_rpc;
pub mod comment_rpc;
pub mod conv_rpc;
mod macro_utils;
mod prelude;

pub fn all_rpc_router() -> RpcRouter {
	RpcRouter::new()
		.extend(agent_rpc::rpc_router())
		.extend(conv_rpc::rpc_router())
		.extend(article_rpc::rpc_router())
		.extend(article_view_rpc::rpc_router())
		.extend(author_rpc::rpc_router())
		.extend(category_rpc::rpc_router())
		.extend(comment_rpc::rpc_router())
}
