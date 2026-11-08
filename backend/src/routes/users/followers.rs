use actix_web::{get, web::Json, web::Path, HttpResponse};
use diesel::prelude::*;

use crate::{
    config::AppConfig,
    establish_connection,
    routes::{
        activity_pub::collection::ActorCollection,
        users::has_user,
    },
    schema::{followers, remote_actors},
};

#[get("/followers")]
pub async fn get_followers(path: Path<String>) -> HttpResponse {
    let username = path.into_inner().to_lowercase();
    let Some(user) = has_user(username.clone()) else {
        return HttpResponse::NotFound().finish();
    };

    let connection = &mut establish_connection();
    let actors = followers::table
        .inner_join(remote_actors::table)
        .filter(followers::user_id.eq(&user.id))
        .filter(followers::accepted.eq(true))
        .select(remote_actors::id)
        .load::<String>(connection);

    match actors {
        Ok(actors) => {
            let collection =
                ActorCollection::new(&AppConfig::get().followers_url(&username), actors);
            match collection {
                Ok(collection) => HttpResponse::Ok()
                    .content_type(
                        "application/activity+json; profile=\"https://www.w3.org/ns/activitystreams\"",
                    )
                    .json(Json(collection)),
                Err(error) => HttpResponse::InternalServerError().body(error),
            }
        }
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}
