use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod schema {
    table! {
        reports (uuid) {
            uuid -> Uuid,
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
    pub uuid: Uuid,
    pub description: String,
    pub lat: f32,
    pub lng: f32,
    pub status: String
}

#[derive(Deserialize)]
pub struct NewReport {
    pub description: String,
    pub lat: f32,
    pub lng: f32,
}

pub fn insert_report(
    rp: &NewReport,
    conn: &PgConnection,
) -> Result<Report, diesel::result::Error> {
    use self::reports::dsl::*;
    
    let report = Report {
        uuid: Uuid::new_v4(),
        description: rp.description.to_owned(),
        lat: rp.lat.to_owned(),
        lng: rp.lng.to_owned(),
        status: String::from("new")
    };

    diesel::insert_into(reports).values(&report).execute(conn)?;

    Ok(report)
}
