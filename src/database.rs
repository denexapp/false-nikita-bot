use firestore::{errors::FirestoreError, FirestoreDb};
use reqwest::Client;
use serde::{Deserialize, Serialize};

const TEXT_MESSAGES_COLLECTION: &str = "text_messages";
const VIDEO_NOTES_COLLECTION: &str = "video_notes";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TextMessage {
    pub text: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VideoNote {
    pub file_id: String,
}

pub async fn get_database(project_id: &String) -> FirestoreDb {
    FirestoreDb::new(project_id)
        .await
        .expect("FirestoreDb::new should return database client")
}

pub async fn get_text_messages(db: &FirestoreDb) -> Result<Vec<TextMessage>, FirestoreError> {
    Ok(db
        .fluent()
        .select()
        .from(TEXT_MESSAGES_COLLECTION)
        .obj()
        .query()
        .await?)
}

pub async fn get_video_notes(db: &FirestoreDb) -> Result<Vec<VideoNote>, FirestoreError> {
    Ok(db
        .fluent()
        .select()
        .from(VIDEO_NOTES_COLLECTION)
        .obj()
        .query()
        .await?)
}
