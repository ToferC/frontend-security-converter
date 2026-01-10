// example auth: https://github.com/actix/actix-extras/blob/master/actix-identity/src/lib.rs

use serde::{Deserialize};

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct UserForm {
    user_name: String,
    email: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct AdminUserForm {
    user_name: String,
    email: String,
    role: String,
    validated: String,
}

use actix_session::{SessionExt};
use actix_web::{HttpRequest, HttpResponse, Responder, get, web};
use actix_identity::{Identity};
use std::sync::Arc;


use crate::{AppData, generate_basic_context};
use crate::graphql::{get_user_by_id};

#[get("/{lang}/user/{user_id}")]
pub async fn user_by_id(
    data: web::Data<AppData>,
    id: Option<Identity>,
    path: web::Path<(String, String)>,
    req:HttpRequest) -> impl Responder {

    let (lang, user_id) = path.into_inner();

    let session = req.get_session();

    let mut ctx = generate_basic_context(id, &lang, req.uri().path(), &session);

    let bearer = match req.get_session().get::<String>("bearer").unwrap() {
        Some(s) => s,
        None => "".to_string(),
    };

    let r = get_user_by_id(user_id, bearer, &data.api_url, Arc::clone(&data.client))
        .await
        .expect("Unable to get user");

    ctx.insert("user", &r.user_by_id);

    let rendered = data.tmpl.render("users/user_page.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}



