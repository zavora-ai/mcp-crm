//! Pipedrive REST API backend.
use crate::types::*;
use anyhow::Result;
use reqwest::Client;

const BASE: &str = "https://api.pipedrive.com/v1";

#[derive(Clone)]
pub struct PipedriveBackend { http: Client, token: String }

impl PipedriveBackend {
    pub fn new(token: String) -> Self { Self { http: Client::new(), token } }
    async fn get(&self, path: &str) -> Result<serde_json::Value> {
        Ok(self.http.get(format!("{BASE}/{path}&api_token={}", self.token)).send().await?.error_for_status()?.json().await?)
    }
    async fn get_simple(&self, path: &str) -> Result<serde_json::Value> {
        Ok(self.http.get(format!("{BASE}/{path}?api_token={}", self.token)).send().await?.error_for_status()?.json().await?)
    }
    async fn post(&self, path: &str, body: &serde_json::Value) -> Result<serde_json::Value> {
        Ok(self.http.post(format!("{BASE}/{path}?api_token={}", self.token)).json(body).send().await?.error_for_status()?.json().await?)
    }
    async fn put(&self, path: &str, body: &serde_json::Value) -> Result<serde_json::Value> {
        Ok(self.http.put(format!("{BASE}/{path}?api_token={}", self.token)).json(body).send().await?.error_for_status()?.json().await?)
    }
}

fn pd(v: &serde_json::Value, k: &str) -> Option<String> { v[k].as_str().filter(|s| !s.is_empty()).map(Into::into).or_else(|| v[k].as_i64().map(|i| i.to_string())) }

#[async_trait::async_trait]
impl CrmBackend for PipedriveBackend {
    fn name(&self) -> &str { "pipedrive" }

    async fn list_contacts(&self, limit: u32) -> Result<Vec<Contact>> {
        let resp = self.get(&format!("persons?limit={limit}")).await?;
        Ok(resp["data"].as_array().map(|a| a.iter().map(|c| Contact { id: c["id"].as_i64().map(|i| i.to_string()).unwrap_or_default(), first_name: c["first_name"].as_str().map(Into::into), last_name: c["last_name"].as_str().map(Into::into), email: c["email"].as_array().and_then(|e| e.first()).and_then(|e| e["value"].as_str()).map(Into::into), phone: c["phone"].as_array().and_then(|p| p.first()).and_then(|p| p["value"].as_str()).map(Into::into), company_id: c["org_id"]["value"].as_i64().map(|i| i.to_string()), company_name: c["org_id"]["name"].as_str().map(Into::into), title: None, backend: "pipedrive".into() }).collect()).unwrap_or_default())
    }
    async fn get_contact(&self, id: &str) -> Result<Contact> {
        let resp = self.get_simple(&format!("persons/{id}")).await?;
        let c = &resp["data"];
        Ok(Contact { id: id.into(), first_name: c["first_name"].as_str().map(Into::into), last_name: c["last_name"].as_str().map(Into::into), email: c["email"].as_array().and_then(|e| e.first()).and_then(|e| e["value"].as_str()).map(Into::into), phone: c["phone"].as_array().and_then(|p| p.first()).and_then(|p| p["value"].as_str()).map(Into::into), company_id: c["org_id"]["value"].as_i64().map(|i| i.to_string()), company_name: c["org_id"]["name"].as_str().map(Into::into), title: None, backend: "pipedrive".into() })
    }
    async fn create_contact(&self, first_name: &str, last_name: &str, email: Option<&str>, phone: Option<&str>, company_id: Option<&str>) -> Result<Contact> {
        let mut body = serde_json::json!({"first_name": first_name, "last_name": last_name, "name": format!("{first_name} {last_name}")});
        if let Some(e) = email { body["email"] = serde_json::json!([{"value": e, "primary": true}]); }
        if let Some(p) = phone { body["phone"] = serde_json::json!([{"value": p, "primary": true}]); }
        if let Some(o) = company_id { body["org_id"] = o.parse::<i64>().unwrap_or(0).into(); }
        let resp = self.post("persons", &body).await?;
        let id = resp["data"]["id"].as_i64().map(|i| i.to_string()).unwrap_or_default();
        self.get_contact(&id).await
    }
    async fn update_contact(&self, id: &str, first_name: Option<&str>, last_name: Option<&str>, email: Option<&str>, phone: Option<&str>) -> Result<Contact> {
        let mut body = serde_json::json!({});
        if let Some(f) = first_name { body["first_name"] = f.into(); }
        if let Some(l) = last_name { body["last_name"] = l.into(); }
        if let Some(e) = email { body["email"] = serde_json::json!([{"value": e, "primary": true}]); }
        if let Some(p) = phone { body["phone"] = serde_json::json!([{"value": p, "primary": true}]); }
        self.put(&format!("persons/{id}"), &body).await?;
        self.get_contact(id).await
    }
    async fn search_contacts(&self, query: &str, limit: u32) -> Result<Vec<Contact>> {
        let resp = self.get(&format!("persons/search?term={query}&limit={limit}")).await?;
        Ok(resp["data"]["items"].as_array().map(|a| a.iter().map(|item| { let c = &item["item"]; Contact { id: c["id"].as_i64().map(|i| i.to_string()).unwrap_or_default(), first_name: None, last_name: None, email: c["primary_email"].as_str().map(Into::into), phone: c["primary_phone"].as_str().map(Into::into), company_id: None, company_name: c["organization"]["name"].as_str().map(Into::into), title: None, backend: "pipedrive".into() }}).collect()).unwrap_or_default())
    }
    async fn list_companies(&self, limit: u32) -> Result<Vec<Company>> {
        let resp = self.get(&format!("organizations?limit={limit}")).await?;
        Ok(resp["data"].as_array().map(|a| a.iter().map(|c| Company { id: c["id"].as_i64().map(|i| i.to_string()).unwrap_or_default(), name: c["name"].as_str().unwrap_or("").into(), domain: None, industry: None, phone: None, city: pd(c, "address_locality"), country: pd(c, "address_country"), backend: "pipedrive".into() }).collect()).unwrap_or_default())
    }
    async fn get_company(&self, id: &str) -> Result<Company> {
        let resp = self.get_simple(&format!("organizations/{id}")).await?;
        let c = &resp["data"];
        Ok(Company { id: id.into(), name: c["name"].as_str().unwrap_or("").into(), domain: None, industry: None, phone: None, city: pd(c, "address_locality"), country: pd(c, "address_country"), backend: "pipedrive".into() })
    }
    async fn create_company(&self, name: &str, _domain: Option<&str>, _industry: Option<&str>) -> Result<Company> {
        let resp = self.post("organizations", &serde_json::json!({"name": name})).await?;
        let id = resp["data"]["id"].as_i64().map(|i| i.to_string()).unwrap_or_default();
        self.get_company(&id).await
    }
    async fn update_company(&self, id: &str, name: Option<&str>, _domain: Option<&str>, _industry: Option<&str>) -> Result<Company> {
        let mut body = serde_json::json!({});
        if let Some(n) = name { body["name"] = n.into(); }
        self.put(&format!("organizations/{id}"), &body).await?;
        self.get_company(id).await
    }
    async fn list_deals(&self, limit: u32) -> Result<Vec<Deal>> {
        let resp = self.get(&format!("deals?limit={limit}")).await?;
        Ok(resp["data"].as_array().map(|a| a.iter().map(|d| Deal { id: d["id"].as_i64().map(|i| i.to_string()).unwrap_or_default(), name: d["title"].as_str().unwrap_or("").into(), stage: d["stage_id"].as_i64().map(|i| i.to_string()), pipeline: d["pipeline_id"].as_i64().map(|i| i.to_string()), amount: d["value"].as_f64(), currency: pd(d, "currency"), contact_id: d["person_id"]["value"].as_i64().map(|i| i.to_string()), company_id: d["org_id"]["value"].as_i64().map(|i| i.to_string()), close_date: pd(d, "expected_close_date"), probability: d["probability"].as_f64(), status: pd(d, "status"), backend: "pipedrive".into() }).collect()).unwrap_or_default())
    }
    async fn get_deal(&self, id: &str) -> Result<Deal> {
        let resp = self.get_simple(&format!("deals/{id}")).await?;
        let d = &resp["data"];
        Ok(Deal { id: id.into(), name: d["title"].as_str().unwrap_or("").into(), stage: d["stage_id"].as_i64().map(|i| i.to_string()), pipeline: d["pipeline_id"].as_i64().map(|i| i.to_string()), amount: d["value"].as_f64(), currency: pd(d, "currency"), contact_id: d["person_id"]["value"].as_i64().map(|i| i.to_string()), company_id: d["org_id"]["value"].as_i64().map(|i| i.to_string()), close_date: pd(d, "expected_close_date"), probability: d["probability"].as_f64(), status: pd(d, "status"), backend: "pipedrive".into() })
    }
    async fn create_deal(&self, name: &str, amount: Option<f64>, stage: Option<&str>, contact_id: Option<&str>, company_id: Option<&str>) -> Result<Deal> {
        let mut body = serde_json::json!({"title": name});
        if let Some(a) = amount { body["value"] = a.into(); }
        if let Some(s) = stage { body["stage_id"] = s.parse::<i64>().unwrap_or(0).into(); }
        if let Some(c) = contact_id { body["person_id"] = c.parse::<i64>().unwrap_or(0).into(); }
        if let Some(o) = company_id { body["org_id"] = o.parse::<i64>().unwrap_or(0).into(); }
        let resp = self.post("deals", &body).await?;
        let id = resp["data"]["id"].as_i64().map(|i| i.to_string()).unwrap_or_default();
        self.get_deal(&id).await
    }
    async fn update_deal(&self, id: &str, name: Option<&str>, amount: Option<f64>, stage: Option<&str>, close_date: Option<&str>) -> Result<Deal> {
        let mut body = serde_json::json!({});
        if let Some(n) = name { body["title"] = n.into(); }
        if let Some(a) = amount { body["value"] = a.into(); }
        if let Some(s) = stage { body["stage_id"] = s.parse::<i64>().unwrap_or(0).into(); }
        if let Some(d) = close_date { body["expected_close_date"] = d.into(); }
        self.put(&format!("deals/{id}"), &body).await?;
        self.get_deal(id).await
    }
    async fn move_deal_stage(&self, id: &str, stage: &str) -> Result<Deal> {
        self.put(&format!("deals/{id}"), &serde_json::json!({"stage_id": stage.parse::<i64>().unwrap_or(0)})).await?;
        self.get_deal(id).await
    }
    async fn list_activities(&self, _contact_id: Option<&str>, deal_id: Option<&str>, limit: u32) -> Result<Vec<Activity>> {
        let path = match deal_id {
            Some(d) => format!("deals/{d}/activities?limit={limit}"),
            None => format!("activities?limit={limit}"),
        };
        let resp = self.get(&path).await.unwrap_or(serde_json::json!({"data": []}));
        Ok(resp["data"].as_array().map(|a| a.iter().map(|t| Activity { id: t["id"].as_i64().map(|i| i.to_string()).unwrap_or_default(), activity_type: t["type"].as_str().unwrap_or("task").into(), subject: pd(t, "subject"), body: pd(t, "note"), contact_id: t["person_id"].as_i64().map(|i| i.to_string()), deal_id: t["deal_id"].as_i64().map(|i| i.to_string()), done: t["done"].as_bool().unwrap_or(false), due_date: pd(t, "due_date"), backend: "pipedrive".into() }).collect()).unwrap_or_default())
    }
    async fn create_activity(&self, activity_type: &str, subject: &str, body: Option<&str>, contact_id: Option<&str>, deal_id: Option<&str>) -> Result<Activity> {
        let mut b = serde_json::json!({"type": activity_type, "subject": subject});
        if let Some(n) = body { b["note"] = n.into(); }
        if let Some(c) = contact_id { b["person_id"] = c.parse::<i64>().unwrap_or(0).into(); }
        if let Some(d) = deal_id { b["deal_id"] = d.parse::<i64>().unwrap_or(0).into(); }
        let resp = self.post("activities", &b).await?;
        Ok(Activity { id: resp["data"]["id"].as_i64().map(|i| i.to_string()).unwrap_or_default(), activity_type: activity_type.into(), subject: Some(subject.into()), body: body.map(Into::into), contact_id: contact_id.map(Into::into), deal_id: deal_id.map(Into::into), done: false, due_date: None, backend: "pipedrive".into() })
    }
    async fn list_pipelines(&self) -> Result<Vec<Pipeline>> {
        let resp = self.get_simple("pipelines").await?;
        let mut pipelines = Vec::new();
        for p in resp["data"].as_array().unwrap_or(&vec![]) {
            let pid = p["id"].as_i64().map(|i| i.to_string()).unwrap_or_default();
            let stages_resp = self.get_simple(&format!("stages?pipeline_id={pid}")).await.unwrap_or(serde_json::json!({"data": []}));
            let stages = stages_resp["data"].as_array().map(|a| a.iter().map(|s| PipelineStage { id: s["id"].as_i64().map(|i| i.to_string()).unwrap_or_default(), name: s["name"].as_str().unwrap_or("").into(), order: s["order_nr"].as_u64().unwrap_or(0) as u32, deal_count: s["deals_summary"]["total_count"].as_u64().map(|v| v as u32), total_value: s["deals_summary"]["total_value"].as_f64() }).collect()).unwrap_or_default();
            pipelines.push(Pipeline { id: pid, name: p["name"].as_str().unwrap_or("").into(), stages, backend: "pipedrive".into() });
        }
        Ok(pipelines)
    }
    async fn get_pipeline_summary(&self, _pipeline_id: Option<&str>) -> Result<Vec<Pipeline>> { self.list_pipelines().await }
    async fn list_notes(&self, contact_id: Option<&str>, deal_id: Option<&str>, limit: u32) -> Result<Vec<Note>> {
        let filter = match (contact_id, deal_id) {
            (Some(c), _) => format!("person_id={c}&"),
            (_, Some(d)) => format!("deal_id={d}&"),
            _ => String::new(),
        };
        let resp = self.get(&format!("notes?{filter}limit={limit}")).await.unwrap_or(serde_json::json!({"data": []}));
        Ok(resp["data"].as_array().map(|a| a.iter().map(|n| Note { id: n["id"].as_i64().map(|i| i.to_string()).unwrap_or_default(), content: n["content"].as_str().unwrap_or("").into(), contact_id: n["person_id"].as_i64().map(|i| i.to_string()), company_id: n["org_id"].as_i64().map(|i| i.to_string()), deal_id: n["deal_id"].as_i64().map(|i| i.to_string()), created_at: pd(n, "add_time"), backend: "pipedrive".into() }).collect()).unwrap_or_default())
    }
    async fn create_note(&self, content: &str, contact_id: Option<&str>, company_id: Option<&str>, deal_id: Option<&str>) -> Result<Note> {
        let mut body = serde_json::json!({"content": content});
        if let Some(c) = contact_id { body["person_id"] = c.parse::<i64>().unwrap_or(0).into(); }
        if let Some(o) = company_id { body["org_id"] = o.parse::<i64>().unwrap_or(0).into(); }
        if let Some(d) = deal_id { body["deal_id"] = d.parse::<i64>().unwrap_or(0).into(); }
        let resp = self.post("notes", &body).await?;
        Ok(Note { id: resp["data"]["id"].as_i64().map(|i| i.to_string()).unwrap_or_default(), content: content.into(), contact_id: contact_id.map(Into::into), company_id: company_id.map(Into::into), deal_id: deal_id.map(Into::into), created_at: None, backend: "pipedrive".into() })
    }
}
