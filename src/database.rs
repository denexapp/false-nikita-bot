use firestore::{errors::FirestoreError, FirestoreDb};
use serde::{Deserialize, Serialize};

const VIDEO_JOKES_COLLECTION: &str = "video_notes";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VideoJoke {
    pub file_ids: Vec<String>,
}

pub async fn get_database(project_id: &String) -> FirestoreDb {
    FirestoreDb::new(project_id)
        .await
        .expect("FirestoreDb::new should return database client")
}

pub async fn get_video_jokes(db: &FirestoreDb) -> Result<Vec<VideoJoke>, FirestoreError> {
    Ok(db
        .fluent()
        .select()
        .from(VIDEO_JOKES_COLLECTION)
        .obj()
        .query()
        .await?)
}
