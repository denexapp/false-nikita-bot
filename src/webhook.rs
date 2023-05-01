use reqwest::{Client, Url};

use crate::cloud_run::get_service_uri;

pub async fn get_webhook_url(client: &Client, project_id: &String) -> Url {
    let service_uri = get_service_uri(&client, &project_id).await;

    let url: Url = service_uri
        .parse()
        .expect("service_uri should be correct url");
    let url = url
        .join("/webhook")
        .expect("/webhook should join url without errors");

    return url;
}
