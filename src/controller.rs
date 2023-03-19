use diesel::prelude::*;
use rocket::response::status::NoContent;
use rocket::serde::json::Json;

use crate::database;
use crate::models::*;

#[get("/")]
pub fn index() -> Json<Vec<Bird>> {
    use crate::schema::bird::dsl::bird;
    let connection = &mut database::establish_connection();
    bird.load::<Bird>(connection).map(Json).expect("Error loading birds")
}

#[post("/sighting", data = "<sighting>")]
pub fn new_sighting(sighting: Json<BirdSightingInput>) -> Json<BirdSighting> {
    use crate::schema::bird_sighting;

    let connection = &mut database::establish_connection();
    diesel::insert_into(bird_sighting::table)
        .values(sighting.into_inner())
        .execute(connection)
        .expect("Error adding sighting");

    Json(bird_sighting::table
        .order(bird_sighting::id.desc())
        .first(connection).unwrap()
    )
}

#[get("/sighting?<bird>")]
pub fn all_sightings(bird: Option<i32>) -> Json<Vec<BirdSighting>> {
    let connection = &mut database::establish_connection();
    use crate::schema::bird_sighting::dsl::{bird_id, bird_sighting};

    let query_result: QueryResult<Vec<BirdSighting>> = match bird {
        Some(id) => {
            bird_sighting.filter(bird_id.eq(id)).load(connection)
        }
        None => bird_sighting.load(connection)
    };

    query_result.map(Json).expect("Error loading sightings")
}

#[delete("/sighting/<sighting_id>")]
pub fn delete_sighting(sighting_id: i32) -> NoContent {
    use crate::schema::bird_sighting::dsl::*;

    let connection = &mut database::establish_connection();
    diesel::delete(bird_sighting.filter(id.eq(sighting_id)))
        .execute(connection).expect("Error deleting sighting");

    NoContent
}