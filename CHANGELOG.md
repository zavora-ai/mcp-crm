# Changelog

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
