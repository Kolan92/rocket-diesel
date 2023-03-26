use diesel::{table, Insertable, Queryable};
use serde::{Deserialize, Serialize};

table! {
    blog_posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

#[derive(Serialize, Deserialize, Queryable, Debug, Insertable)]
#[table_name = "blog_posts"]
pub struct BlogPost {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ApiError {
    pub details: String,
}