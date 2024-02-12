use modql::filter::OpValString;

use crate::ctx::Ctx;
use crate::model::author::{AuthorBmc, AuthorFilter, AuthorForCreate, AuthorType};
use crate::model::{self, ModelManager};

// region:    --- Author seed/clean

pub async fn seed_authors(
	ctx: &Ctx,
	mm: &ModelManager,
	data: &[(
		i64,
		&str,
		AuthorType,
		Option<String>,
		Option<String>,
		Option<String>,
	)],
) -> model::Result<Vec<i64>> {
	let mut ids = Vec::new();

	for (user_id, pen_name, author_type, bio, website, avatar_url) in data {
		let id = seed_author(
			ctx,
			mm,
			*user_id,
			*pen_name,
			author_type.clone(),
			bio.clone(),
			website.clone(),
			avatar_url.clone(),
		)
		.await?;
		ids.push(id);
	}

	Ok(ids)
}

pub async fn seed_author(
	ctx: &Ctx,
	mm: &ModelManager,
	user_id: i64,
	pen_name: &str,
	author_type: AuthorType,
	bio: Option<String>,
	website: Option<String>,
	avatar_url: Option<String>,
) -> model::Result<i64> {
	AuthorBmc::create(
		ctx,
		mm,
		AuthorForCreate {
			user_id,
			pen_name: pen_name.to_string(),
			author_type,
			bio,
			website,
			avatar_url,
		},
	)
	.await
}

pub async fn clean_authors(
	ctx: &Ctx,
	mm: &ModelManager,
	contains_pen_name: &str,
) -> model::Result<usize> {
	let authors = AuthorBmc::list(
		ctx,
		mm,
		Some(vec![AuthorFilter {
			pen_name: Some(OpValString::Contains(contains_pen_name.to_string()).into()),
			..Default::default()
		}]),
		None,
	)
	.await?;
	let count = authors.len();

	for author in authors {
		AuthorBmc::delete(ctx, mm, author.id).await?;
	}

	Ok(count)
}

// endregion: --- Author seed/clean
