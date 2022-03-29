use actix_web::{HttpRequest, HttpResponse, web};
use diesel::pg::Pg;
use diesel::{OptionalExtension, PgConnection, QueryDsl, RunQueryDsl};
use log::{error, info};
use crate::DbPool;

use diesel_demo;
use diesel_demo::models::SessionKeys;

use crate::repository::keys;

pub async fn get_all(req: HttpRequest, pool: web::Data<DbPool>) -> HttpResponse {
    info!("{:?}", req);
    let con = pool.get().expect("couldn't get db connection from pool");

    let result = web::block(move || keys::find_all(&con))
        .await
        .map_err(|e| {
            error!("{:?}", e);
            HttpResponse::InternalServerError().finish()
        });

    match result {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => e
    }
}

pub async fn get_by_public_id(req: HttpRequest, id: web::Path<uuid::Uuid>, pool: web::Data<DbPool>) -> HttpResponse {
    info!("{:?}", req);
    let con = pool.get().expect("couldn't get db connection from pool");

    let result = web::block(move || keys::find_by_public_id(&con, id.into_inner()))
        .await
        .map_err(|e| {
            error!("{:?}", e);
            HttpResponse::InternalServerError().finish()
        });

    match result {
        Ok(option) => {
            match option {
                Some(user) => HttpResponse::Ok().json(user),
                None => HttpResponse::NotFound().finish()
            }
        }
        Err(e) => e
    }
}
