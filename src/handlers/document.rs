use std::sync::Arc;

use actix_web::{HttpRequest, HttpResponse, HttpMessage, Responder, get, post, web};
use actix_session::{SessionExt};
use actix_identity::{Identity};
use serde::{Serialize, Deserialize};

use crate::{AppData, generate_basic_context, graphql};

#[derive(Deserialize, Debug, Serialize)]
pub struct DocumentSubmissionForm {
    #[serde(rename = "targetNations")]
    pub target_nations: Vec<String>,

    #[serde(rename = "releasableToOrganizations")]
    pub releasable_to_organizations: Vec<String>,

    #[serde(rename = "disclosureCategory")]
    pub disclosure_category: String,

    #[serde(rename = "handlingRestrictions")]
    pub handling_restrictions: Vec<String>,

    #[serde(rename = "handlingAuthority")]
    pub handling_authority: String,

    pub content: String,
}

/// The JSON formatted data payload submitted to the API that triggers
/// a security classification conversion
#[derive(Debug, Serialize, Deserialize, Clone, InputObject)]
#[graphql(name = "ConversionRequestInput")]
pub struct InsertableConversionRequest {
    pub user_id: Uuid,
    pub authority_id: Uuid,
    pub data_object: InsertableDataObject,
    pub metadata: InsertableMetadata,
    pub source_nation_classification: String,
    pub source_nation_code: String,
    pub target_nation_codes: Vec<String>,
}

/// A lightweight struct to accept JSON formatted data from a ConversionRequest
/// needed to create a NewDataObject
/// GraphQL input type accepts plain String (will be encrypted internally)
#[derive(Debug, Clone, Deserialize, Serialize, InputObject)]
#[graphql(name = "DataObjectInput")]
pub struct InsertableDataObject {
    pub title: String,        // GraphQL input as plain String
    pub description: String,   // GraphQL input as plain String
}

/// A light struct to accept the JSON formatted Metadata included with
/// a ConversionRequest
/// GraphQL input type accepts plain String (will be encrypted internally)
#[derive(Debug, Clone, Deserialize, Serialize, InputObject)]
#[graphql(name = "MetadataInput")]
pub struct InsertableMetadata {
    // Global Identifier
    pub identifier: String,

    // Authorization Reference - will be encrypted
    pub authorization_reference: Option<String>,
    pub authorization_reference_date: Option<NaiveDateTime>,

    // Originator and Custodian
    pub originator_organization_id: Uuid, // Authority
    pub custodian_organization_id: Uuid, // Authority

    // Format
    pub format: String,
    pub format_size: Option<i64>,

    // Safeguarding and Securing
    pub security_classification: String,

    // Disclosure & Releasability
    pub releasable_to_countries: Option<Vec<Option<String>>>,
    pub releasable_to_organizations: Option<Vec<Option<String>>>,
    pub releasable_to_categories: Option<Vec<Option<String>>>,
    pub disclosure_category: Option<String>,

    // Handling Restrictions
    pub handling_restrictions: Option<Vec<Option<String>>>,
    pub handling_authority: Option<String>,
    pub no_handling_restrictions: Option<bool>,

    // Legacy fields
    pub domain: String,
    pub tags: Vec<Option<String>>,
}

/*
target_variables = '{
  "input": {
    "userId": "123e4567-e89b-12d3-a456-426614174000",
    "authorityId": "987e6543-e21b-12d3-a456-426614174000",
    "dataObject": {
      "title": "Intelligence Assessment: Eastern Europe",
      "description": "Comprehensive intelligence report on regional threats"
    },
    "metadata": {
      "identifier": "770e8400-e29b-41d4-a716-446655440002",
      "originatorOrganizationId": "987e6543-e21b-12d3-a456-426614174000",
      "custodianOrganizationId": "987e6543-e21b-12d3-a456-426614174000",
      "format": "application/pdf",
      "formatSize": 5242880,
      "securityClassification": "SECRET",
      "releasableToCountries": ["USA", "GBR", "FRA"],
      "releasableToOrganizations": ["NATO"],
      "disclosureCategory": "Category B",
      "handlingRestrictions": ["CUI", "NOFORN"],
      "handlingAuthority": "EO 13526",
      "domain": "INTEL",
      "tags": ["regional", "threat-assessment", "strategic"]
    },
    "sourceNationClassification": "SECRET",
    "sourceNationCode": "USA",
    "targetNationCodes": ["GBR", "FRA"]
  }
}'
*/

#[post("/{lang}/submit_document")]
pub async fn submit_document(
    path: web::Path<String>,
    data: web::Data<AppData>,
    req: HttpRequest, 
    form: web::Form<DocumentSubmissionForm>,
    id: Option<Identity>,
) -> impl Responder {

    let lang = path.into_inner();

    let session = req.get_session();

    // validate form has data or re-load form
    if form.content.is_empty() || form.target_nations.is_empty() {
        println!("Form is empty");
        return HttpResponse::Found().append_header(("Location", format!("/{}", &lang))).finish()
    };

    let mut ctx = generate_basic_context(id, &lang, req.uri().path(), &session);

    let bearer = match req.get_session().get::<String>("bearer").unwrap() {
        Some(s) => s,
        None => "".to_string(),
    };

    // Populate conversion_request from form


    // Populate conversion request from LLM


    // Render validation form
             
     ctx.insert("conversion_request", &conversion_request);
     
    let rendered = data.tmpl.render("document_review.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}