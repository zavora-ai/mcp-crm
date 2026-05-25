//! Unified CRM types shared across all backends.
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub id: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub company_id: Option<String>,
    pub company_name: Option<String>,
    pub title: Option<String>,
    pub backend: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Company {
    pub id: String,
    pub name: String,
    pub domain: Option<String>,
    pub industry: Option<String>,
    pub phone: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub backend: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deal {
    pub id: String,
    pub name: String,
    pub stage: Option<String>,
    pub pipeline: Option<String>,
    pub amount: Option<f64>,
    pub currency: Option<String>,
    pub contact_id: Option<String>,
    pub company_id: Option<String>,
    pub close_date: Option<String>,
    pub probability: Option<f64>,
    pub status: Option<String>,
    pub backend: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity {
    pub id: String,
    pub activity_type: String,
    pub subject: Option<String>,
    pub body: Option<String>,
    pub contact_id: Option<String>,
    pub deal_id: Option<String>,
    pub done: bool,
    pub due_date: Option<String>,
    pub backend: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pipeline {
    pub id: String,
    pub name: String,
    pub stages: Vec<PipelineStage>,
    pub backend: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineStage {
    pub id: String,
    pub name: String,
    pub order: u32,
    pub deal_count: Option<u32>,
    pub total_value: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: String,
    pub content: String,
    pub contact_id: Option<String>,
    pub company_id: Option<String>,
    pub deal_id: Option<String>,
    pub created_at: Option<String>,
    pub backend: String,
}

/// Backend trait — each CRM implements this.
#[async_trait::async_trait]
pub trait CrmBackend: Send + Sync {
    fn name(&self) -> &str;

    // Contacts
    async fn list_contacts(&self, limit: u32) -> anyhow::Result<Vec<Contact>>;
    async fn get_contact(&self, id: &str) -> anyhow::Result<Contact>;
    async fn create_contact(&self, first_name: &str, last_name: &str, email: Option<&str>, phone: Option<&str>, company_id: Option<&str>) -> anyhow::Result<Contact>;
    async fn update_contact(&self, id: &str, first_name: Option<&str>, last_name: Option<&str>, email: Option<&str>, phone: Option<&str>) -> anyhow::Result<Contact>;
    async fn search_contacts(&self, query: &str, limit: u32) -> anyhow::Result<Vec<Contact>>;

    // Companies
    async fn list_companies(&self, limit: u32) -> anyhow::Result<Vec<Company>>;
    async fn get_company(&self, id: &str) -> anyhow::Result<Company>;
    async fn create_company(&self, name: &str, domain: Option<&str>, industry: Option<&str>) -> anyhow::Result<Company>;
    async fn update_company(&self, id: &str, name: Option<&str>, domain: Option<&str>, industry: Option<&str>) -> anyhow::Result<Company>;

    // Deals
    async fn list_deals(&self, limit: u32) -> anyhow::Result<Vec<Deal>>;
    async fn get_deal(&self, id: &str) -> anyhow::Result<Deal>;
    async fn create_deal(&self, name: &str, amount: Option<f64>, stage: Option<&str>, contact_id: Option<&str>, company_id: Option<&str>) -> anyhow::Result<Deal>;
    async fn update_deal(&self, id: &str, name: Option<&str>, amount: Option<f64>, stage: Option<&str>, close_date: Option<&str>) -> anyhow::Result<Deal>;
    async fn move_deal_stage(&self, id: &str, stage: &str) -> anyhow::Result<Deal>;

    // Activities
    async fn list_activities(&self, contact_id: Option<&str>, deal_id: Option<&str>, limit: u32) -> anyhow::Result<Vec<Activity>>;
    async fn create_activity(&self, activity_type: &str, subject: &str, body: Option<&str>, contact_id: Option<&str>, deal_id: Option<&str>) -> anyhow::Result<Activity>;

    // Pipelines
    async fn list_pipelines(&self) -> anyhow::Result<Vec<Pipeline>>;
    async fn get_pipeline_summary(&self, pipeline_id: Option<&str>) -> anyhow::Result<Vec<Pipeline>>;

    // Notes
    async fn list_notes(&self, contact_id: Option<&str>, deal_id: Option<&str>, limit: u32) -> anyhow::Result<Vec<Note>>;
    async fn create_note(&self, content: &str, contact_id: Option<&str>, company_id: Option<&str>, deal_id: Option<&str>) -> anyhow::Result<Note>;
}
