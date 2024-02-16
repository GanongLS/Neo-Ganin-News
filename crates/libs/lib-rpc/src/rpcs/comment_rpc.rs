use crate::rpcs::prelude::*;
use lib_core::model::comment::{
    Comment, CommentBmc, CommentFilter, CommentForCreate, CommentForUpdate,
};

pub fn rpc_router() -> RpcRouter {
    rpc_router!(
        create_comment,
        get_comment,
        list_comments,
        update_comment,
        delete_comment,
    )
}

generate_common_rpc_fns!(
    Bmc: CommentBmc,
    Entity: Comment,
    ForCreate: CommentForCreate,
    ForUpdate: CommentForUpdate,
    Filter: CommentFilter,
    Suffix: comment
);
