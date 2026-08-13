# Changelog

## [1.2.0] - 2026-08-13

### Changed
- Upgraded to rmcp 3.1.2 and raised the minimum supported Rust version to 1.94.1.
- Added MCP 2026-07-28 stateless request handling while retaining MCP 2025-11-25 initialization compatibility.

### Added
- Per-request identity and protocol metadata, on-demand discovery/cache hints, and the configured Tasks and sealed MRTR approval policies.

## [1.1.0] - 2026-05-25

### Added
- `delete_contact` — delete a contact by ID (external_write, requires approval)
- `delete_deal` — delete a deal by ID (external_write, requires approval)
- `associate_contact_company` — link a contact to a company
- `associate_deal_contact` — link a deal to a contact
- `list_deal_contacts` — list contacts associated with a deal
- `search_companies` — search companies by name or domain
- `search_deals` — search deals by name, stage, or value
- `update_activity` — mark activities done, change subject

### Changed
- Total tools: 20 → 28

## [1.0.0] - 2026-05-25

### Added
- Initial release with 20 tools across 6 categories
- **4 backends:** Salesforce, HubSpot, Zoho CRM, Pipedrive
- **Unified CrmBackend trait** — all backends implement the same interface
- Contacts: list, get, create, update, search
- Companies: list, get, create, update
- Deals: list, get, create, update, move_stage
- Activities: list, create (call, email, meeting, task, note)
- Pipelines: list, get_summary with stage counts/values
- Notes: list, create (attached to contacts, companies, or deals)
- Feature flags — compile only the backends you need (default: hubspot + pipedrive)
- Manifest validation on startup (adk-mcp-sdk 0.1.3)
- Health check verifies backend connectivity
- Architecture SVG diagram
- Comprehensive README with client configs
- API reference and backend setup guides
