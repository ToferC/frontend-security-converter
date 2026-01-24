use graphql_client::{GraphQLQuery, Response};
use serde::{Serialize, Deserialize};
use std::error::Error;
use reqwest::Client;
use std::sync::Arc;
use chrono::NaiveDateTime;

type UUID = String;

#[derive(GraphQLQuery, Serialize, Deserialize)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "queries/metadata/metadata_by_tag.graphql",
    response_derives = "Debug, Serialize, PartialEq"
)]
pub struct MetadataByTag;

pub async fn get_metadata_by_tag(tag: String, bearer: String, api_url: &str, client: Arc<Client>) -> Result<metadata_by_tag::ResponseData, Box<dyn Error>> {

    let request_body = MetadataByTag::build_query(metadata_by_tag::Variables {
        tag,
    });

    let res = client
        .post(api_url)
        .header("Bearer", bearer)
        .json(&request_body)
        .send()
        .await?;

    let response_body: Response<metadata_by_tag::ResponseData> = res.json().await?;

    if let Some(errors) = response_body.errors {
        println!("there are errors:");

        for error in &errors {
            println!("{:?}", error);
        }
    };

    let response = response_body.data
        .expect("missing response data");

    Ok(response)
}

#[derive(GraphQLQuery, Serialize, Deserialize)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "queries/metadata/search_metadata_by_tag.graphql",
    response_derives = "Debug, Serialize, PartialEq"
)]
pub struct SearchMetadataByTag;

pub async fn search_metadata_by_tag(pattern: String, bearer: String, api_url: &str, client: Arc<Client>) -> Result<search_metadata_by_tag::ResponseData, Box<dyn Error>> {

    let request_body = SearchMetadataByTag::build_query(search_metadata_by_tag::Variables {
        pattern,
    });

    let res = client
        .post(api_url)
        .header("Bearer", bearer)
        .json(&request_body)
        .send()
        .await?;

    let response_body: Response<search_metadata_by_tag::ResponseData> = res.json().await?;

    if let Some(errors) = response_body.errors {
        println!("there are errors:");

        for error in &errors {
            println!("{:?}", error);
        }
    };

    let response = response_body.data
        .expect("missing response data");

    Ok(response)
}

#[derive(GraphQLQuery, Serialize, Deserialize)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "queries/metadata/metadata_by_domain.graphql",
    response_derives = "Debug, Serialize, PartialEq"
)]
pub struct MetadataByDomain;

pub async fn get_metadata_by_domain(domain: String, bearer: String, api_url: &str, client: Arc<Client>) -> Result<metadata_by_domain::ResponseData, Box<dyn Error>> {

    let request_body = MetadataByDomain::build_query(metadata_by_domain::Variables {
        domain,
    });

    let res = client
        .post(api_url)
        .header("Bearer", bearer)
        .json(&request_body)
        .send()
        .await?;

    let response_body: Response<metadata_by_domain::ResponseData> = res.json().await?;

    if let Some(errors) = response_body.errors {
        println!("there are errors:");

        for error in &errors {
            println!("{:?}", error);
        }
    };

    let response = response_body.data
        .expect("missing response data");

    Ok(response)
}
