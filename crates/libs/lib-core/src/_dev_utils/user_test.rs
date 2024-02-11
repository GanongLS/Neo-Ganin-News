use crate::ctx::Ctx;
use crate::model::{self, ModelManager};
use modql::filter::OpValString;

// region:    --- User seed/clean

pub async fn seed_users(
	ctx: &Ctx,
	mm: &ModelManager,
	usernames: &[&str],
) -> model::Result<Vec<i64>> {
	let mut ids = Vec::new();

	for name in usernames {
		let id = seed_user(ctx, mm, name).await?;
		ids.push(id);
	}

	Ok(ids)
}

pub async fn seed_user(
	ctx: &Ctx,
	mm: &ModelManager,
	username: &str,
) -> model::Result<i64> {
	let pwd_clear = "seed-user-pwd";
	let email = "seed_user@example.com";
	let first_name = "Seed";
	let last_name = "User";

	let id = model::user::UserBmc::create(
		ctx,
		mm,
		model::user::UserForCreate {
			username: username.to_string(),
			pwd_clear: pwd_clear.to_string(),
			email: email.to_string(),
			first_name: first_name.to_string(),
			last_name: last_name.to_string(),
		},
	)
	.await?;

	Ok(id)
}

pub async fn clean_users(
	ctx: &Ctx,
	mm: &ModelManager,
	contains_username: &str,
) -> model::Result<usize> {
	let users = model::user::UserBmc::list(
		ctx,
		mm,
		Some(vec![model::user::UserFilter {
			username: Some(OpValString::Contains(contains_username.to_string()).into()),
			..Default::default()
		}]),
		None,
	)
	.await?;
	let count = users.len();

	for user in users {
		model::user::UserBmc::delete(ctx, mm, user.id).await?;
	}

	Ok(count)
}

// endregion: --- User seed/clean
