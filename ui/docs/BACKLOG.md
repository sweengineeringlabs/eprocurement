# eProcurement UI Backlog

## Pending

### Migrate eProcurement components to use Rustscript framework versions

**Priority:** Low
**Effort:** Medium

The eProcurement UI currently has its own implementations of components that have been promoted to the Rustscript framework. These local implementations coexist with the framework versions but should eventually be migrated to use the framework components for consistency, maintainability, and to benefit from the enhanced accessibility features.

**Components to migrate:**
- `data_table.rs` → `DataTable` from `ui.rs`
- `stepper.rs` → `Stepper` from `ui.rs`
- `timeline.rs` → `Timeline` from `ui.rs`
- `kpi_card.rs` → `KpiCard` from `ui.rs`
- `pagination.rs` → `Pagination` from `ui.rs`
- `panel.rs` → `Panel` from `ui.rs`
- `notice_bar.rs` → `NoticeBar` from `ui.rs`
- `empty_state.rs` → `EmptyState` from `ui.rs`
- `tag.rs` → `Tag` from `ui.rs`

**Migration steps:**
1. Update imports in eProcurement to use `components::prelude::*` versions
2. Adapt call sites to match framework component APIs (may require minor refactoring)
3. Remove local component implementations from `src/shared/components/`
4. Update `src/shared/components/mod.rs` to re-export from framework if needed for backwards compatibility

**Benefits:**
- Enhanced ARIA accessibility attributes
- Keyboard navigation support
- Consistent API across projects
- Single source of truth for component behavior

---
