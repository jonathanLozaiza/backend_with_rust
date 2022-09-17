#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

use dotenvy::dotenv;
use std::env;
use diesel::{pg::PgConnection, Connection, RunQueryDsl};
// get Post and Schema
use self::models::Post;
use self::schema::posts::dsl::*;

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must");
    let mut conn = PgConnection::establish(&database_url).expect("Connection to database failed");

    // select * from database_url
    let postsDB = posts.load::<Post>(&mut conn).expect("query error");;
    for post in postsDB {
        println!("{}", post.title);
    }
}
