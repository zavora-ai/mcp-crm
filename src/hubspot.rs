//! HubSpot CRM API v3 backend.
use crate::types::*;
use anyhow::Result;
use reqwest::Client;

const BASE: &str = "https://api.hubapi.com";

#[derive(Clone)]
pub struct HubSpotBackend { http: Client, token: String }

impl HubSpotBackend {
    pub fn new(token: String) -> Self { Self { http: Client::new(), token } }
    async fn get(&self, path: &str) -> Result<serde_json::Value> {
        Ok(self.http.get(format!("{BASE}/{path}")).bearer_auth(&self.token).send().await?.error_for_status()?.json().await?)
    }
    async fn post(&self, path: &str, body: &serde_json::Value) -> Result<serde_json::Value> {
        Ok(self.http.post(format!("{BASE}/{path}")).bearer_auth(&self.token).json(body).send().await?.error_for_status()?.json().await?)
    }
    async fn patch(&self, path: &str, body: &serde_json::Value) -> Result<serde_json::Value> {
        Ok(self.http.patch(format!("{BASE}/{path}")).bearer_auth(&self.token).json(body).send().await?.error_for_status()?.json().await?)
    }
}

fn hs_prop(v: &serde_json::Value, k: &str) -> Option<String> { v["properties"][k].as_str().filter(|s| !s.is_empty()).map(Into::into) }

#[async_trait::async_trait]
impl CrmBackend for HubSpotBackend {
    fn name(&self) -> &str { "hubspot" }

    async fn list_contacts(&self, limit: u32) -> Result<Vec<Contact>> {
        let resp = self.get(&format!("crm/v3/objects/contacts?limit={limit}&properties=firstname,lastname,email,phone,company,jobtitle")).await?;
        Ok(resp["results"].as_array().map(|a| a.iter().map(|c| Contact { id: c["id"].as_str().unwrap_or("").into(), first_name: hs_prop(c, "firstname"), last_name: hs_prop(c, "lastname"), email: hs_prop(c, "email"), phone: hs_prop(c, "phone"), company_id: None, company_name: hs_prop(c, "company"), title: hs_prop(c, "jobtitle"), backend: "hubspot".into() }).collect()).unwrap_or_default())
    }
    async fn get_contact(&self, id: &str) -> Result<Contact> {
        let c = self.get(&format!("crm/v3/objects/contacts/{id}?properties=firstname,lastname,email,phone,company,jobtitle")).await?;
        Ok(Contact { id: c["id"].as_str().unwrap_or("").into(), first_name: hs_prop(&c, "firstname"), last_name: hs_prop(&c, "lastname"), email: hs_prop(&c, "email"), phone: hs_prop(&c, "phone"), company_id: None, company_name: hs_prop(&c, "company"), title: hs_prop(&c, "jobtitle"), backend: "hubspot".into() })
    }
    async fn create_contact(&self, first_name: &str, last_name: &str, email: Option<&str>, phone: Option<&str>, _company_id: Option<&str>) -> Result<Contact> {
        let mut props = serde_json::json!({"firstname": first_name, "lastname": last_name});
        if let Some(e) = email { props["email"] = e.into(); }
        if let Some(p) = phone { props["phone"] = p.into(); }
        let resp = self.post("crm/v3/objects/contacts", &serde_json::json!({"properties": props})).await?;
        Ok(Contact { id: resp["id"].as_str().unwrap_or("").into(), first_name: Some(first_name.into()), last_name: Some(last_name.into()), email: email.map(Into::into), phone: phone.map(Into::into), company_id: None, company_name: None, title: None, backend: "hubspot".into() })
    }
    async fn update_contact(&self, id: &str, first_name: Option<&str>, last_name: Option<&str>, email: Option<&str>, phone: Option<&str>) -> Result<Contact> {
        let mut props = serde_json::json!({});
        if let Some(f) = first_name { props["firstname"] = f.into(); }
        if let Some(l) = last_name { props["lastname"] = l.into(); }
        if let Some(e) = email { props["email"] = e.into(); }
        if let Some(p) = phone { props["phone"] = p.into(); }
        self.patch(&format!("crm/v3/objects/contacts/{id}"), &serde_json::json!({"properties": props})).await?;
        self.get_contact(id).await
    }
    async fn search_contacts(&self, query: &str, limit: u32) -> Result<Vec<Contact>> {
        let body = serde_json::json!({"query": query, "limit": limit, "properties": ["firstname", "lastname", "email", "phone"]});
        let resp = self.post("crm/v3/objects/contacts/search", &body).await?;
        Ok(resp["results"].as_array().map(|a| a.iter().map(|c| Contact { id: c["id"].as_str().unwrap_or("").into(), first_name: hs_prop(c, "firstname"), last_name: hs_prop(c, "lastname"), email: hs_prop(c, "email"), phone: hs_prop(c, "phone"), company_id: None, company_name: None, title: None, backend: "hubspot".into() }).collect()).unwrap_or_default())
    }
    async fn list_companies(&self, limit: u32) -> Result<Vec<Company>> {
        let resp = self.get(&format!("crm/v3/objects/companies?limit={limit}&properties=name,domain,industry,phone,city,country")).await?;
        Ok(resp["results"].as_array().map(|a| a.iter().map(|c| Company { id: c["id"].as_str().unwrap_or("").into(), name: hs_prop(c, "name").unwrap_or_default(), domain: hs_prop(c, "domain"), industry: hs_prop(c, "industry"), phone: hs_prop(c, "phone"), city: hs_prop(c, "city"), country: hs_prop(c, "country"), backend: "hubspot".into() }).collect()).unwrap_or_default())
    }
    async fn get_company(&self, id: &str) -> Result<Company> {
        let c = self.get(&format!("crm/v3/objects/companies/{id}?properties=name,domain,industry,phone,city,country")).await?;
        Ok(Company { id: c["id"].as_str().unwrap_or("").into(), name: hs_prop(&c, "name").unwrap_or_default(), domain: hs_prop(&c, "domain"), industry: hs_prop(&c, "industry"), phone: hs_prop(&c, "phone"), city: hs_prop(&c, "city"), country: hs_prop(&c, "country"), backend: "hubspot".into() })
    }
    async fn create_company(&self, name: &str, domain: Option<&str>, industry: Option<&str>) -> Result<Company> {
        let mut props = serde_json::json!({"name": name});
        if let Some(d) = domain { props["domain"] = d.into(); }
        if let Some(i) = industry { props["industry"] = i.into(); }
        let resp = self.post("crm/v3/objects/companies", &serde_json::json!({"properties": props})).await?;
        Ok(Company { id: resp["id"].as_str().unwrap_or("").into(), name: name.into(), domain: domain.map(Into::into), industry: industry.map(Into::into), phone: None, city: None, country: None, backend: "hubspot".into() })
    }
    async fn update_company(&self, id: &str, name: Option<&str>, domain: Option<&str>, industry: Option<&str>) -> Result<Company> {
        let mut props = serde_json::json!({});
        if let Some(n) = name { props["name"] = n.into(); }
        if let Some(d) = domain { props["domain"] = d.into(); }
        if let Some(i) = industry { props["industry"] = i.into(); }
        self.patch(&format!("crm/v3/objects/companies/{id}"), &serde_json::json!({"properties": props})).await?;
        self.get_company(id).await
    }
    async fn list_deals(&self, limit: u32) -> Result<Vec<Deal>> {
        let resp = self.get(&format!("crm/v3/objects/deals?limit={limit}&properties=dealname,dealstage,pipeline,amount,closedate,hs_deal_stage_probability")).await?;
        Ok(resp["results"].as_array().map(|a| a.iter().map(|d| Deal { id: d["id"].as_str().unwrap_or("").into(), name: hs_prop(d, "dealname").unwrap_or_default(), stage: hs_prop(d, "dealstage"), pipeline: hs_prop(d, "pipeline"), amount: hs_prop(d, "amount").and_then(|s| s.parse().ok()), currency: None, contact_id: None, company_id: None, close_date: hs_prop(d, "closedate"), probability: hs_prop(d, "hs_deal_stage_probability").and_then(|s| s.parse().ok()), status: None, backend: "hubspot".into() }).collect()).unwrap_or_default())
    }
    async fn get_deal(&self, id: &str) -> Result<Deal> {
        let d = self.get(&format!("crm/v3/objects/deals/{id}?properties=dealname,dealstage,pipeline,amount,closedate")).await?;
        Ok(Deal { id: d["id"].as_str().unwrap_or("").into(), name: hs_prop(&d, "dealname").unwrap_or_default(), stage: hs_prop(&d, "dealstage"), pipeline: hs_prop(&d, "pipeline"), amount: hs_prop(&d, "amount").and_then(|s| s.parse().ok()), currency: None, contact_id: None, company_id: None, close_date: hs_prop(&d, "closedate"), probability: None, status: None, backend: "hubspot".into() })
    }
    async fn create_deal(&self, name: &str, amount: Option<f64>, stage: Option<&str>, _contact_id: Option<&str>, _company_id: Option<&str>) -> Result<Deal> {
        let mut props = serde_json::json!({"dealname": name});
        if let Some(a) = amount { props["amount"] = a.to_string().into(); }
        if let Some(s) = stage { props["dealstage"] = s.into(); }
        let resp = self.post("crm/v3/objects/deals", &serde_json::json!({"properties": props})).await?;
        self.get_deal(resp["id"].as_str().unwrap_or("")).await
    }
    async fn update_deal(&self, id: &str, name: Option<&str>, amount: Option<f64>, stage: Option<&str>, close_date: Option<&str>) -> Result<Deal> {
        let mut props = serde_json::json!({});
        if let Some(n) = name { props["dealname"] = n.into(); }
        if let Some(a) = amount { props["amount"] = a.to_string().into(); }
        if let Some(s) = stage { props["dealstage"] = s.into(); }
        if let Some(d) = close_date { props["closedate"] = d.into(); }
        self.patch(&format!("crm/v3/objects/deals/{id}"), &serde_json::json!({"properties": props})).await?;
        self.get_deal(id).await
    }
    async fn move_deal_stage(&self, id: &str, stage: &str) -> Result<Deal> {
        self.patch(&format!("crm/v3/objects/deals/{id}"), &serde_json::json!({"properties": {"dealstage": stage}})).await?;
        self.get_deal(id).await
    }
    async fn list_activities(&self, contact_id: Option<&str>, deal_id: Option<&str>, limit: u32) -> Result<Vec<Activity>> {
        let path = match (contact_id, deal_id) {
            (Some(c), _) => format!("crm/v3/objects/contacts/{c}/associations/engagements?limit={limit}"),
            (_, Some(d)) => format!("crm/v3/objects/deals/{d}/associations/engagements?limit={limit}"),
            _ => format!("crm/v3/objects/engagements?limit={limit}"),
        };
        let resp = self.get(&path).await.unwrap_or(serde_json::json!({"results": []}));
        Ok(resp["results"].as_array().map(|a| a.iter().map(|e| Activity { id: e["id"].as_str().unwrap_or("").into(), activity_type: "engagement".into(), subject: None, body: None, contact_id: contact_id.map(Into::into), deal_id: deal_id.map(Into::into), done: false, due_date: None, backend: "hubspot".into() }).collect()).unwrap_or_default())
    }
    async fn create_activity(&self, activity_type: &str, subject: &str, body: Option<&str>, contact_id: Option<&str>, deal_id: Option<&str>) -> Result<Activity> {
        let mut props = serde_json::json!({"hs_activity_type": activity_type, "hs_task_subject": subject});
        if let Some(b) = body { props["hs_task_body"] = b.into(); }
        let resp = self.post("crm/v3/objects/tasks", &serde_json::json!({"properties": props})).await?;
        let id = resp["id"].as_str().unwrap_or("").to_string();
        if let Some(c) = contact_id { let _ = self.post(&format!("crm/v3/objects/tasks/{id}/associations/contacts/{c}/task_to_contact"), &serde_json::json!({})).await; }
        if let Some(d) = deal_id { let _ = self.post(&format!("crm/v3/objects/tasks/{id}/associations/deals/{d}/task_to_deal"), &serde_json::json!({})).await; }
        Ok(Activity { id, activity_type: activity_type.into(), subject: Some(subject.into()), body: body.map(Into::into), contact_id: contact_id.map(Into::into), deal_id: deal_id.map(Into::into), done: false, due_date: None, backend: "hubspot".into() })
    }
    async fn list_pipelines(&self) -> Result<Vec<Pipeline>> {
        let resp = self.get("crm/v3/pipelines/deals").await?;
        Ok(resp["results"].as_array().map(|a| a.iter().map(|p| Pipeline { id: p["id"].as_str().unwrap_or("").into(), name: p["label"].as_str().unwrap_or("").into(), stages: p["stages"].as_array().map(|s| s.iter().enumerate().map(|(i, st)| PipelineStage { id: st["id"].as_str().unwrap_or("").into(), name: st["label"].as_str().unwrap_or("").into(), order: i as u32, deal_count: None, total_value: None }).collect()).unwrap_or_default(), backend: "hubspot".into() }).collect()).unwrap_or_default())
    }
    async fn get_pipeline_summary(&self, _pipeline_id: Option<&str>) -> Result<Vec<Pipeline>> { self.list_pipelines().await }
    async fn list_notes(&self, contact_id: Option<&str>, deal_id: Option<&str>, limit: u32) -> Result<Vec<Note>> {
        let resp = self.get(&format!("crm/v3/objects/notes?limit={limit}&properties=hs_note_body,hs_createdate")).await?;
        Ok(resp["results"].as_array().map(|a| a.iter().map(|n| Note { id: n["id"].as_str().unwrap_or("").into(), content: hs_prop(n, "hs_note_body").unwrap_or_default(), contact_id: contact_id.map(Into::into), company_id: None, deal_id: deal_id.map(Into::into), created_at: hs_prop(n, "hs_createdate"), backend: "hubspot".into() }).collect()).unwrap_or_default())
    }
    async fn create_note(&self, content: &str, contact_id: Option<&str>, _company_id: Option<&str>, deal_id: Option<&str>) -> Result<Note> {
        let resp = self.post("crm/v3/objects/notes", &serde_json::json!({"properties": {"hs_note_body": content}})).await?;
        let id = resp["id"].as_str().unwrap_or("").to_string();
        if let Some(c) = contact_id { let _ = self.post(&format!("crm/v3/objects/notes/{id}/associations/contacts/{c}/note_to_contact"), &serde_json::json!({})).await; }
        if let Some(d) = deal_id { let _ = self.post(&format!("crm/v3/objects/notes/{id}/associations/deals/{d}/note_to_deal"), &serde_json::json!({})).await; }
        Ok(Note { id, content: content.into(), contact_id: contact_id.map(Into::into), company_id: None, deal_id: deal_id.map(Into::into), created_at: None, backend: "hubspot".into() })
    }
}
