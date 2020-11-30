use diesel::{self, prelude::*};

mod schema {
    table! {
        reports {
            id -> Nullable<Integer>,
            description -> Text,
            lat -> Float,
            lng -> Float,
            completed -> Bool,
        }
    }
}

use self::schema::reports;
use self::schema::reports::dsl::{reports as all_reports};

#[table_name="reports"]
#[derive(Serialize, Queryable, Insertable, Debug, Clone)]
pub struct Report {
    pub id: Option<i32>,
    pub description: String,
    pub lat: f32,
    pub lng: f32,
    pub completed: bool
}

impl Report {
    pub fn all(conn: &SqliteConnection) -> Vec<Report> {
        all_reports.order(reports::id.desc()).load::<Report>(conn).unwrap()
    }
}
