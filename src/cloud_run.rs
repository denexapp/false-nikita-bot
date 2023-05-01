use std::env;

use log::info;
use reqwest::Client;
use serde::Deserialize;
use urlencoding::{decode, encode};

#[derive(Deserialize)]
pub struct Service {
    pub uri: String,
}

pub async fn get_project_id(client: &Client) -> String {
    client
        .get("http://metadata.google.internal/computeMetadata/v1/project/project-id")
        .header("Metadata-Flavor", "Google")
        .send()
        .await
        .expect("A request for project ID should not fail when sending")
        .text()
        .await
        .expect("A request for project ID should not fail when awaiting for text")
}

async fn get_instance_region(client: &Client) -> String {
    decode(
        client
            .get("http://metadata.google.internal/computeMetadata/v1/instance/region")
            .header("Metadata-Flavor", "Google")
            .send()
            .await
            .expect("A request for instance region should not fail when sending")
            .text()
            .await
            .expect("A request for instance region should not fail when awaiting for text")
            .split('/')
            .last()
            .expect("Instance region &str should exist"),
    )
    .expect("Region should be decodeable")
    .to_string()
}

fn get_service_name() -> String {
    env::var("K_SERVICE")
        .expect("K_SERVICE environment variable should have Cloud Run service name")
}

pub async fn get_service_uri(client: &Client, project_id: &str) -> String {
    let instance_region = get_instance_region(client).await;
    let service_name = get_service_name();

    let encoded_project_id = encode(&project_id);
    let encoded_instance_region = encode(&instance_region);
    let encoded_service_name = encode(&service_name);

    let url = format!("https://run.googleapis.com/v2/projects/{encoded_project_id}/locations/{encoded_instance_region}/services/{encoded_service_name}");

    let service = client
        .get(&url)
        .header("Metadata-Flavor", "Google")
        .send()
        .await
        .expect("A request for Service information should not fail when sending")
        .text()
        .await
        .expect("A request for Service information should not fail when awaiting for text");

    info!(
        "Cloud Admin API responded with the following text: {}",
        service
    );

    let service: Service = client
        .get(&url)
        .header("Metadata-Flavor", "Google")
        .send()
        .await
        .expect("A request for Service information should not fail when sending")
        .json()
        .await
        .expect(
            "A request for Service information should not fail when awaiting for json and decoding",
        );

    return service.uri;
}