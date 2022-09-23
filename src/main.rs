#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use dotenvy::dotenv;
use std::env;
//use diesel::{pg::PgConnection, Connection, RunQueryDsl};
use diesel::prelude::*;

// get Post and Schema
use self::models::{Post, NewPost, PostSimplificado, NewPostHandler};
//use self::schema::posts;
use self::schema::posts::dsl::*;

// Librerías para crear una conexión a la BBDD y compartirla en toda la aplicación
use diesel::r2d2::{self, ConnectionManager};
use diesel::r2d2::Pool;

// Importamos TERA
use tera::Tera;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[get("/")]
async fn index(pool: web::Data<DbPool>, template_manager: web::Data<tera::Tera>) -> impl Responder {
    let mut conn = pool.get().expect("Problemas al traer la base de datos");

    // Consulta para obtener todos los registros
    match web::block(move || {posts.load::<Post>(&mut conn)}).await {
        Ok(data) => {
            let data = data.unwrap();

            // Enviamos, a través del contexto, los datos al HTML
            let mut ctx = tera::Context::new();
            ctx.insert("posts", &data);

            // Pasamos los datos al template index.html
            HttpResponse::Ok().content_type("text/html").body(
                template_manager.render("index.html", &ctx).unwrap()
            )
        },
        Err(err) => HttpResponse::Ok().body("Error al recibir la data")
    }
}

#[post("/new_post")]
async fn new_post(pool: web::Data<DbPool>, item: web::Json<NewPostHandler>) -> impl Responder {
    // Traemos el POOL para disponer de la conexión a la BBDD
    let mut conn = pool.get().expect("Problemas al traer la base de datos");

    // Utiliamos la función creada en el modelo para crear un nuevo registro y devolverlo
    match web::block(move || {Post::create_post(&mut conn, &item)}).await {
        Ok(data) => {
            return HttpResponse::Ok().body(format!("{:?}", data));
        },
        Err(err) => HttpResponse::Ok().body("Error al recibir la data")
    }
}

#[get("/tera_test")]
async fn tera_test(template_manager: web::Data<tera::Tera>) -> impl Responder {

    // Creamos un contexto para pasarle datos al template
    let mut ctx = tera::Context::new();

    // Enviamos el template que queremos localizándolo por su nombre
    HttpResponse::Ok().content_type("text/html").body(
        template_manager.render("index.html", &ctx).unwrap()
    )
}

// Capturamos el parámetro por URL
#[get("/blog/{blog_slug}")]
async fn get_post(
    pool: web::Data<DbPool>, 
    template_manager: web::Data<tera::Tera>, 
    blog_slug: web::Path<String>
) -> impl Responder {
    let mut conn = pool.get().expect("Problemas al traer la base de datos");

    let url_slug = blog_slug.into_inner();

    match web::block(move || {posts.filter(slug.eq(url_slug)).load::<Post>(&mut conn)}).await {
        Ok(data) => {
            let data = data.unwrap();

            // Si el post no existe, devolvemos 404
            if data.len() == 0 {
                return HttpResponse::NotFound().finish();
            }

            let data = &data[0];

            // Enviamos, a través del contexto, los datos del post al HTML
            let mut ctx = tera::Context::new();
            ctx.insert("post", data);

            HttpResponse::Ok().content_type("text/html").body(
                template_manager.render("post.html", &ctx).unwrap()
            )
        },
        Err(err) => HttpResponse::Ok().body("Error al recibir la data")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must");
    //let mut conn = PgConnection::establish(&database_url).expect("Connection to database failed");

    let connection = ConnectionManager::<PgConnection>::new(database_url);

    // El POOL sirve para compartir la conexión con otros servicios
    let pool = Pool::builder().build(connection).expect("No se pudo construir el Pool");

    // insert the post into
    //  let new_post = NewPost {
    //      title: "Mi segundo Blog Post",
    //      body: "Lorem Ipsum...",
    //      slug: "segundo-blog-post"
    //  };
    //  let post: Post = diesel::insert_into(posts::table).values(new_post).get_result(&mut conn).expect("ha ocurrido un error insertando datos en la db.");

    HttpServer::new(move || {
        // Instanciamos TERA y le indicamos en qué directorio buscar los templates
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
            // Compartimos el pool de conexión con todos los endpoints
        App::new()
            .service(index)
            .service(new_post)
            .service(get_post)
            .service(tera_test)
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(tera.clone()))
    }).bind(("127.0.0.1", 8080)).unwrap().run().await

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
    // diesel::delete(posts.filter(slug.like("%-post%")))
    //     .execute(&mut conn)
    //     .expect("Error deleting record");

    // // select * from database_url
    // println!("consultar todos los items");
    // let postsDB = posts.load::<Post>(&mut conn).expect("query error");
    // for post in postsDB {
    //     println!("{:?}", post);
    // }

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