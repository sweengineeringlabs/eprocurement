# Software Requirements Specification (SRS)
# Analytics & Reporting Module

**Document Version:** 1.0
**Date:** 27 February 2026
**Project:** GovProcure eProcurement Platform
**Module:** Analytics, Dashboards & Reporting

---

## Table of Contents

1. [Introduction](#1-introduction)
2. [Overall Description](#2-overall-description)
3. [Functional Requirements](#3-functional-requirements)
4. [Non-Functional Requirements](#4-non-functional-requirements)
5. [User Interface Requirements](#5-user-interface-requirements)
6. [Data Requirements](#6-data-requirements)
7. [Use Cases](#7-use-cases)
8. [Business Rules](#8-business-rules)

---

## 1. Introduction

### 1.1 Purpose

This SRS document describes the requirements for the Analytics & Reporting module of the GovProcure eProcurement Platform. This module provides insights, visualizations, and reports across all procurement activities.

### 1.2 Scope

The Analytics module encompasses:

- Executive dashboards
- Procurement performance metrics
- Spend analysis
- Supplier analytics
- Compliance reporting
- Custom report builder
- Scheduled report distribution
- Data export capabilities

### 1.3 Definitions

| Term | Definition |
|------|------------|
| KPI | Key Performance Indicator |
| MTD | Month-to-Date |
| YTD | Year-to-Date |
| Drill-down | Ability to view detailed data from summary |
| Widget | Self-contained dashboard component |

---

## 2. Overall Description

### 2.1 Product Perspective

```
┌─────────────────────────────────────────────────────────────────┐
│                    ANALYTICS ARCHITECTURE                        │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐     │
│  │ Data     │ → │ Process  │ → │ Analyze  │ → │ Present  │     │
│  │ Sources  │   │ & Store  │   │ & Model  │   │ & Report │     │
│  └──────────┘   └──────────┘   └──────────┘   └──────────┘     │
│       │              │              │              │            │
│       ▼              ▼              ▼              ▼            │
│  ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐     │
│  │Requistn  │   │Data      │   │ KPIs &   │   │Dashboards│     │
│  │Tenders   │   │Warehouse │   │ Metrics  │   │ Reports  │     │
│  │Invoices  │   │          │   │          │   │          │     │
│  │Suppliers │   │          │   │          │   │          │     │
│  └──────────┘   └──────────┘   └──────────┘   └──────────┘     │
└─────────────────────────────────────────────────────────────────┘
```

### 2.2 User Classes

| User Class | Description | Access Level |
|------------|-------------|--------------|
| Executive | Strategic overview dashboards | Executive views |
| Procurement Manager | Operational analytics | Full analytics |
| Analyst | Report creation and analysis | Full access |
| Department User | Department-specific views | Limited scope |

---

## 3. Functional Requirements

### 3.1 Executive Dashboard

#### FR-3.1.1 KPI Widgets
| ID | FR-3.1.1 |
|----|----------|
| **Description** | System shall display key performance indicators |
| **Priority** | High |

**Standard KPIs:**
| KPI | Description | Target |
|-----|-------------|--------|
| Total Spend | Cumulative procurement spend | vs Budget |
| Active POs | Open purchase orders | Count |
| Pending Approvals | Items awaiting approval | < 5 days |
| Supplier Count | Active supplier base | Growth % |
| Savings Realized | Negotiated savings | vs List price |
| Compliance Rate | Policy adherence | > 95% |

#### FR-3.1.2 Trend Charts
| ID | FR-3.1.2 |
|----|----------|
| **Description** | System shall display trend visualizations |
| **Priority** | High |

**Chart Types:**
- Line charts (trends over time)
- Bar charts (comparisons)
- Pie/Donut charts (composition)
- Area charts (cumulative values)

#### FR-3.1.3 Dashboard Customization
| ID | FR-3.1.3 |
|----|----------|
| **Description** | System shall allow dashboard personalization |
| **Priority** | Medium |

**Customization Options:**
- Add/remove widgets
- Rearrange widget positions
- Set date range filters
- Choose visualization types
- Save custom layouts

### 3.2 Spend Analysis

#### FR-3.2.1 Spend by Category
| ID | FR-3.2.1 |
|----|----------|
| **Description** | System shall analyze spend by category |
| **Priority** | High |

**Analysis Dimensions:**
- Product/Service category
- Supplier
- Department/Cost Centre
- Time period
- Contract vs Non-contract

#### FR-3.2.2 Spend Trends
| ID | FR-3.2.2 |
|----|----------|
| **Description** | System shall show spend trends |
| **Priority** | High |

**Trend Views:**
- Monthly comparison
- Year-over-year
- Budget vs Actual
- Forecast projection

#### FR-3.2.3 Maverick Spend Detection
| ID | FR-3.2.3 |
|----|----------|
| **Description** | System shall identify off-contract spending |
| **Priority** | Medium |

**Detection Criteria:**
- Purchases from non-preferred suppliers
- Purchases without PO
- Purchases exceeding contract rates

### 3.3 Supplier Analytics

#### FR-3.3.1 Supplier Performance
| ID | FR-3.3.1 |
|----|----------|
| **Description** | System shall track supplier metrics |
| **Priority** | High |

**Performance Metrics:**
| Metric | Description |
|--------|-------------|
| On-time Delivery | % orders delivered on time |
| Quality Score | Based on rejections/returns |
| Price Competitiveness | vs Market benchmarks |
| Response Time | Average quote turnaround |
| Invoice Accuracy | % invoices without errors |

#### FR-3.3.2 Supplier Spend Distribution
| ID | FR-3.3.2 |
|----|----------|
| **Description** | System shall show spend concentration |
| **Priority** | Medium |

**Analysis Views:**
- Top 10 suppliers by spend
- Pareto analysis (80/20)
- Supplier diversity metrics
- B-BBEE spend breakdown

#### FR-3.3.3 Supplier Risk Analysis
| ID | FR-3.3.3 |
|----|----------|
| **Description** | System shall assess supplier risk |
| **Priority** | Medium |

**Risk Indicators:**
- Single-source dependencies
- Financial health indicators
- Geographic concentration
- Contract expiration clustering

### 3.4 Procurement Efficiency

#### FR-3.4.1 Cycle Time Analysis
| ID | FR-3.4.1 |
|----|----------|
| **Description** | System shall measure process cycle times |
| **Priority** | High |

**Cycle Times:**
| Process | Measurement |
|---------|-------------|
| Requisition to PO | Days from request to order |
| Tender Duration | Days from publish to award |
| Invoice Processing | Days from receipt to payment |
| Approval Time | Days in approval queue |

#### FR-3.4.2 Bottleneck Identification
| ID | FR-3.4.2 |
|----|----------|
| **Description** | System shall identify process delays |
| **Priority** | Medium |

**Bottleneck Indicators:**
- Items pending > SLA threshold
- Approval queue depth
- Rejection rates by stage
- Rework frequency

### 3.5 Compliance Reporting

#### FR-3.5.1 Policy Compliance
| ID | FR-3.5.1 |
|----|----------|
| **Description** | System shall track policy adherence |
| **Priority** | High |

**Compliance Metrics:**
- % purchases through approved channels
- % competitive bids obtained
- % preferred supplier usage
- % timely approvals

#### FR-3.5.2 Regulatory Compliance
| ID | FR-3.5.2 |
|----|----------|
| **Description** | System shall report regulatory metrics |
| **Priority** | High |

**Regulatory Reports:**
- B-BBEE spending targets
- Local content requirements
- Women/Youth-owned business spend
- Small business participation

#### FR-3.5.3 Audit Trail Reports
| ID | FR-3.5.3 |
|----|----------|
| **Description** | System shall generate audit reports |
| **Priority** | High |

**Audit Data:**
- User actions log
- Approval decisions
- Document modifications
- System access history

### 3.6 Report Builder

#### FR-3.6.1 Custom Report Creation
| ID | FR-3.6.1 |
|----|----------|
| **Description** | System shall allow custom report design |
| **Priority** | Medium |

**Builder Features:**
- Drag-and-drop field selection
- Filter configuration
- Grouping and sorting
- Calculated fields
- Chart embedding

#### FR-3.6.2 Report Templates
| ID | FR-3.6.2 |
|----|----------|
| **Description** | System shall provide report templates |
| **Priority** | Medium |

**Standard Templates:**
- Monthly Procurement Summary
- Supplier Scorecard
- Budget Utilization Report
- Outstanding Payments Report
- Contract Expiry Report

#### FR-3.6.3 Report Scheduling
| ID | FR-3.6.3 |
|----|----------|
| **Description** | System shall schedule report delivery |
| **Priority** | Medium |

**Schedule Options:**
- Daily/Weekly/Monthly
- Specific day and time
- Email distribution list
- Multiple formats (PDF, Excel, CSV)

### 3.7 Data Export

#### FR-3.7.1 Export Formats
| ID | FR-3.7.1 |
|----|----------|
| **Description** | System shall export data in multiple formats |
| **Priority** | High |

**Supported Formats:**
| Format | Use Case |
|--------|----------|
| Excel (.xlsx) | Further analysis |
| CSV | Data integration |
| PDF | Formal reports |
| JSON | API consumption |

#### FR-3.7.2 Bulk Export
| ID | FR-3.7.2 |
|----|----------|
| **Description** | System shall support bulk data export |
| **Priority** | Medium |

---

## 4. Non-Functional Requirements

### 4.1 Performance

| ID | Requirement | Target |
|----|-------------|--------|
| NFR-4.1.1 | Dashboard load time | < 3 seconds |
| NFR-4.1.2 | Report generation | < 30 seconds |
| NFR-4.1.3 | Data refresh frequency | Every 15 minutes |
| NFR-4.1.4 | Concurrent users | 100+ |

### 4.2 Data Quality

| ID | Requirement |
|----|-------------|
| NFR-4.2.1 | Data accuracy > 99.9% |
| NFR-4.2.2 | Historical data retention: 7 years |
| NFR-4.2.3 | Data reconciliation with source systems |

### 4.3 Usability

| ID | Requirement |
|----|-------------|
| NFR-4.3.1 | Mobile-responsive dashboards |
| NFR-4.3.2 | Intuitive drill-down navigation |
| NFR-4.3.3 | Contextual help on all metrics |

---

## 5. User Interface Requirements

### 5.1 Analytics Dashboard

```
┌─────────────────────────────────────────────────────────────────┐
│ Analytics                          [Date Range ▼] [Export]      │
├─────────────────────────────────────────────────────────────────┤
│ ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────────┐│
│ │  R 24.7M    │ │    156      │ │    94.2%    │ │   R 2.1M    ││
│ │ Total Spend │ │ Active POs  │ │ On-Time %   │ │   Savings   ││
│ │   ↑ 12%     │ │   ↓ 8       │ │   ↑ 2.1%    │ │   ↑ 15%     ││
│ └─────────────┘ └─────────────┘ └─────────────┘ └─────────────┘│
├─────────────────────────────────────────────────────────────────┤
│ Monthly Spend Trend                                             │
│ ┌─────────────────────────────────────────────────────────────┐│
│ │    ▄                                                        ││
│ │   ▄█▄     ▄                                          ▄      ││
│ │  ▄███▄   ▄█▄   ▄▄    ▄▄    ▄▄   ▄▄    ▄▄   ▄▄    ▄▄▄█▄     ││
│ │ ▄█████▄ ▄███▄ ▄██▄  ▄██▄  ▄██▄ ▄██▄  ▄██▄ ▄██▄  ▄█████▄    ││
│ │ Jul  Aug  Sep  Oct  Nov  Dec  Jan  Feb  Mar  Apr  May  Jun  ││
│ └─────────────────────────────────────────────────────────────┘│
├───────────────────────────────────┬─────────────────────────────┤
│ Spend by Category                 │ Top Suppliers               │
│ ┌───────────────────────────────┐ │ ┌─────────────────────────┐│
│ │        ╭───────╮              │ │ │ SAP Africa    R 4.2M    ││
│ │      ╭─┤ IT 35%├─╮            │ │ │ ████████████████        ││
│ │     ╭┤ ╰───────╯ ├╮           │ │ │ TechServ      R 2.8M    ││
│ │    ╭┤Prof Svcs 25├─╮          │ │ │ ██████████████          ││
│ │   ╭─┤────────────├──╮         │ │ │ BuildRight    R 1.9M    ││
│ │  ╭──┤ Ops 20%    ├───╮        │ │ │ ██████████              ││
│ │ ╭───┤────────────├────╮       │ │ │ Datacom       R 1.4M    ││
│ │ │   │ Other 20%  │    │       │ │ │ ████████                ││
│ └───────────────────────────────┘ │ └─────────────────────────┘│
└───────────────────────────────────┴─────────────────────────────┘
```

### 5.2 Report Builder Interface

```
┌─────────────────────────────────────────────────────────────────┐
│ Report Builder                              [Preview] [Save]    │
├─────────────────────────────────────────────────────────────────┤
│ ┌───────────────┐ ┌───────────────────────────────────────────┐│
│ │ Available     │ │ Report Canvas                             ││
│ │ Fields        │ │ ┌───────────────────────────────────────┐ ││
│ │               │ │ │ [Title: Monthly Procurement Report  ] │ ││
│ │ ☐ PO Number   │ │ └───────────────────────────────────────┘ ││
│ │ ☐ PO Date     │ │                                           ││
│ │ ☐ Supplier    │ │ Columns: [PO #] [Supplier] [Amount] [Date]││
│ │ ☐ Amount      │ │                                           ││
│ │ ☐ Status      │ │ Filters:                                  ││
│ │ ☐ Department  │ │ ┌─────────────────────────────────────┐   ││
│ │ ☐ Category    │ │ │ Date Range: [This Month      ▼]    │   ││
│ │ ☐ Requester   │ │ │ Status:     [All             ▼]    │   ││
│ │               │ │ └─────────────────────────────────────┘   ││
│ │ [+ Add Field] │ │                                           ││
│ │               │ │ Group By: [Department ▼]                  ││
│ │               │ │ Sort By:  [Amount (Desc) ▼]               ││
│ └───────────────┘ └───────────────────────────────────────────┘│
├─────────────────────────────────────────────────────────────────┤
│ Schedule: [☐ Enable] [Weekly ▼] [Monday ▼] [08:00 ▼]           │
│ Recipients: [finance@org.gov.za, procurement@org.gov.za    ]   │
└─────────────────────────────────────────────────────────────────┘
```

---

## 6. Data Requirements

### 6.1 Dashboard Widget Entity

| Field | Type | Description |
|-------|------|-------------|
| widget_id | VARCHAR(20) | Primary key |
| dashboard_id | VARCHAR(20) | Parent dashboard |
| widget_type | ENUM | Chart/KPI/Table/Custom |
| title | VARCHAR(100) | Widget title |
| data_source | VARCHAR(50) | Data query reference |
| config | JSON | Widget configuration |
| position_x | INT | Grid X position |
| position_y | INT | Grid Y position |
| width | INT | Grid width units |
| height | INT | Grid height units |

### 6.2 Report Definition Entity

| Field | Type | Description |
|-------|------|-------------|
| report_id | VARCHAR(20) | Primary key |
| name | VARCHAR(100) | Report name |
| description | TEXT | Report description |
| created_by | VARCHAR(20) | Creator |
| columns | JSON | Column definitions |
| filters | JSON | Filter criteria |
| grouping | JSON | Group by fields |
| sorting | JSON | Sort configuration |
| schedule | JSON | Schedule settings |
| recipients | JSON | Email recipients |
| is_template | BOOLEAN | Template flag |

### 6.3 Analytics Metric Entity

| Field | Type | Description |
|-------|------|-------------|
| metric_id | VARCHAR(20) | Primary key |
| name | VARCHAR(100) | Metric name |
| description | TEXT | Metric description |
| calculation | TEXT | Calculation formula |
| data_source | VARCHAR(50) | Source table/view |
| refresh_interval | INT | Minutes between refresh |
| threshold_warning | DECIMAL | Warning threshold |
| threshold_critical | DECIMAL | Critical threshold |

---

## 7. Use Cases

### UC-01: View Executive Dashboard

**Actor:** Executive User

**Main Flow:**
1. User logs into system
2. System displays default executive dashboard
3. User views KPI summary widgets
4. User clicks on KPI for drill-down
5. System shows detailed breakdown
6. User adjusts date range filter
7. System refreshes all widgets
8. User exports dashboard as PDF

### UC-02: Create Custom Report

**Actor:** Analyst

**Main Flow:**
1. Analyst opens Report Builder
2. Analyst selects data source
3. Analyst drags fields to report canvas
4. Analyst configures filters and grouping
5. Analyst previews report
6. Analyst adjusts formatting
7. Analyst saves report
8. Analyst configures schedule (optional)
9. System sends report per schedule

### UC-03: Analyze Supplier Performance

**Actor:** Procurement Manager

**Main Flow:**
1. Manager opens Supplier Analytics
2. Manager selects supplier
3. System displays performance scorecard
4. Manager views trend charts
5. Manager compares to category average
6. Manager identifies improvement areas
7. Manager exports supplier report
8. Manager schedules review meeting

---

## 8. Business Rules

### BR-01: Data Access Control
- Users see only data for their authorized scope
- Executives see organization-wide data
- Department users see department data only
- Sensitive financial data requires Finance role

### BR-02: Metric Calculations
- All financial metrics in base currency (ZAR)
- Percentages calculated to 1 decimal place
- Averages exclude null/zero values
- Trends compare same period prior year

### BR-03: Dashboard Refresh
- Real-time data refresh every 15 minutes
- Users can force manual refresh
- Heavy reports run during off-peak hours
- Cached data shows refresh timestamp

### BR-04: Report Distribution
- Scheduled reports run between 06:00-08:00
- Failed reports retry 3 times
- Large reports (>10MB) send download link
- Recipients must have system access

### BR-05: Data Retention
- Operational data: Current + 2 years online
- Historical data: 7 years archived
- Audit logs: Permanent retention
- User-created reports: Until deleted

---

**End of Document**
