use std::sync::Arc;

use actix_session::SessionExt;
use actix_web::{HttpRequest, HttpResponse, Responder, get, web};
use actix_identity::Identity;

use crate::{AppData, generate_basic_context};
use crate::graphql::{get_metadata_by_tag, get_metadata_by_domain, search_metadata_by_tag};

#[get("/{lang}/metadata/tag/{tag}")]
pub async fn metadata_by_tag(
    data: web::Data<AppData>,
    id: Option<Identity>,
    path: web::Path<(String, String)>,
    req: HttpRequest,
) -> impl Responder {
    let (lang, tag) = path.into_inner();

    let session = req.get_session();

    let mut ctx = generate_basic_context(id, &lang, req.uri().path(), &session);

    let bearer = match session.get::<String>("bearer").unwrap() {
        Some(s) => s,
        None => "".to_string(),
    };

    let metadata_response = get_metadata_by_tag(
        tag.clone(),
        bearer,
        &data.api_url,
        Arc::clone(&data.client),
    )
    .await
    .expect("Unable to get metadata by tag");

    ctx.insert("tag", &tag);
    ctx.insert("metadata_list", &metadata_response.metadata_by_tag);

    // Serialize metadata to JSON for JavaScript rendering
    let metadata_json = serde_json::to_string(&metadata_response.metadata_by_tag)
        .expect("Failed to serialize metadata data");
    ctx.insert("metadata_json", &metadata_json);

    let rendered = data.tmpl.render("metadata/tag_results.html", &ctx).unwrap();
    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(rendered)
}

#[get("/{lang}/search_metadata/pattern/{pattern}")]
pub async fn search_metadata(
    data: web::Data<AppData>,
    id: Option<Identity>,
    path: web::Path<(String, String)>,
    req: HttpRequest,
) -> impl Responder {
    let (lang, pattern) = path.into_inner();

    let session = req.get_session();

    let mut ctx = generate_basic_context(id, &lang, req.uri().path(), &session);

    let bearer = match session.get::<String>("bearer").unwrap() {
        Some(s) => s,
        None => "".to_string(),
    };

    let metadata_response = search_metadata_by_tag(
        pattern.clone(),
        bearer,
        &data.api_url,
        Arc::clone(&data.client),
    )
    .await
    .expect("Unable to search metadata by tag");

    ctx.insert("tag", &pattern);
    ctx.insert("metadata_list", &metadata_response.search_metadata_by_tag);

    // Serialize metadata to JSON for JavaScript rendering
    let metadata_json = serde_json::to_string(&metadata_response.search_metadata_by_tag)
        .expect("Failed to serialize metadata data");
    ctx.insert("metadata_json", &metadata_json);

    let rendered = data.tmpl.render("metadata/tag_results.html", &ctx).unwrap();
    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(rendered)
}

#[get("/{lang}/metadata/domain/{domain}")]
pub async fn metadata_by_domain(
    data: web::Data<AppData>,
    id: Option<Identity>,
    path: web::Path<(String, String)>,
    req: HttpRequest,
) -> impl Responder {
    let (lang, domain) = path.into_inner();

    let session = req.get_session();

    let mut ctx = generate_basic_context(id, &lang, req.uri().path(), &session);

    let bearer = match session.get::<String>("bearer").unwrap() {
        Some(s) => s,
        None => "".to_string(),
    };

    let metadata_response = get_metadata_by_domain(
        domain.clone(),
        bearer,
        &data.api_url,
        Arc::clone(&data.client),
    )
    .await
    .expect("Unable to get metadata by domain");

    ctx.insert("domain", &domain);
    ctx.insert("metadata_list", &metadata_response.metadata_by_domain);

    // Serialize metadata to JSON for JavaScript rendering
    let metadata_json = serde_json::to_string(&metadata_response.metadata_by_domain)
        .expect("Failed to serialize metadata data");
    ctx.insert("metadata_json", &metadata_json);

    let rendered = data.tmpl.render("metadata/domain_results.html", &ctx).unwrap();
    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(rendered)
}
