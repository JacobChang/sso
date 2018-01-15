use rocket::State;
use rocket_contrib::Json;

use hex;
use hex::{FromHex};
use redis::Commands;
use url::Url;

use super::Error;
use super::super::guards::bearer;
use super::super::guards::bearer::AuthorizationBearer;
use super::super::common;
use super::super::config::Config;
use super::super::models::application;
use super::super::models::application::ClientSecret;
use super::super::models::authorization;
use super::super::models::authorization::{Authorization, AuthorizationPreview};
use super::super::storage::{Database, Cache};

const AUTH_CODE_SIZE: usize = 64;

#[derive(Serialize, Deserialize)]
pub struct CreateAuthorizationParams {
    client_id: String,
    scope: String,
    redirect_uri: String,
    response_type: String,
    state: String,
}

#[derive(Serialize, Deserialize)]
pub struct Credientials {
    code: String,
    state: String,
}

#[post("/users/<user_id>/authorizations", data = "<params>")]
pub fn create_authorization(
    config: State<Config>,
    cache: State<Cache>,
    db: State<Database>,
    user_id: i64,
    params: Json<CreateAuthorizationParams>,
    bearer: AuthorizationBearer,
) -> Result<Json<Credientials>, Error> {
    let claims = bearer::decode(&config.jwt.secret, bearer.0.as_str())?;
    if claims.uid == user_id {
        let pg_conn = db.get_conn()?;
        let client_id = Vec::<u8>::from_hex(&params.client_id)?;
        let client_application =
            application::select_one(&*pg_conn, &client_id, ClientSecret::plaintext)?;

        let callback_uri = Url::parse(&client_application.callback_uri)?;
        let redirect_uri = Url::parse(&params.redirect_uri)?;
        if callback_uri.origin() != redirect_uri.origin() ||
            callback_uri.path() != redirect_uri.path()
        {
            return Err(Error::Params);
        }

        let new_authorization =
            authorization::create(&*pg_conn, user_id, &client_id, &params.scope)?;

        let redis_conn = cache.get_conn()?;
        let code = common::gen_rand_bytes(AUTH_CODE_SIZE)?;
        let expire = 5 * 60;
        let key = format!("oauth:code:{}", hex::encode(&code));
        let _: String = redis_conn.set_ex(&key, new_authorization.id, expire)?;

        let credientials = Credientials {
            code: hex::encode(&code),
            state: params.state.clone(),
        };

        Ok(Json(credientials))
    } else {
        Err(Error::Privilege)
    }
}

#[get("/users/<user_id>/authorizations")]
pub fn select_authorizations(
    config: State<Config>,
    db: State<Database>,
    user_id: i64,
    bearer: AuthorizationBearer,
) -> Result<Json<Vec<Authorization>>, Error> {
    let claims = bearer::decode(&config.jwt.secret, bearer.0.as_str())?;
    if claims.uid == user_id {
        let pg_conn = db.get_conn()?;
        let authorizations = authorization::select(&*pg_conn, user_id)?;

        Ok(Json(authorizations))
    } else {
        Err(Error::Privilege)
    }
}

#[delete("/users/<user_id>/authorizations/<authorization_id>")]
pub fn remove_authorization(
    config: State<Config>,
    db: State<Database>,
    user_id: i64,
    authorization_id: i64,
    bearer: AuthorizationBearer,
) -> Result<Json<Authorization>, Error> {
    let claims = bearer::decode(&config.jwt.secret, bearer.0.as_str())?;
    if claims.uid == user_id {
        let pg_conn = db.get_conn()?;
        let removed_authorization = authorization::remove(&*pg_conn, user_id, authorization_id)?;

        Ok(Json(removed_authorization))
    } else {
        Err(Error::Privilege)
    }
}

#[derive(Serialize, Deserialize, FromForm)]
pub struct SelectAuthorizationParams {
    client_id: String,
    scope: String,
}

#[get("/authorizations/preview?<params>")]
pub fn preview_authorization(
    db: State<Database>,
    params: SelectAuthorizationParams,
) -> Result<Json<AuthorizationPreview>, Error> {
    let client_id = Vec::<u8>::from_hex(params.client_id)?;
    let pg_conn = db.get_conn()?;
    let match_authorization = authorization::preview(&*pg_conn, &client_id, &params.scope)?;

    Ok(Json(match_authorization))
}
