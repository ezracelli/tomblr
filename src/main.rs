use std::convert::Infallible;

use graphql::http::graphiql_source;
use warp::Filter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    env_logger::try_init()?;

    let pool = tumblr::db::Pool::new(std::env::var("DATABASE_URL")?)?;
    let schema = tumblr::Schema::build(Default::default(), Default::default(), Default::default())
        .data(pool)
        .finish();

    let filter = warp::path::end()
        .and(warp::get())
        .map(|| warp::reply::html(graphiql_source("/", None)))
        .or(graphql_warp::graphql(schema).and_then(
            |(schema, request): (tumblr::Schema, graphql::Request)| async move {
                Ok::<graphql_warp::Response, Infallible>(schema.execute(request).await.into())
            },
        ))
        .with(warp::log(env!("CARGO_PKG_NAME")));

    warp::serve(filter).run(([0, 0, 0, 0], 4000)).await;
    Ok(())
}
