mod models;
mod schema;

use rocket::form::FromForm;
use rocket::{get, post};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::settings::UrlObject;
use rocket_okapi::{openapi, openapi_get_routes, rapidoc::*, swagger_ui::*};
use rocket_okapi::request::OpenApiFromRequest;
use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket::response::{Debug, status::Created};
use rocket::fairing::AdHoc;

use models::*;
use schema::posts;

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;


#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket_sync_db_pools;

//use self::diesel::prelude::*;

use rocket_sync_db_pools::{database, diesel::prelude::*};

#[derive(OpenApiFromRequest)]
#[database("boilerplate")]
pub struct DB(rocket_sync_db_pools::diesel::MysqlConnection);

/// # Lists all Posts
///
/// Returns list of Posts
#[openapi(tag = "Posts")]
#[get("/")]
async fn list_posts(db: DB) -> Result<Json<Vec<Post>>> {
    let postss: Vec<Post> = db.run(move |conn| {
        posts::table
            .select(posts::all_columns)
            .load(conn)
    }).await?;

    Ok(Json(postss))
}

/// # Create new Post
///
/// Returns successful or not
#[openapi(tag = "Posts")]
#[post("/", data = "<post>")]
async fn create_post(db: DB, post: Json<Post>) -> Result<Created<Json<Post>>> {
    let post_value = post.clone();
    db.run(move |conn| {
        diesel::insert_into(posts::table)
            .values(&post_value)
            .execute(conn)
    }).await?;

    Ok(Created::new("/").body(post))
}

#[rocket::main]
async fn main() {
    rocket::build()
        .attach(DB::fairing())
        .mount( "/",
         openapi_get_routes![
             create_post,
             list_posts
         ],
        )
        .mount(
            "/swagger/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .launch()
        .await
        .unwrap();
}
