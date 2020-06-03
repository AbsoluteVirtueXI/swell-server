//psql -d swell -a -f swell.sql
use swell_server::db::*;
use swell_server::filters::*;

#[tokio::main]
async fn main() {
    let db = get_db().await;
    let rest_api = rest_swell(db);

    let routes = rest_api;
    warp::serve(routes).run(([192, 168, 0, 10], 7777)).await;
}
