#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

pub mod cors;
pub mod link;
pub mod schema;

use crate::cors::CORS;
use crate::diesel::prelude::*;
use crate::link::Link;
use link::LinkInput;
use rocket::serde::{Deserialize};
use rocket::serde::json::{Json};
use rocket::{
    http::{Status},
    response::status,
};
use rocket_dyn_templates::{Template};
use reqwest::header::CONTENT_LENGTH;

// use rocket::fairing::AdHoc;
// use rocket::{Build, Rocket};

use rocket_sync_db_pools::database;

#[database("cuty")]
pub struct Db(diesel::SqliteConnection);

#[derive(Debug, Clone, Deserialize)]
#[serde(crate = "rocket::serde")]
struct CheckCaptchaResult {
    success: Option<bool>,
    score: Option<f32>,
    #[serde(alias="error-codes")]
    error_codes: Option<Vec<String>>,
}

#[get("/<slug>")]
async fn index(slug: &str, db: Db) -> Result<Template, Status> {
    let results = Link::select(slug.to_owned(), db).await;
    if results.len() > 0 {
        // let current_link = results[0].to_owned();
        Ok(Template::render(
            "index",
            // context! {
            //     // title: current_link.title,
            //     // description: current_link.description,
            //     // redirectUrl: current_link.redirectUrl,
            //     // photoUrl: current_link.photoUrl,
            //     ..current_link
            // },
            results[0].to_owned(),
        ))
    } else {
        Err(Status::NotFound)
    }
}

#[post("/", format = "json", data = "<data>")]
async fn insert_link(
    data: Json<LinkInput>,
    db: Db,
) -> Result<status::Created<&'static str>, status::BadRequest<String>> {
    let current_link_data = data.into_inner();
    if current_link_data.password.is_some() {
        return Err(status::BadRequest(Some("Bot!".to_owned())));
    }
    if current_link_data.token.is_none() {
        return Err(status::BadRequest(Some("Bot!".to_owned())));
    }
    let url = format!("https://www.google.com/recaptcha/api/siteverify?secret={}&response={}", dotenvy::var("SECRET_KEY").unwrap(), current_link_data.token.unwrap());
    let new_post: CheckCaptchaResult = reqwest::Client::new()
        .post(url)
        .header(CONTENT_LENGTH, 0)
        .send()
        .await
        .map_err(|err| status::BadRequest(Some(err.to_string())))?
        .json()
        .await
        .map_err(|err| status::BadRequest(Some(err.to_string())))?;
    let errors = new_post.error_codes.unwrap_or(vec![]);
    if errors.len() > 0 {
        return Err(status::BadRequest(Some(errors.join(", "))));
    }
    if new_post.success.is_none() || new_post.score.is_none() {
        return Err(status::BadRequest(Some("Bot!".to_owned())));  
    }
    if new_post.success.unwrap() && new_post.score.unwrap() > 0.5 {
        let cleaned = Link {
            shortUrl: current_link_data.shortUrl,
            redirectUrl: current_link_data.redirectUrl,
            photoUrl: current_link_data.photoUrl,
            title: current_link_data.title,
            description: current_link_data.description,
        };
        let result = Link::insert(cleaned, db).await;
        return Ok(status::Created::new(result));
    }
    return Err(status::BadRequest(Some("Bot!".to_owned()))); 
}

#[options("/<_..>")]
fn all_options() {
    /* Intentionally left empty */
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
        .attach(CORS)
        // .attach(AdHoc::on_ignite("Diesel Migrations", run_migrations))
        .mount("/", routes![index, insert_link, all_options])
}
