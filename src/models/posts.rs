use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket_sync_db_pools::diesel::prelude::*;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;

use crate::schema::posts;

#[serde(crate = "rocket::serde")]
#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[table_name = "posts"]
pub struct Post {
    #[serde(skip_deserializing)]
    id: i32,
    title: String,
    text: String,
    #[serde(default)]
    published: bool,
}

