//! Salesforce REST API backend.
use crate::types::*;
use anyhow::Result;
use reqwest::Client;

#[derive(Clone)]
pub struct SalesforceBackend { http: Client, instance_url: String, token: String }

impl SalesforceBackend {
    pub fn new(instance_url: String, token: String) -> Self {
        Self { http: Client::new(), instance_url: instance_url.trim_end_matches('/').to_string(), token }
    }
    async fn get(&self, path: &str) -> Result<serde_json::Value> {
        Ok(self.http.get(format!("{}/services/data/v59.0/{path}", self.instance_url)).bearer_auth(&self.token).send().await?.error_for_status()?.json().await?)
    }
    async fn post(&self, path: &str, body: &serde_json::Value) -> Result<serde_json::Value> {
        Ok(self.http.post(format!("{}/services/data/v59.0/{path}", self.instance_url)).bearer_auth(&self.token).json(body).send().await?.error_for_status()?.json().await?)
    }
    async fn patch(&self, path: &str, body: &serde_json::Value) -> Result<()> {
        self.http.patch(format!("{}/services/data/v59.0/{path}", self.instance_url)).bearer_auth(&self.token).json(body).send().await?.error_for_status()?; Ok(())
    }
    async fn query(&self, soql: &str) -> Result<serde_json::Value> {
        self.get(&format!("query?q={}", urlencoding::encode(soql))).await
    }
}

mod urlencoding {
    pub fn encode(s: &str) -> String {
        s.bytes().map(|b| match b { b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => (b as char).to_string(), _ => format!("%{:02X}", b) }).collect()
    }
}

fn sf_str(v: &serde_json::Value, k: &str) -> Option<String> { v[k].as_str().filter(|s| !s.is_empty()).map(Into::into) }

#[async_trait::async_trait]
impl CrmBackend for SalesforceBackend {
    fn name(&self) -> &str { "salesforce" }

    async fn list_contacts(&self, limit: u32) -> Result<Vec<Contact>> {
        let resp = self.query(&format!("SELECT Id,FirstName,LastName,Email,Phone,AccountId,Account.Name,Title FROM Contact LIMIT {limit}")).await?;
        Ok(resp["records"].as_array().map(|a| a.iter().map(|c| Contact { id: c["Id"].as_str().unwrap_or("").into(), first_name: sf_str(c, "FirstName"), last_name: sf_str(c, "LastName"), email: sf_str(c, "Email"), phone: sf_str(c, "Phone"), company_id: sf_str(c, "AccountId"), company_name: c["Account"]["Name"].as_str().map(Into::into), title: sf_str(c, "Title"), backend: "salesforce".into() }).collect()).unwrap_or_default())
    }
    async fn get_contact(&self, id: &str) -> Result<Contact> {
        let c = self.get(&format!("sobjects/Contact/{id}")).await?;
        Ok(Contact { id: c["Id"].as_str().unwrap_or("").into(), first_name: sf_str(&c, "FirstName"), last_name: sf_str(&c, "LastName"), email: sf_str(&c, "Email"), phone: sf_str(&c, "Phone"), company_id: sf_str(&c, "AccountId"), company_name: None, title: sf_str(&c, "Title"), backend: "salesforce".into() })
    }
    async fn create_contact(&self, first_name: &str, last_name: &str, email: Option<&str>, phone: Option<&str>, company_id: Option<&str>) -> Result<Contact> {
        let mut body = serde_json::json!({"FirstName": first_name, "LastName": last_name});
        if let Some(e) = email { body["Email"] = e.into(); }
        if let Some(p) = phone { body["Phone"] = p.into(); }
        if let Some(a) = company_id { body["AccountId"] = a.into(); }
        let resp = self.post("sobjects/Contact", &body).await?;
        let id = resp["id"].as_str().unwrap_or("");
        self.get_contact(id).await
    }
    async fn update_contact(&self, id: &str, first_name: Option<&str>, last_name: Option<&str>, email: Option<&str>, phone: Option<&str>) -> Result<Contact> {
        let mut body = serde_json::json!({});
        if let Some(f) = first_name { body["FirstName"] = f.into(); }
        if let Some(l) = last_name { body["LastName"] = l.into(); }
        if let Some(e) = email { body["Email"] = e.into(); }
        if let Some(p) = phone { body["Phone"] = p.into(); }
        self.patch(&format!("sobjects/Contact/{id}"), &body).await?;
        self.get_contact(id).await
    }
    async fn search_contacts(&self, query: &str, limit: u32) -> Result<Vec<Contact>> {
        let escaped = query.replace('\'', "\\'");
        self.list_contacts(limit).await.or_else(|_| {
            Ok(vec![]) // fallback if SOSL fails
        }).and_then(|contacts| {
            let filtered: Vec<_> = contacts.into_iter().filter(|c| {
                let name = format!("{} {}", c.first_name.as_deref().unwrap_or(""), c.last_name.as_deref().unwrap_or(""));
                name.to_lowercase().contains(&escaped.to_lowercase()) || c.email.as_deref().unwrap_or("").to_lowercase().contains(&escaped.to_lowercase())
            }).take(limit as usize).collect();
            Ok(filtered)
        })
    }
    async fn list_companies(&self, limit: u32) -> Result<Vec<Company>> {
        let resp = self.query(&format!("SELECT Id,Name,Website,Industry,Phone,BillingCity,BillingCountry FROM Account LIMIT {limit}")).await?;
        Ok(resp["records"].as_array().map(|a| a.iter().map(|c| Company { id: c["Id"].as_str().unwrap_or("").into(), name: c["Name"].as_str().unwrap_or("").into(), domain: sf_str(c, "Website"), industry: sf_str(c, "Industry"), phone: sf_str(c, "Phone"), city: sf_str(c, "BillingCity"), country: sf_str(c, "BillingCountry"), backend: "salesforce".into() }).collect()).unwrap_or_default())
    }
    async fn get_company(&self, id: &str) -> Result<Company> {
        let c = self.get(&format!("sobjects/Account/{id}")).await?;
        Ok(Company { id: c["Id"].as_str().unwrap_or("").into(), name: c["Name"].as_str().unwrap_or("").into(), domain: sf_str(&c, "Website"), industry: sf_str(&c, "Industry"), phone: sf_str(&c, "Phone"), city: sf_str(&c, "BillingCity"), country: sf_str(&c, "BillingCountry"), backend: "salesforce".into() })
    }
    async fn create_company(&self, name: &str, domain: Option<&str>, industry: Option<&str>) -> Result<Company> {
        let mut body = serde_json::json!({"Name": name});
        if let Some(d) = domain { body["Website"] = d.into(); }
        if let Some(i) = industry { body["Industry"] = i.into(); }
        let resp = self.post("sobjects/Account", &body).await?;
        self.get_company(resp["id"].as_str().unwrap_or("")).await
    }
    async fn update_company(&self, id: &str, name: Option<&str>, domain: Option<&str>, industry: Option<&str>) -> Result<Company> {
        let mut body = serde_json::json!({});
        if let Some(n) = name { body["Name"] = n.into(); }
        if let Some(d) = domain { body["Website"] = d.into(); }
        if let Some(i) = industry { body["Industry"] = i.into(); }
        self.patch(&format!("sobjects/Account/{id}"), &body).await?;
        self.get_company(id).await
    }
    async fn list_deals(&self, limit: u32) -> Result<Vec<Deal>> {
        let resp = self.query(&format!("SELECT Id,Name,StageName,Amount,CloseDate,Probability,AccountId,ContactId FROM Opportunity LIMIT {limit}")).await?;
        Ok(resp["records"].as_array().map(|a| a.iter().map(|d| Deal { id: d["Id"].as_str().unwrap_or("").into(), name: d["Name"].as_str().unwrap_or("").into(), stage: sf_str(d, "StageName"), pipeline: None, amount: d["Amount"].as_f64(), currency: None, contact_id: sf_str(d, "ContactId"), company_id: sf_str(d, "AccountId"), close_date: sf_str(d, "CloseDate"), probability: d["Probability"].as_f64(), status: None, backend: "salesforce".into() }).collect()).unwrap_or_default())
    }
    async fn get_deal(&self, id: &str) -> Result<Deal> {
        let d = self.get(&format!("sobjects/Opportunity/{id}")).await?;
        Ok(Deal { id: d["Id"].as_str().unwrap_or("").into(), name: d["Name"].as_str().unwrap_or("").into(), stage: sf_str(&d, "StageName"), pipeline: None, amount: d["Amount"].as_f64(), currency: None, contact_id: sf_str(&d, "ContactId"), company_id: sf_str(&d, "AccountId"), close_date: sf_str(&d, "CloseDate"), probability: d["Probability"].as_f64(), status: None, backend: "salesforce".into() })
    }
    async fn create_deal(&self, name: &str, amount: Option<f64>, stage: Option<&str>, contact_id: Option<&str>, company_id: Option<&str>) -> Result<Deal> {
        let mut body = serde_json::json!({"Name": name, "StageName": stage.unwrap_or("Prospecting"), "CloseDate": chrono::Utc::now().format("%Y-%m-%d").to_string()});
        if let Some(a) = amount { body["Amount"] = a.into(); }
        if let Some(c) = contact_id { body["ContactId"] = c.into(); }
        if let Some(c) = company_id { body["AccountId"] = c.into(); }
        let resp = self.post("sobjects/Opportunity", &body).await?;
        self.get_deal(resp["id"].as_str().unwrap_or("")).await
    }
    async fn update_deal(&self, id: &str, name: Option<&str>, amount: Option<f64>, stage: Option<&str>, close_date: Option<&str>) -> Result<Deal> {
        let mut body = serde_json::json!({});
        if let Some(n) = name { body["Name"] = n.into(); }
        if let Some(a) = amount { body["Amount"] = a.into(); }
        if let Some(s) = stage { body["StageName"] = s.into(); }
        if let Some(d) = close_date { body["CloseDate"] = d.into(); }
        self.patch(&format!("sobjects/Opportunity/{id}"), &body).await?;
        self.get_deal(id).await
    }
    async fn move_deal_stage(&self, id: &str, stage: &str) -> Result<Deal> {
        self.patch(&format!("sobjects/Opportunity/{id}"), &serde_json::json!({"StageName": stage})).await?;
        self.get_deal(id).await
    }
    async fn list_activities(&self, contact_id: Option<&str>, deal_id: Option<&str>, limit: u32) -> Result<Vec<Activity>> {
        let filter = match (contact_id, deal_id) {
            (Some(c), _) => format!("WHERE WhoId = '{c}'"),
            (_, Some(d)) => format!("WHERE WhatId = '{d}'"),
            _ => String::new(),
        };
        let resp = self.query(&format!("SELECT Id,Subject,Description,ActivityDate,Status,TaskSubtype FROM Task {filter} LIMIT {limit}")).await?;
        Ok(resp["records"].as_array().map(|a| a.iter().map(|t| Activity { id: t["Id"].as_str().unwrap_or("").into(), activity_type: t["TaskSubtype"].as_str().unwrap_or("task").into(), subject: sf_str(t, "Subject"), body: sf_str(t, "Description"), contact_id: contact_id.map(Into::into), deal_id: deal_id.map(Into::into), done: t["Status"].as_str() == Some("Completed"), due_date: sf_str(t, "ActivityDate"), backend: "salesforce".into() }).collect()).unwrap_or_default())
    }
    async fn create_activity(&self, activity_type: &str, subject: &str, body: Option<&str>, contact_id: Option<&str>, deal_id: Option<&str>) -> Result<Activity> {
        let mut b = serde_json::json!({"Subject": subject, "TaskSubtype": activity_type});
        if let Some(d) = body { b["Description"] = d.into(); }
        if let Some(c) = contact_id { b["WhoId"] = c.into(); }
        if let Some(d) = deal_id { b["WhatId"] = d.into(); }
        let resp = self.post("sobjects/Task", &b).await?;
        Ok(Activity { id: resp["id"].as_str().unwrap_or("").into(), activity_type: activity_type.into(), subject: Some(subject.into()), body: body.map(Into::into), contact_id: contact_id.map(Into::into), deal_id: deal_id.map(Into::into), done: false, due_date: None, backend: "salesforce".into() })
    }
    async fn list_pipelines(&self) -> Result<Vec<Pipeline>> {
        let resp = self.query("SELECT MasterLabel,ApiName FROM OpportunityStage ORDER BY SortOrder").await?;
        let stages: Vec<PipelineStage> = resp["records"].as_array().map(|a| a.iter().enumerate().map(|(i, s)| PipelineStage { id: s["ApiName"].as_str().unwrap_or("").into(), name: s["MasterLabel"].as_str().unwrap_or("").into(), order: i as u32, deal_count: None, total_value: None }).collect()).unwrap_or_default();
        Ok(vec![Pipeline { id: "default".into(), name: "Sales Pipeline".into(), stages, backend: "salesforce".into() }])
    }
    async fn get_pipeline_summary(&self, _pipeline_id: Option<&str>) -> Result<Vec<Pipeline>> { self.list_pipelines().await }
    async fn list_notes(&self, contact_id: Option<&str>, deal_id: Option<&str>, limit: u32) -> Result<Vec<Note>> {
        let filter = match (contact_id, deal_id) {
            (Some(c), _) => format!("WHERE ParentId = '{c}'"),
            (_, Some(d)) => format!("WHERE ParentId = '{d}'"),
            _ => String::new(),
        };
        let resp = self.query(&format!("SELECT Id,Body,ParentId,CreatedDate FROM Note {filter} LIMIT {limit}")).await?;
        Ok(resp["records"].as_array().map(|a| a.iter().map(|n| Note { id: n["Id"].as_str().unwrap_or("").into(), content: n["Body"].as_str().unwrap_or("").into(), contact_id: contact_id.map(Into::into), company_id: None, deal_id: deal_id.map(Into::into), created_at: sf_str(n, "CreatedDate"), backend: "salesforce".into() }).collect()).unwrap_or_default())
    }
    async fn create_note(&self, content: &str, contact_id: Option<&str>, _company_id: Option<&str>, deal_id: Option<&str>) -> Result<Note> {
        let parent = contact_id.or(deal_id).unwrap_or("");
        let resp = self.post("sobjects/Note", &serde_json::json!({"Body": content, "ParentId": parent, "Title": "Note"})).await?;
        Ok(Note { id: resp["id"].as_str().unwrap_or("").into(), content: content.into(), contact_id: contact_id.map(Into::into), company_id: None, deal_id: deal_id.map(Into::into), created_at: None, backend: "salesforce".into() })
    }
}
