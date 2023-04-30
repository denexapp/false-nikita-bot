use firestore::{errors::FirestoreError, FirestoreDb};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::consts;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TextMessage {
    pub text: String,
}

pub async fn get_database(client: &Client) -> FirestoreDb {
    let project_id = client
        .get("http://metadata.google.internal/computeMetadata/v1/project/project-id")
        .header("Metadata-Flavor", "Google")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    FirestoreDb::new(project_id).await.unwrap()
}

pub async fn get_text_messages(db: &FirestoreDb) -> Result<Vec<TextMessage>, FirestoreError> {
    Ok(db
        .fluent()
        .select()
        .from(consts::TEXT_MESSAGES_COLLECTION)
        .obj()
        .query()
        .await?)
}
