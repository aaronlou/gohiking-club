/// Photo filter parameters for list queries.
#[derive(Debug, serde::Deserialize)]
pub struct PhotoFilter {
    pub status: Option<String>,
    pub min_score: Option<f64>,
    pub user_id: Option<uuid::Uuid>,
    pub event_id: Option<uuid::Uuid>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
