# CRM MCP Server

[![Crates.io](https://img.shields.io/crates/v/mcp-crm.svg)](https://crates.io/crates/mcp-crm)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![ADK-Rust Enterprise](https://img.shields.io/badge/ADK--Rust-Enterprise-purple.svg)](https://enterprise.adk-rust.com)
[![Registry Ready](https://img.shields.io/badge/ADK_Registry-Ready-green.svg)](https://www.zavora.ai)

Unified CRM MCP server with **20 tools** across **4 backends** — Salesforce, HubSpot, Zoho CRM, and Pipedrive. Contacts, companies, deals, activities, pipelines, and notes with a single consistent schema.

## Architecture

<p align="center">
  <img src="https://raw.githubusercontent.com/zavora-ai/mcp-crm/main/docs/assets/architecture.svg" alt="MCP CRM Architecture" width="800"/>
</p>

## Key Principles

- **Unified schema** — agents see consistent types regardless of backend
- **Feature-flagged backends** — compile only what you need
- **Full CRM lifecycle** — contacts, companies, deals, activities, pipelines, notes
- **No credential exposure** — tokens stay in env vars
- **Single binary** — no Node.js, no Python

## Tools (20)

| Category | Tools | Risk |
|----------|-------|------|
| Contacts | `list_contacts`, `get_contact`, `create_contact`, `update_contact`, `search_contacts` | read / internal_write |
| Companies | `list_companies`, `get_company`, `create_company`, `update_company` | read / internal_write |
| Deals | `list_deals`, `get_deal`, `create_deal`, `update_deal`, `move_deal_stage` | read / internal_write |
| Activities | `list_activities`, `create_activity` | read / internal_write |
| Pipelines | `list_pipelines`, `get_pipeline_summary` | read_only |
| Notes | `list_notes`, `create_note` | read / internal_write |

## Backends

| Backend | Auth | Env Vars |
|---------|------|----------|
| **Salesforce** | OAuth2 | `SALESFORCE_INSTANCE_URL`, `SALESFORCE_TOKEN` |
| **HubSpot** | API key / OAuth | `HUBSPOT_TOKEN` |
| **Zoho CRM** | OAuth2 | `ZOHO_CRM_TOKEN` |
| **Pipedrive** | API token | `PIPEDRIVE_TOKEN` |

## Installation

```bash
cargo install mcp-crm --features all-backends
```

### Feature flags

```bash
# Default: HubSpot + Pipedrive
cargo install mcp-crm

# All backends
cargo install mcp-crm --features all-backends

# Specific
cargo install mcp-crm --no-default-features --features salesforce
```

## Client Configuration

### Claude Desktop / Kiro / Cursor

```json
{
  "mcpServers": {
    "crm": {
      "command": "mcp-crm",
      "args": [],
      "env": {
        "HUBSPOT_TOKEN": "pat-na1-xxxx"
      }
    }
  }
}
```

## Usage Examples

```
"List my top 10 contacts"
→ list_contacts(limit: 10)

"Search for contacts at Acme Corp"
→ search_contacts(query: "Acme")

"Create a deal for $50k in the Negotiation stage"
→ create_deal(name: "Acme Enterprise", amount: 50000, stage: "Negotiation")

"Move the deal to Closed Won"
→ move_deal_stage(id: "deal-123", stage: "closedwon")

"Log a call with Sarah about the proposal"
→ create_activity(activity_type: "call", subject: "Proposal discussion", contact_id: "sarah-id")

"Show me the pipeline summary"
→ get_pipeline_summary()
```

## Registry Compliance

- **HealthCheck** — verifies backend connectivity
- **mcp-server.toml** — 20 tools with risk classes and credential bindings
- **Manifest validation** — startup fails fast on invalid manifest
- **Structured tracing** — `RUST_LOG` env-filter

## Contributors

| [<img src="https://github.com/jkmaina.png" width="80px;" alt=""/><br /><sub><b>James Karanja Maina</b></sub>](https://github.com/jkmaina) |
|:---:|

## License

Apache-2.0

---

Part of the [ADK-Rust Enterprise](https://enterprise.adk-rust.com) MCP server ecosystem.
