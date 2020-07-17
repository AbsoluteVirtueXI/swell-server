//psql -d swell -a -f swell.sql
//SERVER COMIT TEST
use swell_server::database::*;
use swell_server::filters::*;

use dotenv::dotenv;
use std::env;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()>{
    dotenv()?;
    let database_url = env::var("DATABASE_URL").unwrap();
    let db = Database::new(&database_url).await?;

    let rest_api = rest_swell(db);

    let routes = rest_api;
    warp::serve(routes)
        .run(([127, 0, 0, 1], 7777)).await;

    Ok(())
}
