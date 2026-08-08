# CRM MCP Server

[![Crates.io](https://img.shields.io/crates/v/mcp-crm.svg)](https://crates.io/crates/mcp-crm)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![ADK-Rust Enterprise](https://img.shields.io/badge/ADK--Rust-Enterprise-purple.svg)](https://enterprise.adk-rust.com)
[![Registry Ready](https://img.shields.io/badge/ADK_Registry-Ready-green.svg)](https://www.zavora.ai)

Unified CRM MCP server with **28 tools** across **4 backends** — Salesforce, HubSpot, Zoho CRM, and Pipedrive. Unified contacts, companies, deals, activities, pipelines, and notes with associations, search, and lifecycle management. Single Rust binary with feature-flagged backends and enterprise governance.

## Architecture

<p align="center">
  <img src="https://raw.githubusercontent.com/zavora-ai/mcp-crm/main/docs/assets/architecture.svg" alt="MCP CRM Architecture" width="800"/>
</p>

## Key Principles

- **Unified schema** — agents see consistent types (Contact, Company, Deal, Activity, Pipeline, Note) regardless of backend
- **Feature-flagged backends** — compile only what you need (`--features salesforce,hubspot`)
- **Full CRM lifecycle** — search contacts, manage deals through pipeline stages, log activities, attach notes
- **No credential exposure** — tokens stay in env vars, never reach LLM context
- **Single binary** — no Node.js, no Python, no runtime dependencies

## Comparison

| Feature | Generic REST | Zapier | **mcp-crm** |
|---------|:---:|:---:|:---:|
| Multi-backend (4 CRMs) | ❌ | Partial | ✅ |
| Unified schema | ❌ | ❌ | ✅ |
| Pipeline stage management | ❌ | ❌ | ✅ |
| Contact search | ❌ | ❌ | ✅ |
| Activity logging | ❌ | Partial | ✅ |
| Risk classification | ❌ | ❌ | ✅ |
| Agent-native (MCP) | ❌ | ❌ | ✅ |
| Single binary | ❌ | ❌ | ✅ |

## Tools (28)

### Contacts (6)

| Tool | Purpose | Risk Class |
|------|---------|------------|
| `list_contacts` | List contacts with optional limit | Read-only |
| `get_contact` | Get a contact by ID | Read-only |
| `create_contact` | Create a new contact | Internal write |
| `update_contact` | Update an existing contact | Internal write |
| `search_contacts` | Search by name, email, or query | Read-only |
| `delete_contact` | Delete a contact by ID | External write |

### Companies (5)

| Tool | Purpose | Risk Class |
|------|---------|------------|
| `list_companies` | List companies/accounts | Read-only |
| `get_company` | Get a company by ID | Read-only |
| `create_company` | Create a new company | Internal write |
| `update_company` | Update an existing company | Internal write |
| `search_companies` | Search companies by name or domain | Read-only |

### Deals (7)

| Tool | Purpose | Risk Class |
|------|---------|------------|
| `list_deals` | List deals/opportunities | Read-only |
| `get_deal` | Get a deal with stage, value, probability | Read-only |
| `create_deal` | Create a new deal | Internal write |
| `update_deal` | Update deal (stage, value, close date) | Internal write |
| `move_deal_stage` | Move a deal to a different pipeline stage | Internal write |
| `search_deals` | Search deals by name, stage, or value | Read-only |
| `delete_deal` | Delete a deal by ID | External write |

### Activities (3)

| Tool | Purpose | Risk Class |
|------|---------|------------|
| `list_activities` | List activities for a contact or deal | Read-only |
| `create_activity` | Log a call, email, meeting, task, or note | Internal write |
| `update_activity` | Update an activity (mark done, change subject) | Internal write |

### Pipelines (2)

| Tool | Purpose | Risk Class |
|------|---------|------------|
| `list_pipelines` | List sales pipelines and their stages | Read-only |
| `get_pipeline_summary` | Get deal counts and values per stage | Read-only |

### Notes (2)

| Tool | Purpose | Risk Class |
|------|---------|------------|
| `list_notes` | List notes for a contact, company, or deal | Read-only |
| `create_note` | Add a note to a record | Internal write |

### Associations (3)

| Tool | Purpose | Risk Class |
|------|---------|------------|
| `associate_contact_company` | Link a contact to a company | Internal write |
| `associate_deal_contact` | Link a deal to a contact | Internal write |
| `list_deal_contacts` | List contacts associated with a deal | Read-only |

## Backends

| Backend | Protocol | Auth | Default Feature |
|---------|----------|------|:---:|
| **Salesforce** | REST v59.0 + SOQL | OAuth2 | ❌ |
| **HubSpot** | CRM API v3 | Private App Token | ✅ |
| **Zoho CRM** | REST API v2 | OAuth2 | ❌ |
| **Pipedrive** | REST API v1 | API Token | ✅ |

### Backend Capabilities

| Capability | Salesforce | HubSpot | Zoho CRM | Pipedrive |
|-----------|:---:|:---:|:---:|:---:|
| Contacts CRUD | ✅ | ✅ | ✅ | ✅ |
| Contact search | ✅ (SOQL) | ✅ (native) | ✅ (criteria) | ✅ (term) |
| Contact delete | ✅ | ✅ | ✅ | ✅ |
| Companies CRUD | ✅ | ✅ | ✅ | ✅ |
| Company search | ✅ (SOQL) | ✅ (native) | ✅ (criteria) | ✅ (term) |
| Deals CRUD | ✅ | ✅ | ✅ | ✅ |
| Deal search | ✅ (SOQL) | ✅ (native) | ✅ (criteria) | ✅ (term) |
| Deal delete | ✅ | ✅ | ✅ | ✅ |
| Pipeline stages | ✅ | ✅ | ✅ | ✅ (with counts) |
| Activities | ✅ (Task) | ✅ (Engagement) | ✅ (Task) | ✅ (Activity) |
| Activity update | ✅ | ✅ | ✅ | ✅ |
| Notes | ✅ | ✅ | ✅ | ✅ |
| Associations | ✅ (ContactRole) | ✅ (v3 assoc) | ✅ (lookup) | ✅ (participants) |

### API Mapping

| Entity | Salesforce | HubSpot | Zoho CRM | Pipedrive |
|--------|-----------|---------|----------|-----------|
| Contact | Contact | contacts | Contacts | persons |
| Company | Account | companies | Accounts | organizations |
| Deal | Opportunity | deals | Deals | deals |
| Activity | Task | tasks/engagements | Tasks | activities |
| Note | Note | notes | Notes | notes |
| Pipeline | OpportunityStage | pipelines | settings/pipeline | pipelines + stages |

## Installation

```bash
cargo install mcp-crm --features all-backends
```

Or build from source:

```bash
git clone https://github.com/zavora-ai/mcp-crm
cd mcp-crm
cargo build --release --features all-backends
```

### Feature flags

```bash
# Default: HubSpot + Pipedrive (lightest, token-based auth)
cargo install mcp-crm

# All backends
cargo install mcp-crm --features all-backends

# Specific backends
cargo install mcp-crm --no-default-features --features salesforce
cargo install mcp-crm --no-default-features --features "hubspot,zoho-crm"
```

## Configuration

### Salesforce

```bash
export SALESFORCE_INSTANCE_URL="https://yourorg.my.salesforce.com"
export SALESFORCE_TOKEN="00D..."
```

### HubSpot

```bash
export HUBSPOT_TOKEN="pat-na1-xxxxxxxx"
```

### Zoho CRM

```bash
export ZOHO_CRM_TOKEN="1000.xxxxxxxx"
```

### Pipedrive

```bash
export PIPEDRIVE_TOKEN="xxxxxxxxxxxxxxxx"
```

## Client Configuration

### Claude Desktop

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

### Kiro

Add to `.kiro/settings/mcp.json`:

```json
{
  "mcpServers": {
    "crm": {
      "command": "mcp-crm",
      "args": [],
      "env": {
        "PIPEDRIVE_TOKEN": "xxxx"
      }
    }
  }
}
```

### Cursor

Add to `.cursor/mcp.json`:

```json
{
  "mcpServers": {
    "crm": {
      "command": "mcp-crm",
      "args": [],
      "env": {
        "SALESFORCE_INSTANCE_URL": "https://yourorg.my.salesforce.com",
        "SALESFORCE_TOKEN": "00D..."
      }
    }
  }
}
```

### Windsurf

Add to `~/.codeium/windsurf/mcp_config.json`:

```json
{
  "mcpServers": {
    "crm": {
      "command": "mcp-crm",
      "args": [],
      "env": {
        "ZOHO_CRM_TOKEN": "1000.xxxx"
      }
    }
  }
}
```

## Usage Examples

### Contact management
```
"List my top 10 contacts"
→ list_contacts(limit: 10)

"Search for contacts at Acme Corp"
→ search_contacts(query: "Acme")

"Create a contact for Sarah Chen at Acme, email sarah@acme.com"
→ create_contact(first_name: "Sarah", last_name: "Chen", email: "sarah@acme.com")

"Link Sarah to the Acme company"
→ associate_contact_company(contact_id: "sarah-id", company_id: "acme-id")

"Delete the test contact"
→ delete_contact(id: "test-id")
```

### Deal pipeline
```
"Show me all open deals"
→ list_deals(limit: 50)

"Search for deals related to enterprise"
→ search_deals(query: "enterprise")

"Create a $50k deal for the Acme enterprise contract"
→ create_deal(name: "Acme Enterprise", amount: 50000, stage: "Negotiation")

"Link Sarah to the Acme deal"
→ associate_deal_contact(deal_id: "deal-123", contact_id: "sarah-id")

"Who's involved in the Acme deal?"
→ list_deal_contacts(id: "deal-123")

"Move the Acme deal to Closed Won"
→ move_deal_stage(id: "deal-123", stage: "closedwon")

"What does our pipeline look like?"
→ get_pipeline_summary()
```

### Companies
```
"Search for companies in fintech"
→ search_companies(query: "fintech")
```

### Activity logging
```
"Log a call with Sarah about the proposal"
→ create_activity(activity_type: "call", subject: "Proposal discussion", contact_id: "sarah-id")

"Mark that task as done"
→ update_activity(id: "activity-id", done: true)

"Show all activities for the Acme deal"
→ list_activities(deal_id: "deal-123")
```

### Notes
```
"Add a note to the Acme deal: 'Budget approved by CFO'"
→ create_note(content: "Budget approved by CFO", deal_id: "deal-123")
```

## Documentation

| Document | Description |
|----------|-------------|
| [Architecture](docs/assets/architecture.svg) | System diagram |
| [mcp-server.toml](mcp-server.toml) | ADK-Rust Enterprise registry manifest |
| [CHANGELOG.md](CHANGELOG.md) | Version history |
| [LICENSE](LICENSE) | Apache-2.0 license |

## Registry Compliance

This server implements the [ADK MCP SDK](https://crates.io/crates/adk-mcp-sdk) contract:

- **HealthCheck** — verifies backend connectivity on startup
- **mcp-server.toml** — manifest with 28 tools, risk classes, and credential bindings
- **Manifest validation** — startup fails fast on invalid manifest (SDK 0.1.3+)
- **Structured tracing** — `RUST_LOG` env-filter for observability

## Contributors

<!-- ALL-CONTRIBUTORS-LIST:START -->
| [<img src="https://github.com/jkmaina.png" width="80px;" alt=""/><br /><sub><b>James Karanja Maina</b></sub>](https://github.com/jkmaina) |
|:---:|
<!-- ALL-CONTRIBUTORS-LIST:END -->

## License

Apache-2.0 — see [LICENSE](LICENSE) for details.

---

Part of the [ADK-Rust Enterprise](https://enterprise.adk-rust.com) MCP server ecosystem.

Built with ❤️ by [Zavora AI](https://zavora.ai)

## rmcp and MCP compatibility

This server is built with [`rmcp` 3.1.2](https://github.com/modelcontextprotocol/rust-sdk/releases/tag/rmcp-v3.1.2) and requires Rust 1.88 or newer. The rmcp 3 rollout retains legacy MCP initialization compatibility and targets MCP protocol revisions `2025-11-25` and `2026-07-28`.
