use std::sync::Arc;

use actix_web::{HttpRequest, HttpResponse, HttpMessage, Responder, get, post, web};
use actix_session::{SessionExt};
use actix_identity::{Identity};
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

use ollama_rs::{
    Ollama, coordinator::Coordinator, generation::{
        chat::ChatMessage, completion::request::GenerationRequest, parameters::{FormatType, JsonSchema, JsonStructure}
    }, models::ModelOptions
};

use uuid::Uuid;

use crate::{AppData, generate_basic_context, graphql::{self, AuthorityById, get_authority_by_id, submit_conversion_request, user}};

#[derive(Deserialize, Debug, Serialize)]
pub struct DocumentSubmissionForm {

    #[serde(rename = "securityClassificationLevel")]
    pub security_classification_level: String,

    // Target nations for checkbox form processing - NATO members + AUS/NZL
    pub alb: Option<String>,
    pub aus: Option<String>,
    pub bel: Option<String>,
    pub bgr: Option<String>,
    pub can: Option<String>,
    pub hrv: Option<String>,
    pub cze: Option<String>,
    pub dnk: Option<String>,
    pub est: Option<String>,
    pub fin: Option<String>,
    pub fra: Option<String>,
    pub deu: Option<String>,
    pub grc: Option<String>,
    pub hun: Option<String>,
    pub isl: Option<String>,
    pub ita: Option<String>,
    pub lva: Option<String>,
    pub ltu: Option<String>,
    pub lux: Option<String>,
    pub mne: Option<String>,
    pub nld: Option<String>,
    pub nzl: Option<String>,
    pub mkd: Option<String>,
    pub nor: Option<String>,
    pub pol: Option<String>,
    pub prt: Option<String>,
    pub rou: Option<String>,
    pub svk: Option<String>,
    pub svn: Option<String>,
    pub esp: Option<String>,
    pub swe: Option<String>,
    pub tur: Option<String>,
    pub gbr: Option<String>,
    pub usa: Option<String>,

    // Releasable to organizations - checkbox form processing
    pub nato: Option<String>,
    pub eu: Option<String>,
    pub un: Option<String>,
    pub fvey: Option<String>,
    pub aukus: Option<String>,
    pub quad: Option<String>,

    #[serde(rename = "disclosureCategory")]
    pub disclosure_category: String,

    // Handling restrictions - checkbox form processing
    pub cui: Option<String>,
    pub fouo: Option<String>,
    pub les: Option<String>,
    pub sbu: Option<String>,
    pub noforn: Option<String>,
    pub propin: Option<String>,
    pub orcon: Option<String>,

    #[serde(rename = "handlingAuthority")]
    pub handling_authority: String,

    pub content: String,
}
/// The JSON formatted data payload submitted to the API that triggers
/// a security classification conversion
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct InsertableConversionRequest {
    pub user_id: String,
    pub authority_id: String,
    pub data_object: InsertableDataObject,
    pub metadata: InsertableMetadata,
    pub source_nation_classification: String,
    pub source_nation_code: String,
    pub target_nation_codes: Vec<String>,
}

/// A lightweight struct to accept JSON formatted data from a ConversionRequest
/// needed to create a NewDataObject
/// GraphQL input type accepts plain String (will be encrypted internally)
#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct InsertableDataObject {
    pub title: String,        // GraphQL input as plain String
    pub description: String,   // GraphQL input as plain String
}

impl From<InsertableDataObject> for crate::graphql::submit_conversion::DataObjectInput {
    fn from(data: InsertableDataObject) -> Self {
        crate::graphql::submit_conversion::DataObjectInput {
            title: data.title,
            description: data.description,
        }
    }
}

/// A light struct to accept the JSON formatted Metadata included with
/// a ConversionRequest
/// GraphQL input type accepts plain String (will be encrypted internally)
#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
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
    pub domain: Domain,
    pub tags: Vec<Option<String>>,
}

impl From<InsertableMetadata> for crate::graphql::submit_conversion::MetadataInput {
    fn from(metadata: InsertableMetadata) -> Self {
        crate::graphql::submit_conversion::MetadataInput {
            identifier: metadata.identifier,
            authorization_reference: metadata.authorization_reference,
            authorization_reference_date: metadata.authorization_reference_date,
            originator_organization_id: metadata.originator_organization_id.to_string(),
            custodian_organization_id: metadata.custodian_organization_id.to_string(),
            format: metadata.format,
            format_size: metadata.format_size,
            security_classification: metadata.security_classification,
            releasable_to_countries: metadata.releasable_to_countries,
            releasable_to_organizations: metadata.releasable_to_organizations,
            releasable_to_categories: metadata.releasable_to_categories,
            disclosure_category: metadata.disclosure_category,
            handling_restrictions: metadata.handling_restrictions,
            handling_authority: metadata.handling_authority,
            no_handling_restrictions: metadata.no_handling_restrictions,
            domain: format!("{:?}", metadata.domain),
            tags: metadata.tags,
        }
    }
}

#[derive(JsonSchema, Deserialize, Debug, Clone, Serialize)]
pub enum Domain {
    INTEL,
    CYBER,
    OPERATIONS,
    LOGISTICS,
    COMMUNICATIONS,
    NUCLEAR,
    COUNTERTERRORISM,
    MARITIME,
    AEROSPACE,
    SPECIALOPS,
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

    println!("{:?}", &form);

    // validate form has data or re-load form
    if form.content.is_empty() {
        println!("Form is empty");
        return HttpResponse::Found().append_header(("Location", format!("/{}", &lang))).finish()
    };

    let mut ctx = generate_basic_context(id, &lang, req.uri().path(), &session);

    let bearer = match req.get_session().get::<String>("bearer").unwrap() {
        Some(s) => s,
        None => "".to_string(),
    };

    let user_id = match req.get_session().get::<String>("user_id").unwrap() {
        Some(s) => s,
        None => "".to_string(),
    };

    let authority_id = match req.get_session().get::<String>("authority_id").unwrap() {
        Some(s) => s,
        None => "".to_string(),
    };

    let authority = get_authority_by_id(authority_id, bearer.clone(), &data.api_url, Arc::clone(&data.client))
        .await
        .expect("Unable to retrieve authority")
        .authority_by_id;

    // Handle form - target nations
    let mut target_nations = Vec::new();

    if form.alb != None { target_nations.push("ALB".to_owned());};
    if form.aus != None { target_nations.push("AUS".to_owned());};
    if form.bel != None { target_nations.push("BEL".to_owned());};
    if form.bgr != None { target_nations.push("BGR".to_owned());};
    if form.can != None { target_nations.push("CAN".to_owned());};
    if form.hrv != None { target_nations.push("HRV".to_owned());};
    if form.cze != None { target_nations.push("CZE".to_owned());};
    if form.dnk != None { target_nations.push("DNK".to_owned());};
    if form.est != None { target_nations.push("EST".to_owned());};
    if form.fin != None { target_nations.push("FIN".to_owned());};
    if form.fra != None { target_nations.push("FRA".to_owned());};
    if form.deu != None { target_nations.push("DEU".to_owned());};
    if form.grc != None { target_nations.push("GRC".to_owned());};
    if form.hun != None { target_nations.push("HUN".to_owned());};
    if form.isl != None { target_nations.push("ISL".to_owned());};
    if form.ita != None { target_nations.push("ITA".to_owned());};
    if form.lva != None { target_nations.push("LVA".to_owned());};
    if form.ltu != None { target_nations.push("LTU".to_owned());};
    if form.lux != None { target_nations.push("LUX".to_owned());};
    if form.mne != None { target_nations.push("MNE".to_owned());};
    if form.nld != None { target_nations.push("NLD".to_owned());};
    if form.nzl != None { target_nations.push("NZL".to_owned());};
    if form.mkd != None { target_nations.push("MKD".to_owned());};
    if form.nor != None { target_nations.push("NOR".to_owned());};
    if form.pol != None { target_nations.push("POL".to_owned());};
    if form.prt != None { target_nations.push("PRT".to_owned());};
    if form.rou != None { target_nations.push("ROU".to_owned());};
    if form.svk != None { target_nations.push("SVK".to_owned());};
    if form.svn != None { target_nations.push("SVN".to_owned());};
    if form.esp != None { target_nations.push("ESP".to_owned());};
    if form.swe != None { target_nations.push("SWE".to_owned());};
    if form.tur != None { target_nations.push("TUR".to_owned());};
    if form.gbr != None { target_nations.push("GBR".to_owned());};
    if form.usa != None { target_nations.push("USA".to_owned());};

    // Handle form - releasable to organizations
    let mut releasable_orgs = Vec::new();

    if form.nato != None { releasable_orgs.push("NATO".to_owned());};
    if form.eu != None { releasable_orgs.push("EU".to_owned());};
    if form.un != None { releasable_orgs.push("UN".to_owned());};
    if form.fvey != None { releasable_orgs.push("FVEY".to_owned());};
    if form.aukus != None { releasable_orgs.push("AUKUS".to_owned());};
    if form.quad != None { releasable_orgs.push("QUAD".to_owned());};

    // Handle form - handling restrictions
    let mut handling_restrictions = Vec::new();

    if form.cui != None { handling_restrictions.push("CUI".to_owned());};
    if form.fouo != None { handling_restrictions.push("FOUO".to_owned());};
    if form.les != None { handling_restrictions.push("LES".to_owned());};
    if form.sbu != None { handling_restrictions.push("SBU".to_owned());};
    if form.noforn != None { handling_restrictions.push("NOFORN".to_owned());};
    if form.propin != None { handling_restrictions.push("PROPIN".to_owned());};
    if form.orcon != None { handling_restrictions.push("ORCON".to_owned());};

    // Use LLLM to generate DataObject

    let data_obj_format = FormatType::StructuredJson(Box::new(JsonStructure::new::<InsertableDataObject>()));

    let model = "ministral-3:14b".to_owned();
    let prompt = format!("Take the data from document to populate data object: {}", &form.content);

    let ollama = Ollama::default();

    let data_res = ollama
        .generate(
            GenerationRequest::new(
                model, 
                prompt)
        .format(data_obj_format)
        .options(ModelOptions::default().temperature(0.0)),
        )
        .await
        .expect("Unable to retrieve LLM generated content");

    dbg!(&data_res.response);

    let data_struct: InsertableDataObject = serde_json::from_str(&data_res.response)
        .expect("Unable to derive InsertableDataObject from LLM");

    println!("{:?}", &data_struct);

    // Use LLM to Generate Metadata

    let model = "ministral-3:14b".to_owned();
    let prompt = format!("Take the data from this form to populate metadata object: {}", &form.content);

    let metadata_format = FormatType::StructuredJson(Box::new(JsonStructure::new::<InsertableMetadata>()));

    let meta_res = ollama
        .generate(
            GenerationRequest::new(
                model, 
                prompt)
        .format(metadata_format)
        .options(ModelOptions::default().temperature(0.0)),
        )
        .await
        .expect("Unable to retrieve LLM generated content");

    dbg!(&meta_res.response);

    let mut meta_struct: InsertableMetadata = serde_json::from_str(&meta_res.response)
        .expect("Unable to derive InsertableDataObject from LLM");

    meta_struct.custodian_organization_id = Uuid::parse_str(&authority.id).expect("Unable to convert Str to UUID");
    meta_struct.originator_organization_id = Uuid::parse_str(&authority.id).expect("Unable to convert Str to UUID");
    meta_struct.security_classification = form.security_classification_level.clone();

    println!("{:?}", &meta_struct);

    println!("Structured Data Successfully Generated");

    // Integrate into full ConversionRequest 

    let conversion_request = InsertableConversionRequest {
        user_id: user_id,
        authority_id: authority.id,
        data_object: data_struct,
        metadata: meta_struct,
        source_nation_classification: form.security_classification_level.clone(),
        source_nation_code: authority.nation.nation_code,
        target_nation_codes: target_nations,
    };

    println!("{:?}", conversion_request);

    // Submit Form and Await Response
    let response = submit_conversion_request(
        conversion_request, 
        &data.api_url, 
        Arc::clone(&data.client),
        bearer
    )
    .await
    .expect("Unable to get ConversionResponse from server");

    // Generate Response for User
    ctx.insert("conversion_response", &response);
                  
    let rendered = data.tmpl.render("conversion_request/conversion_response.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

