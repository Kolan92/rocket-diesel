use diesel::RunQueryDsl;
use rocket::{response::status::NotFound, serde::json::Json};
extern crate diesel;
use diesel::prelude::*;
mod models;
use crate::{
    models::BlogPost,
    models::{blog_posts, ApiError},
};
use rocket_sync_db_pools::database;

#[macro_use]
extern crate rocket;

#[get("/<id>")]
async fn get_blog_post(
    connection: Db,
    id: i32,
) -> Result<Json<BlogPost>, NotFound<Json<ApiError>>> {
    connection
        .run(move |c| blog_posts::table.filter(blog_posts::id.eq(id)).first(c))
        .await
        .map(Json)
        .map_err(|e| {
            NotFound(Json(ApiError {
                details: e.to_string(),
            }))
        })
}
#[get("/")]
async fn get_all_blog_posts(connection: Db) -> Json<Vec<BlogPost>> {
    connection
        .run(|c| blog_posts::table.load(c))
        .await
        .map(Json)
        .expect("Failed to fetch blog posts")
}
#[get("/")]
fn index() -> &'static str {
    "Hello, world2!"
}

#[post("/", data = "<blog_post>")]
async fn create_blog_post(connection: Db, blog_post: Json<BlogPost>) -> Json<BlogPost> {
    connection
        .run(move |c| {
            diesel::insert_into(blog_posts::table)
                .values(&blog_post.into_inner())
                .get_result(c)
        })
        .await
        .map(Json)
        .expect("Failed to insert post")
}

#[database("blogs_db")]
pub struct Db(diesel::PgConnection);

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build();
    rocket
        .attach(Db::fairing())
        .mount("/", routes![index])
        .mount(
            "/blog-posts",
            routes![get_blog_post, get_all_blog_posts, create_blog_post],
        )
}
