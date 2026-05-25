//! Zoho CRM REST API v2 backend.
use crate::types::*;
use anyhow::Result;
use reqwest::Client;

const BASE: &str = "https://www.zohoapis.com/crm/v2";

#[derive(Clone)]
pub struct ZohoCrmBackend { http: Client, token: String }

impl ZohoCrmBackend {
    pub fn new(token: String) -> Self { Self { http: Client::new(), token } }
    async fn get(&self, path: &str) -> Result<serde_json::Value> {
        Ok(self.http.get(format!("{BASE}/{path}")).header("Authorization", format!("Zoho-oauthtoken {}", self.token)).send().await?.error_for_status()?.json().await?)
    }
    async fn post(&self, path: &str, body: &serde_json::Value) -> Result<serde_json::Value> {
        Ok(self.http.post(format!("{BASE}/{path}")).header("Authorization", format!("Zoho-oauthtoken {}", self.token)).json(body).send().await?.error_for_status()?.json().await?)
    }
    async fn put(&self, path: &str, body: &serde_json::Value) -> Result<serde_json::Value> {
        Ok(self.http.put(format!("{BASE}/{path}")).header("Authorization", format!("Zoho-oauthtoken {}", self.token)).json(body).send().await?.error_for_status()?.json().await?)
    }
}

fn z(v: &serde_json::Value, k: &str) -> Option<String> { v[k].as_str().filter(|s| !s.is_empty()).map(Into::into) }

#[async_trait::async_trait]
impl CrmBackend for ZohoCrmBackend {
    fn name(&self) -> &str { "zoho_crm" }

    async fn list_contacts(&self, limit: u32) -> Result<Vec<Contact>> {
        let resp = self.get(&format!("Contacts?per_page={limit}")).await?;
        Ok(resp["data"].as_array().map(|a| a.iter().map(|c| Contact { id: c["id"].as_str().unwrap_or("").into(), first_name: z(c, "First_Name"), last_name: z(c, "Last_Name"), email: z(c, "Email"), phone: z(c, "Phone"), company_id: c["Account_Name"]["id"].as_str().map(Into::into), company_name: c["Account_Name"]["name"].as_str().map(Into::into), title: z(c, "Title"), backend: "zoho_crm".into() }).collect()).unwrap_or_default())
    }
    async fn get_contact(&self, id: &str) -> Result<Contact> {
        let resp = self.get(&format!("Contacts/{id}")).await?;
        let c = &resp["data"][0];
        Ok(Contact { id: c["id"].as_str().unwrap_or("").into(), first_name: z(c, "First_Name"), last_name: z(c, "Last_Name"), email: z(c, "Email"), phone: z(c, "Phone"), company_id: c["Account_Name"]["id"].as_str().map(Into::into), company_name: c["Account_Name"]["name"].as_str().map(Into::into), title: z(c, "Title"), backend: "zoho_crm".into() })
    }
    async fn create_contact(&self, first_name: &str, last_name: &str, email: Option<&str>, phone: Option<&str>, _company_id: Option<&str>) -> Result<Contact> {
        let mut rec = serde_json::json!({"First_Name": first_name, "Last_Name": last_name});
        if let Some(e) = email { rec["Email"] = e.into(); }
        if let Some(p) = phone { rec["Phone"] = p.into(); }
        let resp = self.post("Contacts", &serde_json::json!({"data": [rec]})).await?;
        let id = resp["data"][0]["details"]["id"].as_str().unwrap_or("");
        self.get_contact(id).await
    }
    async fn update_contact(&self, id: &str, first_name: Option<&str>, last_name: Option<&str>, email: Option<&str>, phone: Option<&str>) -> Result<Contact> {
        let mut rec = serde_json::json!({"id": id});
        if let Some(f) = first_name { rec["First_Name"] = f.into(); }
        if let Some(l) = last_name { rec["Last_Name"] = l.into(); }
        if let Some(e) = email { rec["Email"] = e.into(); }
        if let Some(p) = phone { rec["Phone"] = p.into(); }
        self.put("Contacts", &serde_json::json!({"data": [rec]})).await?;
        self.get_contact(id).await
    }
    async fn search_contacts(&self, query: &str, limit: u32) -> Result<Vec<Contact>> {
        let resp = self.get(&format!("Contacts/search?criteria=(Last_Name:contains:{query})&per_page={limit}")).await.unwrap_or(serde_json::json!({"data": []}));
        Ok(resp["data"].as_array().map(|a| a.iter().map(|c| Contact { id: c["id"].as_str().unwrap_or("").into(), first_name: z(c, "First_Name"), last_name: z(c, "Last_Name"), email: z(c, "Email"), phone: z(c, "Phone"), company_id: None, company_name: None, title: z(c, "Title"), backend: "zoho_crm".into() }).collect()).unwrap_or_default())
    }
    async fn list_companies(&self, limit: u32) -> Result<Vec<Company>> {
        let resp = self.get(&format!("Accounts?per_page={limit}")).await?;
        Ok(resp["data"].as_array().map(|a| a.iter().map(|c| Company { id: c["id"].as_str().unwrap_or("").into(), name: c["Account_Name"].as_str().unwrap_or("").into(), domain: z(c, "Website"), industry: z(c, "Industry"), phone: z(c, "Phone"), city: z(c, "Billing_City"), country: z(c, "Billing_Country"), backend: "zoho_crm".into() }).collect()).unwrap_or_default())
    }
    async fn get_company(&self, id: &str) -> Result<Company> {
        let resp = self.get(&format!("Accounts/{id}")).await?;
        let c = &resp["data"][0];
        Ok(Company { id: c["id"].as_str().unwrap_or("").into(), name: c["Account_Name"].as_str().unwrap_or("").into(), domain: z(c, "Website"), industry: z(c, "Industry"), phone: z(c, "Phone"), city: z(c, "Billing_City"), country: z(c, "Billing_Country"), backend: "zoho_crm".into() })
    }
    async fn create_company(&self, name: &str, domain: Option<&str>, industry: Option<&str>) -> Result<Company> {
        let mut rec = serde_json::json!({"Account_Name": name});
        if let Some(d) = domain { rec["Website"] = d.into(); }
        if let Some(i) = industry { rec["Industry"] = i.into(); }
        let resp = self.post("Accounts", &serde_json::json!({"data": [rec]})).await?;
        let id = resp["data"][0]["details"]["id"].as_str().unwrap_or("");
        self.get_company(id).await
    }
    async fn update_company(&self, id: &str, name: Option<&str>, domain: Option<&str>, industry: Option<&str>) -> Result<Company> {
        let mut rec = serde_json::json!({"id": id});
        if let Some(n) = name { rec["Account_Name"] = n.into(); }
        if let Some(d) = domain { rec["Website"] = d.into(); }
        if let Some(i) = industry { rec["Industry"] = i.into(); }
        self.put("Accounts", &serde_json::json!({"data": [rec]})).await?;
        self.get_company(id).await
    }
    async fn list_deals(&self, limit: u32) -> Result<Vec<Deal>> {
        let resp = self.get(&format!("Deals?per_page={limit}")).await?;
        Ok(resp["data"].as_array().map(|a| a.iter().map(|d| Deal { id: d["id"].as_str().unwrap_or("").into(), name: d["Deal_Name"].as_str().unwrap_or("").into(), stage: z(d, "Stage"), pipeline: z(d, "Pipeline"), amount: d["Amount"].as_f64(), currency: None, contact_id: d["Contact_Name"]["id"].as_str().map(Into::into), company_id: d["Account_Name"]["id"].as_str().map(Into::into), close_date: z(d, "Closing_Date"), probability: d["Probability"].as_f64(), status: None, backend: "zoho_crm".into() }).collect()).unwrap_or_default())
    }
    async fn get_deal(&self, id: &str) -> Result<Deal> {
        let resp = self.get(&format!("Deals/{id}")).await?;
        let d = &resp["data"][0];
        Ok(Deal { id: d["id"].as_str().unwrap_or("").into(), name: d["Deal_Name"].as_str().unwrap_or("").into(), stage: z(d, "Stage"), pipeline: z(d, "Pipeline"), amount: d["Amount"].as_f64(), currency: None, contact_id: d["Contact_Name"]["id"].as_str().map(Into::into), company_id: d["Account_Name"]["id"].as_str().map(Into::into), close_date: z(d, "Closing_Date"), probability: d["Probability"].as_f64(), status: None, backend: "zoho_crm".into() })
    }
    async fn create_deal(&self, name: &str, amount: Option<f64>, stage: Option<&str>, contact_id: Option<&str>, company_id: Option<&str>) -> Result<Deal> {
        let mut rec = serde_json::json!({"Deal_Name": name, "Stage": stage.unwrap_or("Qualification")});
        if let Some(a) = amount { rec["Amount"] = a.into(); }
        if let Some(c) = contact_id { rec["Contact_Name"] = serde_json::json!({"id": c}); }
        if let Some(c) = company_id { rec["Account_Name"] = serde_json::json!({"id": c}); }
        let resp = self.post("Deals", &serde_json::json!({"data": [rec]})).await?;
        let id = resp["data"][0]["details"]["id"].as_str().unwrap_or("");
        self.get_deal(id).await
    }
    async fn update_deal(&self, id: &str, name: Option<&str>, amount: Option<f64>, stage: Option<&str>, close_date: Option<&str>) -> Result<Deal> {
        let mut rec = serde_json::json!({"id": id});
        if let Some(n) = name { rec["Deal_Name"] = n.into(); }
        if let Some(a) = amount { rec["Amount"] = a.into(); }
        if let Some(s) = stage { rec["Stage"] = s.into(); }
        if let Some(d) = close_date { rec["Closing_Date"] = d.into(); }
        self.put("Deals", &serde_json::json!({"data": [rec]})).await?;
        self.get_deal(id).await
    }
    async fn move_deal_stage(&self, id: &str, stage: &str) -> Result<Deal> {
        self.put("Deals", &serde_json::json!({"data": [{"id": id, "Stage": stage}]})).await?;
        self.get_deal(id).await
    }
    async fn list_activities(&self, contact_id: Option<&str>, _deal_id: Option<&str>, limit: u32) -> Result<Vec<Activity>> {
        let path = match contact_id {
            Some(c) => format!("Contacts/{c}/Activities?per_page={limit}"),
            None => format!("Activities?per_page={limit}"),
        };
        let resp = self.get(&path).await.unwrap_or(serde_json::json!({"data": []}));
        Ok(resp["data"].as_array().map(|a| a.iter().map(|t| Activity { id: t["id"].as_str().unwrap_or("").into(), activity_type: t["Activity_Type"].as_str().unwrap_or("task").into(), subject: z(t, "Subject"), body: z(t, "Description"), contact_id: contact_id.map(Into::into), deal_id: None, done: t["Status"].as_str() == Some("Completed"), due_date: z(t, "Due_Date"), backend: "zoho_crm".into() }).collect()).unwrap_or_default())
    }
    async fn create_activity(&self, activity_type: &str, subject: &str, body: Option<&str>, contact_id: Option<&str>, deal_id: Option<&str>) -> Result<Activity> {
        let mut rec = serde_json::json!({"Subject": subject, "Activity_Type": activity_type});
        if let Some(b) = body { rec["Description"] = b.into(); }
        if let Some(c) = contact_id { rec["Who_Id"] = serde_json::json!({"id": c}); }
        if let Some(d) = deal_id { rec["What_Id"] = serde_json::json!({"id": d}); }
        let resp = self.post("Tasks", &serde_json::json!({"data": [rec]})).await?;
        Ok(Activity { id: resp["data"][0]["details"]["id"].as_str().unwrap_or("").into(), activity_type: activity_type.into(), subject: Some(subject.into()), body: body.map(Into::into), contact_id: contact_id.map(Into::into), deal_id: deal_id.map(Into::into), done: false, due_date: None, backend: "zoho_crm".into() })
    }
    async fn list_pipelines(&self) -> Result<Vec<Pipeline>> {
        let resp = self.get("settings/pipeline?module=Deals").await?;
        Ok(resp["pipeline"].as_array().map(|a| a.iter().map(|p| Pipeline { id: p["id"].as_str().unwrap_or("").into(), name: p["display_value"].as_str().unwrap_or("").into(), stages: p["maps"].as_array().map(|s| s.iter().enumerate().map(|(i, st)| PipelineStage { id: st["id"].as_str().unwrap_or("").into(), name: st["display_value"].as_str().unwrap_or("").into(), order: i as u32, deal_count: None, total_value: None }).collect()).unwrap_or_default(), backend: "zoho_crm".into() }).collect()).unwrap_or_default())
    }
    async fn get_pipeline_summary(&self, _pipeline_id: Option<&str>) -> Result<Vec<Pipeline>> { self.list_pipelines().await }
    async fn list_notes(&self, contact_id: Option<&str>, deal_id: Option<&str>, limit: u32) -> Result<Vec<Note>> {
        let path = match (contact_id, deal_id) {
            (Some(c), _) => format!("Contacts/{c}/Notes?per_page={limit}"),
            (_, Some(d)) => format!("Deals/{d}/Notes?per_page={limit}"),
            _ => format!("Notes?per_page={limit}"),
        };
        let resp = self.get(&path).await.unwrap_or(serde_json::json!({"data": []}));
        Ok(resp["data"].as_array().map(|a| a.iter().map(|n| Note { id: n["id"].as_str().unwrap_or("").into(), content: n["Note_Content"].as_str().unwrap_or("").into(), contact_id: contact_id.map(Into::into), company_id: None, deal_id: deal_id.map(Into::into), created_at: z(n, "Created_Time"), backend: "zoho_crm".into() }).collect()).unwrap_or_default())
    }
    async fn create_note(&self, content: &str, contact_id: Option<&str>, _company_id: Option<&str>, deal_id: Option<&str>) -> Result<Note> {
        let path = match (contact_id, deal_id) {
            (Some(c), _) => format!("Contacts/{c}/Notes"),
            (_, Some(d)) => format!("Deals/{d}/Notes"),
            _ => "Notes".into(),
        };
        let resp = self.post(&path, &serde_json::json!({"data": [{"Note_Content": content}]})).await?;
        Ok(Note { id: resp["data"][0]["details"]["id"].as_str().unwrap_or("").into(), content: content.into(), contact_id: contact_id.map(Into::into), company_id: None, deal_id: deal_id.map(Into::into), created_at: None, backend: "zoho_crm".into() })
    }
}
