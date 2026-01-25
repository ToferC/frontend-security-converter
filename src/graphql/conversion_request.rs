use graphql_client::{GraphQLQuery, Response};
use serde::{Serialize, Deserialize};
use std::error::Error;
use reqwest::Client;
use std::sync::Arc;

use chrono::NaiveDateTime;
use serde_json::Value as JSON;

use crate::graphql::conversion_request;
use crate::graphql::submit_conversion::{ConversionRequestInput, DataObjectInput, MetadataInput};

use crate::handlers::conversion_response::InsertableConversionRequest;

type UUID = String;
type JSONObject = serde_json::Value;

#[derive(GraphQLQuery, Serialize, Deserialize)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "queries/conversion_request.graphql",
    response_derives = "Debug, Serialize, PartialEq"
)]
pub struct SubmitConversion; 

pub async fn submit_conversion_request(conversion_request: InsertableConversionRequest, api_url: &str, client: Arc<Client>, bearer: String) -> Result<submit_conversion::ResponseData, Box<dyn Error>> {

    let data_object = DataObjectInput::from(conversion_request.data_object);

    let metadata = MetadataInput::from(conversion_request.metadata);

    let input = conversion_request::ConversionRequestInput{
        user_id: conversion_request.user_id,
        authority_id: conversion_request.authority_id,
        data_object: data_object,
        metadata: metadata,
        source_nation_classification: conversion_request.source_nation_classification,
        source_nation_code: conversion_request.source_nation_code,
        target_nation_codes: conversion_request.target_nation_codes,
    };

    let request_body = SubmitConversion::build_query(submit_conversion::Variables {
        input,
    });

    let res = client
        .post(api_url)
            .header("Authorization", format!("Bearer {}", bearer))
        .json(&request_body)
        .send()
        .await?;

    let response_body: Response<submit_conversion::ResponseData> = res.json().await?;

    if let Some(errors) = response_body.errors {
        println!("there are errors:");

        for error in &errors {
            println!("{:?}", error);
        }
    };

    let response = response_body.data
        .expect("missing response data");

    println!("{:?}", &response);

    // serve HTML page with response_body
    Ok(response)
}

#[derive(GraphQLQuery, Serialize, Deserialize)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "queries/conversion_requests/conversion_request_by_id.graphql",
    response_derives = "Debug, Serialize, PartialEq"
)]
pub struct ConversionRequestById;

pub async fn get_conversion_request_by_id(id: String, bearer: String, api_url: &str, client: Arc<Client>) -> Result<conversion_request_by_id::ResponseData, Box<dyn Error>> {

    let request_body = ConversionRequestById::build_query(conversion_request_by_id::Variables {
        id,
    });

    let res = client
        .post(api_url)
        .header("Bearer", bearer)
        .json(&request_body)
        .send()
        .await?;

    let response_body: Response<conversion_request_by_id::ResponseData> = res.json().await?;

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