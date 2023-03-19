use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use chrono;
use crate::schema::bird_sighting;

#[derive(Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct Bird {
    pub id: i32,
    pub name: String,
    pub scientific_name: String,
    pub commonwealth_status: String,
}

#[derive(Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct BirdSighting {
    pub id: i32,
    pub bird_id: i32,
    pub sighting_date: Option<chrono::NaiveDateTime>,
    pub sighting_location: Option<String>,
    pub additional_information: Option<String>,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = bird_sighting)]
pub struct BirdSightingInput {
    pub bird_id: i32,
    pub sighting_location: Option<String>,
    pub additional_information: Option<String>,
    pub sighting_date: Option<chrono::NaiveDateTime>,
}