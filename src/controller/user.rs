use crate::data::user as user_model;
use crate::error::Error;
use actix_web::*;
use deadpool_postgres::{Client, Pool};
use crate::controller::HttpResult;

#[post("/add_user")]
async fn add_user(
    user: web::Json<user_model::User>,
    db_pool: web::Data<Pool>,
) -> HttpResult {
    let user_info: user_model::User = user.into_inner();
    let client: Client = db_pool.get().await.map_err(Error::PoolError)?;

    let new_user = user_model::add_user(&client, user_info).await?;
    // known issues: same user_name raises an 500.
    // if checked by frontend then this is ok?
    Ok(HttpResponse::Ok().json(&new_user))
}

pub fn user_service() -> Scope {
    web::scope("/user")
        .service(add_user)
}