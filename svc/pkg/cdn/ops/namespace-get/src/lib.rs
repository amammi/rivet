use proto::backend::{self, pkg::*};
use rivet_operation::prelude::*;

#[derive(sqlx::FromRow)]
struct GameNamespace {
	namespace_id: Uuid,
	enable_domain_public_auth: bool,
	auth_type: i64,
}

#[derive(sqlx::FromRow)]
struct GameNamespaceDomain {
	namespace_id: Uuid,
	domain: String,
	create_ts: i64,
}

#[derive(sqlx::FromRow)]
struct GameNamespaceAuthUser {
	namespace_id: Uuid,
	user_name: String,
	password: String,
}

#[operation(name = "cdn-namespace-get")]
async fn handle(
	ctx: OperationContext<cdn::namespace_get::Request>,
) -> GlobalResult<cdn::namespace_get::Response> {
	let namespace_ids = ctx
		.namespace_ids
		.iter()
		.map(common::Uuid::as_uuid)
		.collect::<Vec<_>>();

	let sql_pool = ctx.crdb("db-cdn").await?;
	let (namespace_domains, auth_users, namespaces) = tokio::try_join!(
		sqlx::query_as::<_, GameNamespaceDomain>(indoc!(
			"
			SELECT namespace_id, domain, create_ts
			FROM game_namespace_domains
			WHERE namespace_id = ANY($1)
			"
		))
		.bind(&namespace_ids)
		.fetch_all(&sql_pool),
		sqlx::query_as::<_, GameNamespaceAuthUser>(indoc!(
			"
			SELECT namespace_id, user_name, password
			FROM game_namespace_auth_users
			WHERE namespace_id = ANY($1)
			"
		))
		.bind(&namespace_ids)
		.fetch_all(&sql_pool),
		sqlx::query_as::<_, GameNamespace>(indoc!(
			"
			SELECT namespace_id, enable_domain_public_auth, auth_type
			FROM game_namespaces
			WHERE namespace_id = ANY($1)
			"
		))
		.bind(&namespace_ids)
		.fetch_all(&sql_pool),
	)?;

	let namespace_proto = namespaces
		.into_iter()
		.map(|ns| {
			Ok(cdn::namespace_get::response::Namespace {
				namespace_id: Some(ns.namespace_id.into()),
				config: Some(backend::cdn::NamespaceConfig {
					enable_domain_public_auth: ns.enable_domain_public_auth,
					domains: namespace_domains
						.iter()
						.filter(|d| d.namespace_id == ns.namespace_id)
						.map(|domain| backend::cdn::namespace_config::Domain {
							domain: domain.domain.clone(),
							create_ts: domain.create_ts,
						})
						.collect(),
					auth_type: internal_unwrap_owned!(
						backend::cdn::namespace_config::AuthType::from_i32(ns.auth_type as i32),
						"unknown cdn auth type"
					) as i32,
					auth_user_list: auth_users
						.iter()
						.filter(|d| d.namespace_id == ns.namespace_id)
						.map(|user| backend::cdn::namespace_config::AuthUser {
							user: user.user_name.clone(),
							password: user.password.clone(),
						})
						.collect(),
				}),
			})
		})
		.collect::<GlobalResult<Vec<_>>>()?;

	Ok(cdn::namespace_get::Response {
		namespaces: namespace_proto,
	})
}
