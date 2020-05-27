//psql -d swell -a -f swell.sql
use swell_server::db::*;
use swell_server::filters::rest_swell(db);

#[tokio::main]
async fn main() {
    let db = get_db().await;
    //let rest_api =
    println!("Hello, world!");
}
