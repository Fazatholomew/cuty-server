use crate::diesel::prelude::*;
use crate::diesel::{Insertable, Queryable};
use rocket::serde::{Deserialize, Serialize};

use crate::schema::links;
use crate::Db;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
#[table_name = "links"]
pub struct Link {
    pub shortUrl: String,
    pub redirectUrl: Option<String>,
    pub photoUrl: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LinkInput {
    pub shortUrl: String,
    pub redirectUrl: Option<String>,
    pub photoUrl: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub token: Option<String>,
    pub password: Option<String>
}

impl Link {
    pub async fn select(shortUrl: String, connection: Db) -> Vec<Link> {
        connection
            .run(move |c| {
                let result = links::table
                    .filter(links::shortUrl.eq(shortUrl))
                    .limit(1)
                    .load::<Link>(c);
                result.unwrap_or(vec![])
            })
            .await
    }

    pub async fn insert(data: Link, connection: Db) -> String {
        let current_shortUrl = data.shortUrl.clone();
        connection
            .run(move |c| {
                // let t = Link {
                //     shortUrl: data.shortUrl,
                //     redirectUrl: data.redirectUrl,
                //     photoUrl: data.photoUrl,
                //     title: data.title,
                //     description: data.description,
                // };
                diesel::insert_into(links::table).values(&data).execute(c)
            }).await;
        current_shortUrl
    }
}
