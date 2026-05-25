//! mcp-crm — Enterprise CRM MCP Server
mod types;
mod server;

#[cfg(feature = "salesforce")]
mod salesforce;
#[cfg(feature = "hubspot")]
mod hubspot;
#[cfg(feature = "zoho-crm")]
mod zoho_crm;
#[cfg(feature = "pipedrive")]
mod pipedrive;

use rmcp::{ServiceExt, transport::stdio};
use server::CrmServer;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env().add_directive("info".parse()?))
        .init();

    let manifest = adk_mcp_sdk::ServerManifest::from_file(std::path::Path::new("mcp-server.toml"))?;
    let errors = manifest.validate();
    if !errors.is_empty() {
        for e in &errors { tracing::error!("manifest: {e}"); }
        anyhow::bail!("invalid mcp-server.toml ({} error(s))", errors.len());
    }

    let backend: Arc<dyn types::CrmBackend> = init_backend()?;
    tracing::info!("{} v{} starting on stdio (backend: {})", manifest.display_name, manifest.version, backend.name());
    let server = CrmServer { backend };
    let service = server.serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}

fn init_backend() -> anyhow::Result<Arc<dyn types::CrmBackend>> {
    #[cfg(feature = "salesforce")]
    if let (Ok(url), Ok(token)) = (std::env::var("SALESFORCE_INSTANCE_URL"), std::env::var("SALESFORCE_TOKEN")) {
        tracing::info!("Using Salesforce backend");
        return Ok(Arc::new(salesforce::SalesforceBackend::new(url, token)));
    }

    #[cfg(feature = "hubspot")]
    if let Ok(token) = std::env::var("HUBSPOT_TOKEN") {
        tracing::info!("Using HubSpot backend");
        return Ok(Arc::new(hubspot::HubSpotBackend::new(token)));
    }

    #[cfg(feature = "zoho-crm")]
    if let Ok(token) = std::env::var("ZOHO_CRM_TOKEN") {
        tracing::info!("Using Zoho CRM backend");
        return Ok(Arc::new(zoho_crm::ZohoCrmBackend::new(token)));
    }

    #[cfg(feature = "pipedrive")]
    if let Ok(token) = std::env::var("PIPEDRIVE_TOKEN") {
        tracing::info!("Using Pipedrive backend");
        return Ok(Arc::new(pipedrive::PipedriveBackend::new(token)));
    }

    anyhow::bail!("No CRM backend configured. Set one of: SALESFORCE_INSTANCE_URL+SALESFORCE_TOKEN, HUBSPOT_TOKEN, ZOHO_CRM_TOKEN, PIPEDRIVE_TOKEN")
}
