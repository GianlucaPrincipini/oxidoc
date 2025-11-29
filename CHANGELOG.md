

# 📝 CHANGELOG — oxidoc Devlogs

This file tracks the progress of **oxidoc** through incremental devlogs.  
Each devlog corresponds to a milestone, feature, or design exploration.

---

## Devlog #1 — Initial In-Memory Engine + Basic TCP Server  
**Status:** ✔️ Completed  
**Date:** YYYY-MM-DD

### Added
- Initial JSON-based `Document` type using `serde_json`
- `Collection`: in-memory key → JSON map
- `Database`: collection manager with simple CRUD
- Minimal TCP server accepting client connections
- Basic “Welcome to oxidoc!” handshake
- Tests for server connection + in-memory engine

### Notes
This devlog establishes the foundation:
- in-memory storage only  
- no protocol  
- no persistence  
- minimal networking  

---

## Versioning

This project uses *devlog-based* semantic evolution rather than strict SemVer.  
Version numbers are assigned as milestones are completed.

---

## Author Notes

These notes serve as a transparent, educational history of the database’s design, failures, rewrites, and experiments.  
oxidoc is intentionally a learning tool — not a production system.


