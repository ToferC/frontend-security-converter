use actix_web::{web, get, HttpResponse, HttpRequest, Responder};
use actix_session::SessionExt;
use actix_identity::Identity;
use std::sync::Arc;

use crate::{AppData, generate_basic_context};
use crate::graphql::get_authority_by_id;

#[get("/{lang}/authority_analytics/{authority_id}")]
pub async fn authority_analytics(
    data: web::Data<AppData>,
    req: HttpRequest,
    id: Option<Identity>,
    path: web::Path<(String, String)>,
) -> impl Responder {

    let session = req.get_session();

    let (lang, authority_id) = path.into_inner();

    let mut ctx = generate_basic_context(id, &lang, req.uri().path(), &session);

    let session = req.get_session();
    let bearer = match session.get::<String>("bearer").unwrap() {
        Some(s) => s,
        None => "".to_string(),
    };

    // Fetch the authority by ID from GraphQL
    let authority_response = get_authority_by_id(
        authority_id,
        bearer,
        &data.api_url,
        Arc::clone(&data.client)
    )
    .await
    .expect("Unable to get authority");

    ctx.insert("authority", &authority_response.authority_by_id);

    // Serialize authority data to JSON for JavaScript
    let authority_json = serde_json::to_string(&authority_response.authority_by_id)
        .expect("Failed to serialize authority data");
    ctx.insert("authority_json", &authority_json);

    let rendered = data.tmpl.render("analytics/authority_analytics.html", &ctx).unwrap();
    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(rendered)
}