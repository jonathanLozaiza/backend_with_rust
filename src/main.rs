#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

use dotenvy::dotenv;
use std::env;
//use diesel::{pg::PgConnection, Connection, RunQueryDsl};
use diesel::prelude::*;

// get Post and Schema
use self::models::{Post, NewPost, PostSimplificado};
//use self::schema::posts;
use self::schema::posts::dsl::*;

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must");
    let mut conn = PgConnection::establish(&database_url).expect("Connection to database failed");

    // insert the post into
    // let new_post = NewPost {
    //     title: "Mi segundo Blog Post",
    //     body: "Lorem Ipsum...",
    //     slug: "segundo-blog-post"
    // };
    // let post: Post = diesel::insert_into(posts::table).values(new_post).get_result(&mut conn).expect("ha ocurrido un error insertando datos en la db.");

    // UPDATE
    // diesel::update(posts.filter(id.eq(2)))
    //     .set((slug.eq("second-blogpost"), body.eq("maicol i love you!")))
    //     .get_result::<Post>(&mut conn)
    //     .expect("Error updating record");

    // DELETE
    // diesel::delete(posts.filter(slug.eq("second-blogpost")))
    //     .execute(&mut conn)
    //     .expect("Error deleting record");

    // DELETE ALL
    diesel::delete(posts.filter(slug.like("%-post%")))
        .execute(&mut conn)
        .expect("Error deleting record");

    // select * from database_url
    println!("consultar todos los items");
    let postsDB = posts.load::<Post>(&mut conn).expect("query error");
    for post in postsDB {
        println!("{:?}", post);
    }

    // select * from posts limit 1
    // println!("consultar solo 1 item");
    // let postsDB = posts.limit(1).load::<Post>(&mut conn).expect("query error");
    // for post in postsDB {
    //     println!("{:?}", post);
    // }

    // select (title, body) from posts limit 1
    // println!("consultar solo title y body");
    // let postsDB = posts.select((title, body)).limit(1).load::<PostSimplificado>(&mut conn).expect("query error");
    // for post in postsDB {
    //     println!("{:?}", post);
    // }

    // select * from posts order by id limit 1
    // println!("consultar ordenando el id de manera descendiente");
    // let postsDB = posts.order(id.desc()).limit(1).load::<Post>(&mut conn).expect("query error");
    // for post in postsDB {
    //     println!("{:?}", post);
    // }

    // select * from posts where slug = "Mi segundo blog post"
    // println!("consultar filtrando por el title");
    // let postsDB = posts.filter(title.eq("Mi segundo Blog Post")).load::<Post>(&mut conn).expect("query error");
    // for post in postsDB {
    //     println!("{:?}", post);
    // }
}
