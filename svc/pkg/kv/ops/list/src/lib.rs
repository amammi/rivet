use proto::backend::pkg::*;
use rivet_operation::prelude::*;

const DEFAULT_LIMIT: i64 = 32;

#[operation(name = "kv-list")]
async fn handle(ctx: OperationContext<kv::list::Request>) -> GlobalResult<kv::list::Response> {
	// This function is very expensive, since it scans all rows in the KV
	// without an index.
	//
	// We do this because building an index over the directory will make writes
	// much more expensive. We only use this for the developer dashboard and not
	// production games.

	let crdb = ctx.crdb("db-kv").await?;

	let namespace_id = internal_unwrap!(ctx.namespace_id).as_uuid();
	let limit = ctx.limit.map(|x| x as i64).unwrap_or(DEFAULT_LIMIT);

	// Query keys.
	//
	// Uses `AS OF SYSTEM TIME` to improve performance and avoid locking.
	let mut entries = if ctx.with_values {
		sqlx::query_as::<_, (String, String)>(indoc!(
			"
			SELECT key, value::TEXT
			FROM kv AS OF SYSTEM TIME '-1s'
			WHERE namespace_id = $1 AND directory = $2
			LIMIT $3
			"
		))
		.bind(namespace_id)
		.bind(&ctx.directory)
		.bind(limit)
		.fetch_all(&crdb)
		.await?
		.into_iter()
		.map(|(key, value)| kv::list::response::Entry {
			key,
			value: Some(value.as_bytes().to_vec()),
		})
		.collect::<Vec<_>>()
	} else {
		sqlx::query_as::<_, (String,)>(indoc!(
			"
			SELECT key
			FROM kv AS OF SYSTEM TIME '-1s'
			WHERE namespace_id = $1 AND directory = $2
			LIMIT $3
			"
		))
		.bind(namespace_id)
		.bind(&ctx.directory)
		.bind(limit)
		.fetch_all(&crdb)
		.await?
		.into_iter()
		.map(|(key,)| kv::list::response::Entry { key, value: None })
		.collect::<Vec<_>>()
	};

	// This is not effective if there's > ctx.limit values, but it's helpful
	// when there's a few keys to ensure the listing is consistent.
	entries.sort_by(|a, b| a.key.cmp(&b.key));

	Ok(kv::list::Response { entries })
}
