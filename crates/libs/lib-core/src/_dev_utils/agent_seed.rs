use crate::ctx::Ctx;
use crate::model::agent::{AgentBmc, AgentFilter, AgentForCreate};
use crate::model::{self, ModelManager};

use modql::filter::OpValString;

// region:    --- Agent seed/clean

pub async fn seed_agents(
	ctx: &Ctx,
	mm: &ModelManager,
	names: &[&str],
) -> model::Result<Vec<i64>> {
	let mut ids = Vec::new();

	for name in names {
		let id = seed_agent(ctx, mm, name).await?;
		ids.push(id);
	}

	Ok(ids)
}

pub async fn seed_agent(
	ctx: &Ctx,
	mm: &ModelManager,
	name: &str,
) -> model::Result<i64> {
	AgentBmc::create(
		ctx,
		mm,
		AgentForCreate {
			name: name.to_string(),
		},
	)
	.await
}

/// Delete all agents that have their title contains contains_name
pub async fn clean_agents(
	ctx: &Ctx,
	mm: &ModelManager,
	contains_name: &str,
) -> model::Result<usize> {
	let agents = AgentBmc::list(
		ctx,
		mm,
		Some(vec![AgentFilter {
			name: Some(OpValString::Contains(contains_name.to_string()).into()),
			..Default::default()
		}]),
		None,
	)
	.await?;
	let count = agents.len();

	for agent in agents {
		AgentBmc::delete(ctx, mm, agent.id).await?;
	}

	Ok(count)
}

// endregion: --- Agent seed/clean
