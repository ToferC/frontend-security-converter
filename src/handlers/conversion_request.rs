use std::sync::Arc;

use actix_session::SessionExt;
use actix_web::{HttpRequest, HttpResponse, Responder, get, web};
use actix_identity::{Identity};


use crate::{AppData, generate_basic_context};
use crate::graphql::{get_authority_by_id, get_conversion_request_by_id};

#[get("/{lang}/conversion_request")]
pub async fn conversion_request_form(
    data: web::Data<AppData>,
    id: Option<Identity>,
    path: web::Path<String>,
    req:HttpRequest) -> impl Responder {
    let lang = path.into_inner();

    let session = req.get_session();

    let mut ctx = generate_basic_context(id, &lang, req.uri().path(), &session);

    let authority_id = match req.get_session().get::<String>("authority_id").unwrap() {
        Some(s) => s,
        None => "".to_string(),
    };

    let bearer = match req.get_session().get::<String>("bearer").unwrap() {
        Some(s) => s,
        None => "".to_string(),
    };

    let r = get_authority_by_id(authority_id, bearer, &data.api_url, Arc::clone(&data.client))
        .await
        .expect("Unable to get authority");

    ctx.insert("authority", &r.authority_by_id);

    let rendered = data.tmpl.render("conversion_request/conversion_request.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[get("/{lang}/conversion_request/{id}")]
pub async fn view_conversion_request(
    data: web::Data<AppData>,
    id: Option<Identity>,
    path: web::Path<(String, String)>,
    req: HttpRequest,
) -> impl Responder {
    let (lang, request_id) = path.into_inner();

    let session = req.get_session();

    let mut ctx = generate_basic_context(id, &lang, req.uri().path(), &session);

    let bearer = match session.get::<String>("bearer").unwrap() {
        Some(s) => s,
        None => "".to_string(),
    };

    let conversion_request_response = get_conversion_request_by_id(
        request_id.clone(),
        bearer,
        &data.api_url,
        Arc::clone(&data.client),
    )
    .await
    .expect("Unable to get conversion request");

    ctx.insert("conversion_request", &conversion_request_response.conversion_request_by_id);

    // Serialize conversion request to JSON for JavaScript rendering
    let request_json = serde_json::to_string(&conversion_request_response.conversion_request_by_id)
        .expect("Failed to serialize conversion request data");
    ctx.insert("request_json", &request_json);

    let rendered = data.tmpl.render("conversion_request/view_request.html", &ctx).unwrap();
    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(rendered)
}

