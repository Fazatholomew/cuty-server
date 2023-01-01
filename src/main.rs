#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

pub mod link;
pub mod schema;

use crate::diesel::prelude::*;
use crate::link::Link;
use diesel::result::Error;
use rocket::{response::status, http::Status};
use rocket_dyn_templates::{context, Template};
use rocket::serde::json::Json;

// use rocket::fairing::AdHoc;
// use rocket::{Build, Rocket};

use rocket_sync_db_pools::database;

#[database("cuty")]
pub struct Db(diesel::SqliteConnection);

#[get("/<slug>")]
async fn index(slug: &str, db: Db) -> Result<Template, Status> {
    let results = Link::select(slug.to_owned(), db).await;
    if results.len() > 0 {
        let current_link = results[0].to_owned();
        Ok(Template::render(
            "index",
            context! {
                title: current_link.title,
                description: current_link.description,
                redirectUrl: current_link.redirectUrl,
                photoUrl: current_link.photoUrl
            },
        ))
    } else {
       Err(Status::NotFound)
    }
}

#[post("/", format = "json", data = "<data>")]
async fn insert_link(data: Json<Link>, db: Db) -> Result<status::Created<&'static str>, status::BadRequest<String>> {
    let current_link_data = data.into_inner();
    let result = Link::insert(current_link_data, db).await;
    Ok(status::Created::new(result))
        // Err(status::BadRequest(Some(result.err().unwrap().to_string())))
    
}

// async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
//     use diesel_migrations::{embed_migrations, EmbedMigrations, MigrationConnection};

//     const MIGRATIONS: EmbedMigrations = embed_migrations!("migrations");

//     Db::get_one(&rocket)
//         .await
//         .expect("database connection")
//         .run(|conn| {
//             conn.run_pending_migrations(MIGRATIONS)
//                 .expect("diesel migrations");
//         })
//         .await;

//     rocket
// }

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .attach(Db::fairing())
        // .attach(AdHoc::on_ignite("Diesel Migrations", run_migrations))
        .mount("/", routes![index, insert_link])
}
