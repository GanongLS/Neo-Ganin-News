use crate::rpcs::prelude::*;
use lib_core::model::subscription::{
    Subscription, SubscriptionBmc, SubscriptionFilter, SubscriptionForCreate, SubscriptionForUpdate,
};

pub fn rpc_router() -> RpcRouter {
    rpc_router!(
        // Same as RpcRouter::new().add...
        create_subscription,
        get_subscription,
        list_subscriptions,
        update_subscription,
        delete_subscription,
    )
}

generate_common_rpc_fns!(
    Bmc: SubscriptionBmc,
    Entity: Subscription,
    ForCreate: SubscriptionForCreate,
    ForUpdate: SubscriptionForUpdate,
    Filter: SubscriptionFilter,
    Suffix: subscription
);
