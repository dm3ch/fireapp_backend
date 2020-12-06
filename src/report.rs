use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod schema {
    table! {
        reports(uiid) {
            uiid -> Uuid,
            description -> Text,
            lat -> Float,
            lng -> Float,
            status -> Text,
        }
    }
}

use self::schema::reports;

#[derive(Serialize, Deserialize, Queryable, Insertable)]
pub struct Report {
    pub uiid: Uuid,
    pub description: String,
    pub lat: f32,
    pub lng: f32,
    pub status: String
}

#[derive(Deserialize)]
pub struct InsertableReport {
    pub description: String,
    pub lat: f32,
    pub lng: f32,
}
