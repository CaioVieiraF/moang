use actix_session::Session;
use actix_web::{
    delete,
    get,
    post,
    web::{Json, Path},
    HttpResponse,
};
use diesel::prelude::*;
use serde::Deserialize;
use serde_json::json;

use crate::{
    config::AppConfig,
    current_user_id,
    establish_connection,
    models::User,
    routes::{
        activity_pub::{
            collection::ActorCollection,
            delivery::{enqueue_activity, new_activity_id, save_activity},
            fetch::fetch_remote_actor,
            Context, ObjType,
        },
        users::has_user,
    },
    schema::{activities, following, remote_actors},
    user_is_loged_in,
};

#[derive(Deserialize)]
pub struct FollowRequest {
    actor: String,
}

#[get("/following")]
pub async fn get_following(path: Path<String>) -> HttpResponse {
    let username = path.into_inner().to_lowercase();
    let Some(user) = has_user(username.clone()) else {
        return HttpResponse::NotFound().finish();
    };

    let connection = &mut establish_connection();
    let actors = following::table
        .inner_join(remote_actors::table)
        .filter(following::user_id.eq(&user.id))
        .filter(following::accepted.eq(true))
        .select(remote_actors::id)
        .load::<String>(connection);

    match actors {
        Ok(actors) => {
            let collection =
                ActorCollection::new(&AppConfig::get().following_url(&username), actors);
            match collection {
                Ok(collection) => HttpResponse::Ok()
                    .content_type(
                        "application/activity+json",
                    )
                    .json(Json(collection)),
                Err(error) => HttpResponse::InternalServerError().body(error),
            }
        }
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

#[post("/following")]
pub async fn follow(
    session: Session,
    path: Path<String>,
    request: Json<FollowRequest>,
) -> HttpResponse {
    if !user_is_loged_in(&session) {
        return HttpResponse::Unauthorized().finish();
    }

    let username = path.into_inner().to_lowercase();
    let Some(user) = has_user(username.clone()) else {
        return HttpResponse::NotFound().finish();
    };
    if !owns_account(&session, &user) {
        return HttpResponse::Forbidden().finish();
    }

    let remote_actor = match fetch_remote_actor(&request.actor, false, Some(&user)).await {
        Ok(actor) => actor,
        Err(error) => return HttpResponse::BadRequest().body(error),
    };

    if following_exists(&user, &remote_actor.id) {
        return HttpResponse::Accepted().finish();
    }

    let config = AppConfig::get();
    let (activity_id, _) = new_activity_id(&user.name);
    let actor_url = config.actor_url(&user.name);
    let payload = json!({
        "@context": Context::activity_streams(),
        "id": activity_id,
        "type": ObjType::Follow,
        "actor": actor_url,
        "object": remote_actor.id,
        "to": [remote_actor.id],
    });

    if let Err(error) = save_activity(
        &activity_id,
        Some(&user.id),
        &actor_url,
        "Follow",
        Some(&remote_actor.id),
        "out",
        payload,
    ) {
        return HttpResponse::InternalServerError().body(error);
    }

    let connection = &mut establish_connection();
    let insert = diesel::insert_into(following::table)
        .values((
            following::user_id.eq(&user.id),
            following::actor_id.eq(&remote_actor.id),
            following::accepted.eq(false),
        ))
        .on_conflict((following::user_id, following::actor_id))
        .do_nothing()
        .execute(connection);

    if let Err(error) = insert {
        return HttpResponse::InternalServerError().body(error.to_string());
    }

    if let Err(error) = enqueue_activity(&activity_id, std::slice::from_ref(&remote_actor.inbox_url))
    {
        return HttpResponse::InternalServerError().body(error);
    }

    HttpResponse::Accepted().finish()
}

#[delete("/following")]
pub async fn unfollow(
    session: Session,
    path: Path<String>,
    request: Json<FollowRequest>,
) -> HttpResponse {
    if !user_is_loged_in(&session) {
        return HttpResponse::Unauthorized().finish();
    }

    let username = path.into_inner().to_lowercase();
    let Some(user) = has_user(username.clone()) else {
        return HttpResponse::NotFound().finish();
    };
    if !owns_account(&session, &user) {
        return HttpResponse::Forbidden().finish();
    }

    let remote_actor = match fetch_remote_actor(&request.actor, false, Some(&user)).await {
        Ok(actor) => actor,
        Err(error) => return HttpResponse::BadRequest().body(error),
    };

    let connection = &mut establish_connection();
    let follow_activity_id = activities::table
        .filter(activities::user_id.eq(&user.id))
        .filter(activities::activity_type.eq("Follow"))
        .filter(activities::object_id.eq(&remote_actor.id))
        .filter(activities::direction.eq("out"))
        .order(activities::created_at.desc())
        .select(activities::id)
        .first::<String>(connection)
        .optional()
        .ok()
        .flatten();

    let deleted = diesel::delete(
        following::table
            .filter(following::user_id.eq(&user.id))
            .filter(following::actor_id.eq(&remote_actor.id)),
    )
    .execute(connection);

    if let Err(error) = deleted {
        return HttpResponse::InternalServerError().body(error.to_string());
    }

    let Some(follow_activity_id) = follow_activity_id else {
        return HttpResponse::NoContent().finish();
    };

    let config = AppConfig::get();
    let (activity_id, _) = new_activity_id(&user.name);
    let actor_url = config.actor_url(&user.name);
    let payload = json!({
        "@context": Context::activity_streams(),
        "id": activity_id,
        "type": ObjType::Undo,
        "actor": actor_url,
        "object": {
            "id": follow_activity_id,
            "type": ObjType::Follow,
            "actor": actor_url,
            "object": remote_actor.id,
        },
        "to": [remote_actor.id],
    });

    if let Err(error) = save_activity(
        &activity_id,
        Some(&user.id),
        &actor_url,
        "Undo",
        Some(&follow_activity_id),
        "out",
        payload,
    ) {
        return HttpResponse::InternalServerError().body(error);
    }

    if let Err(error) = enqueue_activity(&activity_id, std::slice::from_ref(&remote_actor.inbox_url))
    {
        return HttpResponse::InternalServerError().body(error);
    }

    HttpResponse::NoContent().finish()
}

pub(super) fn owns_account(session: &Session, user: &User) -> bool {
    current_user_id(session).as_deref() == Some(user.id.as_str())
}

fn following_exists(user: &User, actor_id: &str) -> bool {
    let connection = &mut establish_connection();
    following::table
        .filter(following::user_id.eq(&user.id))
        .filter(following::actor_id.eq(actor_id))
        .select(following::id)
        .first::<i32>(connection)
        .optional()
        .ok()
        .flatten()
        .is_some()
}
