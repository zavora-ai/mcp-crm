//! MCP tool router for CRM operations.
use adk_mcp_sdk::{HealthCheck, HealthStatus};
use crate::types::CrmBackend;
use rmcp::{handler::server::wrapper::Parameters, schemars, tool, tool_router};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ListInput { #[serde(default = "d20")] pub limit: u32 }
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct IdInput { pub id: String }
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SearchInput { pub query: String, #[serde(default = "d20")] pub limit: u32 }
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CreateContactInput { pub first_name: String, pub last_name: String, #[serde(default)] pub email: Option<String>, #[serde(default)] pub phone: Option<String>, #[serde(default)] pub company_id: Option<String> }
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct UpdateContactInput { pub id: String, #[serde(default)] pub first_name: Option<String>, #[serde(default)] pub last_name: Option<String>, #[serde(default)] pub email: Option<String>, #[serde(default)] pub phone: Option<String> }
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CreateCompanyInput { pub name: String, #[serde(default)] pub domain: Option<String>, #[serde(default)] pub industry: Option<String> }
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct UpdateCompanyInput { pub id: String, #[serde(default)] pub name: Option<String>, #[serde(default)] pub domain: Option<String>, #[serde(default)] pub industry: Option<String> }
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CreateDealInput { pub name: String, #[serde(default)] pub amount: Option<f64>, #[serde(default)] pub stage: Option<String>, #[serde(default)] pub contact_id: Option<String>, #[serde(default)] pub company_id: Option<String> }
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct UpdateDealInput { pub id: String, #[serde(default)] pub name: Option<String>, #[serde(default)] pub amount: Option<f64>, #[serde(default)] pub stage: Option<String>, #[serde(default)] pub close_date: Option<String> }
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct MoveDealInput { pub id: String, pub stage: String }
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ListActivitiesInput { #[serde(default)] pub contact_id: Option<String>, #[serde(default)] pub deal_id: Option<String>, #[serde(default = "d20")] pub limit: u32 }
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CreateActivityInput { pub activity_type: String, pub subject: String, #[serde(default)] pub body: Option<String>, #[serde(default)] pub contact_id: Option<String>, #[serde(default)] pub deal_id: Option<String> }
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct PipelineInput { #[serde(default)] pub pipeline_id: Option<String> }
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ListNotesInput { #[serde(default)] pub contact_id: Option<String>, #[serde(default)] pub deal_id: Option<String>, #[serde(default = "d20")] pub limit: u32 }
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CreateNoteInput { pub content: String, #[serde(default)] pub contact_id: Option<String>, #[serde(default)] pub company_id: Option<String>, #[serde(default)] pub deal_id: Option<String> }
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct AssociateInput { pub contact_id: String, pub company_id: String }
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DealContactInput { pub deal_id: String, pub contact_id: String }
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct UpdateActivityInput { pub id: String, #[serde(default)] pub done: Option<bool>, #[serde(default)] pub subject: Option<String> }

fn d20() -> u32 { 20 }

#[derive(Clone)]
pub struct CrmServer { pub backend: Arc<dyn CrmBackend> }

#[tool_router(server_handler)]
impl CrmServer {
    #[tool(description = "List contacts with optional search query")]
    async fn list_contacts(&self, Parameters(i): Parameters<ListInput>) -> String {
        match self.backend.list_contacts(i.limit).await { Ok(v) => serde_json::to_string_pretty(&v).unwrap(), Err(e) => format!("Error: {e}") }
    }
    #[tool(description = "Get a contact by ID")]
    async fn get_contact(&self, Parameters(i): Parameters<IdInput>) -> String {
        match self.backend.get_contact(&i.id).await { Ok(v) => serde_json::to_string_pretty(&v).unwrap(), Err(e) => format!("Error: {e}") }
    }
    #[tool(description = "Create a new contact")]
    async fn create_contact(&self, Parameters(i): Parameters<CreateContactInput>) -> String {
        match self.backend.create_contact(&i.first_name, &i.last_name, i.email.as_deref(), i.phone.as_deref(), i.company_id.as_deref()).await { Ok(v) => serde_json::to_string_pretty(&v).unwrap(), Err(e) => format!("Error: {e}") }
    }
    #[tool(description = "Update an existing contact")]
    async fn update_contact(&self, Parameters(i): Parameters<UpdateContactInput>) -> String {
        match self.backend.update_contact(&i.id, i.first_name.as_deref(), i.last_name.as_deref(), i.email.as_deref(), i.phone.as_deref()).await { Ok(v) => serde_json::to_string_pretty(&v).unwrap(), Err(e) => format!("Error: {e}") }
    }
    #[tool(description = "Search contacts by name, email, or custom query")]
    async fn search_contacts(&self, Parameters(i): Parameters<SearchInput>) -> String {
        match self.backend.search_contacts(&i.query, i.limit).await { Ok(v) => serde_json::to_string_pretty(&v).unwrap(), Err(e) => format!("Error: {e}") }
    }
    #[tool(description = "List companies/accounts")]
    async fn list_companies(&self, Parameters(i): Parameters<ListInput>) -> String {
        match self.backend.list_companies(i.limit).await { Ok(v) => serde_json::to_string_pretty(&v).unwrap(), Err(e) => format!("Error: {e}") }
    }
    #[tool(description = "Get a company by ID")]
    async fn get_company(&self, Parameters(i): Parameters<IdInput>) -> String {
        match self.backend.get_company(&i.id).await { Ok(v) => serde_json::to_string_pretty(&v).unwrap(), Err(e) => format!("Error: {e}") }
    }
    #[tool(description = "Create a new company/account")]
    async fn create_company(&self, Parameters(i): Parameters<CreateCompanyInput>) -> String {
        match self.backend.create_company(&i.name, i.domain.as_deref(), i.industry.as_deref()).await { Ok(v) => serde_json::to_string_pretty(&v).unwrap(), Err(e) => format!("Error: {e}") }
    }
    #[tool(description = "Update an existing company")]
    async fn update_company(&self, Parameters(i): Parameters<UpdateCompanyInput>) -> String {
        match self.backend.update_company(&i.id, i.name.as_deref(), i.domain.as_deref(), i.industry.as_deref()).await { Ok(v) => serde_json::to_string_pretty(&v).unwrap(), Err(e) => format!("Error: {e}") }
    }
    #[tool(description = "List deals/opportunities")]
    async fn list_deals(&self, Parameters(i): Parameters<ListInput>) -> String {
        match self.backend.list_deals(i.limit).await { Ok(v) => serde_json::to_string_pretty(&v).unwrap(), Err(e) => format!("Error: {e}") }
    }
    #[tool(description = "Get a deal by ID with stage and value")]
    async fn get_deal(&self, Parameters(i): Parameters<IdInput>) -> String {
        match self.backend.get_deal(&i.id).await { Ok(v) => serde_json::to_string_pretty(&v).unwrap(), Err(e) => format!("Error: {e}") }
    }
    #[tool(description = "Create a new deal/opportunity")]
    async fn create_deal(&self, Parameters(i): Parameters<CreateDealInput>) -> String {
        match self.backend.create_deal(&i.name, i.amount, i.stage.as_deref(), i.contact_id.as_deref(), i.company_id.as_deref()).await { Ok(v) => serde_json::to_string_pretty(&v).unwrap(), Err(e) => format!("Error: {e}") }
    }
    #[tool(description = "Update a deal (stage, value, close date)")]
    async fn update_deal(&self, Parameters(i): Parameters<UpdateDealInput>) -> String {
        match self.backend.update_deal(&i.id, i.name.as_deref(), i.amount, i.stage.as_deref(), i.close_date.as_deref()).await { Ok(v) => serde_json::to_string_pretty(&v).unwrap(), Err(e) => format!("Error: {e}") }
    }
    #[tool(description = "Move a deal to a different pipeline stage")]
    async fn move_deal_stage(&self, Parameters(i): Parameters<MoveDealInput>) -> String {
        match self.backend.move_deal_stage(&i.id, &i.stage).await { Ok(v) => serde_json::to_string_pretty(&v).unwrap(), Err(e) => format!("Error: {e}") }
    }
    #[tool(description = "List activities (calls, emails, meetings, tasks) for a record")]
    async fn list_activities(&self, Parameters(i): Parameters<ListActivitiesInput>) -> String {
        match self.backend.list_activities(i.contact_id.as_deref(), i.deal_id.as_deref(), i.limit).await { Ok(v) => serde_json::to_string_pretty(&v).unwrap(), Err(e) => format!("Error: {e}") }
    }
    #[tool(description = "Log an activity (call, email, meeting, task, note)")]
    async fn create_activity(&self, Parameters(i): Parameters<CreateActivityInput>) -> String {
        match self.backend.create_activity(&i.activity_type, &i.subject, i.body.as_deref(), i.contact_id.as_deref(), i.deal_id.as_deref()).await { Ok(v) => serde_json::to_string_pretty(&v).unwrap(), Err(e) => format!("Error: {e}") }
    }
    #[tool(description = "List sales pipelines and their stages")]
    async fn list_pipelines(&self) -> String {
        match self.backend.list_pipelines().await { Ok(v) => serde_json::to_string_pretty(&v).unwrap(), Err(e) => format!("Error: {e}") }
    }
    #[tool(description = "Get pipeline summary with deal counts and values per stage")]
    async fn get_pipeline_summary(&self, Parameters(i): Parameters<PipelineInput>) -> String {
        match self.backend.get_pipeline_summary(i.pipeline_id.as_deref()).await { Ok(v) => serde_json::to_string_pretty(&v).unwrap(), Err(e) => format!("Error: {e}") }
    }
    #[tool(description = "List notes attached to a contact, company, or deal")]
    async fn list_notes(&self, Parameters(i): Parameters<ListNotesInput>) -> String {
        match self.backend.list_notes(i.contact_id.as_deref(), i.deal_id.as_deref(), i.limit).await { Ok(v) => serde_json::to_string_pretty(&v).unwrap(), Err(e) => format!("Error: {e}") }
    }
    #[tool(description = "Add a note to a contact, company, or deal")]
    async fn create_note(&self, Parameters(i): Parameters<CreateNoteInput>) -> String {
        match self.backend.create_note(&i.content, i.contact_id.as_deref(), i.company_id.as_deref(), i.deal_id.as_deref()).await { Ok(v) => serde_json::to_string_pretty(&v).unwrap(), Err(e) => format!("Error: {e}") }
    }
    #[tool(description = "Delete a contact by ID")]
    async fn delete_contact(&self, Parameters(i): Parameters<IdInput>) -> String {
        match self.backend.delete_contact(&i.id).await { Ok(()) => "Contact deleted".into(), Err(e) => format!("Error: {e}") }
    }
    #[tool(description = "Delete a deal by ID")]
    async fn delete_deal(&self, Parameters(i): Parameters<IdInput>) -> String {
        match self.backend.delete_deal(&i.id).await { Ok(()) => "Deal deleted".into(), Err(e) => format!("Error: {e}") }
    }
    #[tool(description = "Associate a contact with a company")]
    async fn associate_contact_company(&self, Parameters(i): Parameters<AssociateInput>) -> String {
        match self.backend.associate_contact_company(&i.contact_id, &i.company_id).await { Ok(()) => "Associated".into(), Err(e) => format!("Error: {e}") }
    }
    #[tool(description = "Associate a deal with a contact")]
    async fn associate_deal_contact(&self, Parameters(i): Parameters<DealContactInput>) -> String {
        match self.backend.associate_deal_contact(&i.deal_id, &i.contact_id).await { Ok(()) => "Associated".into(), Err(e) => format!("Error: {e}") }
    }
    #[tool(description = "List contacts associated with a deal")]
    async fn list_deal_contacts(&self, Parameters(i): Parameters<IdInput>) -> String {
        match self.backend.list_deal_contacts(&i.id).await { Ok(v) => serde_json::to_string_pretty(&v).unwrap(), Err(e) => format!("Error: {e}") }
    }
    #[tool(description = "Search companies by name or domain")]
    async fn search_companies(&self, Parameters(i): Parameters<SearchInput>) -> String {
        match self.backend.search_companies(&i.query, i.limit).await { Ok(v) => serde_json::to_string_pretty(&v).unwrap(), Err(e) => format!("Error: {e}") }
    }
    #[tool(description = "Search deals by name, stage, or value")]
    async fn search_deals(&self, Parameters(i): Parameters<SearchInput>) -> String {
        match self.backend.search_deals(&i.query, i.limit).await { Ok(v) => serde_json::to_string_pretty(&v).unwrap(), Err(e) => format!("Error: {e}") }
    }
    #[tool(description = "Update an activity (mark done, change subject)")]
    async fn update_activity(&self, Parameters(i): Parameters<UpdateActivityInput>) -> String {
        match self.backend.update_activity(&i.id, i.done, i.subject.as_deref()).await { Ok(v) => serde_json::to_string_pretty(&v).unwrap(), Err(e) => format!("Error: {e}") }
    }
}

#[async_trait::async_trait]
impl HealthCheck for CrmServer {
    async fn check_health(&self) -> HealthStatus {
        match self.backend.list_contacts(1).await {
            Ok(_) => HealthStatus { healthy: true, message: Some(format!("{} connected", self.backend.name())), latency_ms: Some(1) },
            Err(e) => HealthStatus { healthy: false, message: Some(format!("{}: {e}", self.backend.name())), latency_ms: None },
        }
    }
}
