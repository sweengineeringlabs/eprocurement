# Software Requirements Specification (SRS)
# Budget Control Module

**Document Version:** 1.0
**Date:** 27 February 2026
**Project:** GovProcure eProcurement Platform
**Module:** Budget Management & Control

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

This SRS document describes the requirements for the Budget Control module of the GovProcure eProcurement Platform. This module manages budget allocation, tracking, and enforcement across procurement activities.

### 1.2 Scope

The Budget Control module encompasses:

- Budget setup and allocation
- Cost centre management
- Budget commitment tracking
- Expenditure monitoring
- Budget variance analysis
- Budget transfer processing
- Year-end processing

### 1.3 Definitions

| Term | Definition |
|------|------------|
| Commitment | Funds reserved for approved requisitions/POs |
| Encumbrance | Legal obligation to pay (contracted amount) |
| Variance | Difference between budgeted and actual spend |
| Cost Centre | Organizational unit responsible for budget |
| Vote | Allocated budget amount for specific purpose |

---

## 2. Overall Description

### 2.1 Product Perspective

```
┌─────────────────────────────────────────────────────────────────┐
│                    BUDGET CONTROL FLOW                          │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐     │
│  │ Allocate │ → │ Commit   │ → │ Encumber │ → │ Expend   │     │
│  │ Budget   │   │ Funds    │   │ Contract │   │ Payment  │     │
│  └──────────┘   └──────────┘   └──────────┘   └──────────┘     │
│       │              │              │              │            │
│       ▼              ▼              ▼              ▼            │
│  ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐     │
│  │Available │   │ Reserved │   │Contracted│   │  Spent   │     │
│  │          │   │          │   │          │   │          │     │
│  └──────────┘   └──────────┘   └──────────┘   └──────────┘     │
└─────────────────────────────────────────────────────────────────┘
```

### 2.2 User Classes

| User Class | Description | Access Level |
|------------|-------------|--------------|
| Budget Officer | Allocates and manages budgets | Full access |
| Cost Centre Manager | Views and monitors cost centre budget | View, Request transfers |
| Finance Manager | Approves transfers and adjustments | Approve |
| CFO | Strategic budget oversight | Full oversight |

---

## 3. Functional Requirements

### 3.1 Budget Setup

#### FR-3.1.1 Create Budget Structure
| ID | FR-3.1.1 |
|----|----------|
| **Description** | System shall allow creation of budget hierarchy |
| **Priority** | High |

**Budget Levels:**
| Level | Description |
|-------|-------------|
| Organization | Top-level entity budget |
| Department | Departmental allocation |
| Cost Centre | Operational unit budget |
| Vote | Specific purpose allocation |

#### FR-3.1.2 Budget Allocation
| ID | FR-3.1.2 |
|----|----------|
| **Description** | System shall allocate funds to budget items |
| **Priority** | High |

**Allocation Fields:**
| Field | Type | Required |
|-------|------|----------|
| Financial Year | Select | Yes |
| Cost Centre | Select | Yes |
| Budget Category | Select | Yes |
| Allocated Amount | Currency | Yes |
| Effective Date | Date | Yes |
| Description | Text | No |

#### FR-3.1.3 Budget Categories
| ID | FR-3.1.3 |
|----|----------|
| **Description** | System shall support budget categorization |
| **Priority** | High |

**Standard Categories:**
- Capital Expenditure (CAPEX)
- Operational Expenditure (OPEX)
- Personnel Costs
- IT & Technology
- Professional Services
- Maintenance & Repairs
- Travel & Subsistence

### 3.2 Commitment Management

#### FR-3.2.1 Budget Check
| ID | FR-3.2.1 |
|----|----------|
| **Description** | System shall verify budget availability |
| **Priority** | High |

**Check Points:**
- Requisition creation
- PO creation
- Contract award
- Invoice approval

#### FR-3.2.2 Commitment Creation
| ID | FR-3.2.2 |
|----|----------|
| **Description** | System shall create budget commitments |
| **Priority** | High |

**Commitment Sources:**
| Source | Trigger |
|--------|---------|
| Requisition | On approval |
| Purchase Order | On creation |
| Contract | On award |

#### FR-3.2.3 Commitment Release
| ID | FR-3.2.3 |
|----|----------|
| **Description** | System shall release commitments |
| **Priority** | High |

**Release Triggers:**
- Requisition cancelled
- PO closed/cancelled
- Invoice paid (converts to expenditure)
- Contract completed

### 3.3 Expenditure Tracking

#### FR-3.3.1 Record Expenditure
| ID | FR-3.3.1 |
|----|----------|
| **Description** | System shall record actual expenditure |
| **Priority** | High |

**Expenditure Sources:**
- Invoice payments
- Direct payments
- Petty cash
- Corporate card transactions

#### FR-3.3.2 Budget Utilization
| ID | FR-3.3.2 |
|----|----------|
| **Description** | System shall calculate utilization |
| **Priority** | High |

**Utilization Formula:**
```
Utilization % = (Committed + Expended) / Allocated × 100
Available = Allocated - Committed - Expended
```

### 3.4 Budget Transfers

#### FR-3.4.1 Request Transfer
| ID | FR-3.4.1 |
|----|----------|
| **Description** | System shall allow budget transfer requests |
| **Priority** | Medium |

**Transfer Fields:**
| Field | Type | Required |
|-------|------|----------|
| From Cost Centre | Select | Yes |
| From Category | Select | Yes |
| To Cost Centre | Select | Yes |
| To Category | Select | Yes |
| Amount | Currency | Yes |
| Justification | Text | Yes |

#### FR-3.4.2 Transfer Approval
| ID | FR-3.4.2 |
|----|----------|
| **Description** | System shall route transfers for approval |
| **Priority** | Medium |

**Approval Matrix:**
| Transfer Amount | Approver |
|-----------------|----------|
| < R 100,000 | Finance Manager |
| R 100,000 - R 500,000 | CFO |
| > R 500,000 | CFO + Board |

### 3.5 Variance Analysis

#### FR-3.5.1 Calculate Variance
| ID | FR-3.5.1 |
|----|----------|
| **Description** | System shall calculate budget variances |
| **Priority** | Medium |

**Variance Types:**
| Type | Formula |
|------|---------|
| Absolute | Budget - Actual |
| Percentage | ((Budget - Actual) / Budget) × 100 |
| YTD | Year-to-date variance |
| Forecast | Projected year-end variance |

#### FR-3.5.2 Variance Alerts
| ID | FR-3.5.2 |
|----|----------|
| **Description** | System shall alert on variance thresholds |
| **Priority** | Medium |

**Alert Thresholds:**
| Utilization | Alert Level |
|-------------|-------------|
| > 75% | Warning (Yellow) |
| > 90% | Critical (Orange) |
| > 100% | Exceeded (Red) |

### 3.6 Reporting

#### FR-3.6.1 Budget Summary Report
| ID | FR-3.6.1 |
|----|----------|
| **Description** | System shall generate budget summary |
| **Priority** | High |

**Report Columns:**
- Cost Centre
- Category
- Allocated
- Committed
- Expended
- Available
- Utilization %

#### FR-3.6.2 Commitment Report
| ID | FR-3.6.2 |
|----|----------|
| **Description** | System shall list all commitments |
| **Priority** | Medium |

#### FR-3.6.3 Variance Report
| ID | FR-3.6.3 |
|----|----------|
| **Description** | System shall report budget variances |
| **Priority** | Medium |

---

## 4. Non-Functional Requirements

### 4.1 Performance

| ID | Requirement | Target |
|----|-------------|--------|
| NFR-4.1.1 | Budget check response | < 1 second |
| NFR-4.1.2 | Utilization calculation | Real-time |
| NFR-4.1.3 | Report generation | < 10 seconds |

### 4.2 Security

| ID | Requirement |
|----|-------------|
| NFR-4.2.1 | Budget modifications require approval |
| NFR-4.2.2 | Full audit trail on all changes |
| NFR-4.2.3 | Role-based access to budget data |

### 4.3 Integration

| ID | Requirement |
|----|-------------|
| NFR-4.3.1 | Integrate with financial system (ERP) |
| NFR-4.3.2 | Real-time sync with requisition module |
| NFR-4.3.3 | Connect to invoice payment system |

---

## 5. User Interface Requirements

### 5.1 Budget Dashboard

```
┌─────────────────────────────────────────────────────────────────┐
│ Budget Control                                  FY 2025/2026 ▼  │
├─────────────────────────────────────────────────────────────────┤
│ ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────────┐│
│ │  R 45.2M    │ │   R 28.1M   │ │   R 12.4M   │ │   R 4.7M    ││
│ │ Total Budget│ │  Committed  │ │   Expended  │ │  Available  ││
│ │             │ │    62.2%    │ │    27.4%    │ │    10.4%    ││
│ └─────────────┘ └─────────────┘ └─────────────┘ └─────────────┘│
├─────────────────────────────────────────────────────────────────┤
│ Budget by Category                                              │
├─────────────────────────────────────────────────────────────────┤
│ IT & Technology    ████████████████░░░░  R 12.4M / R 15M   83% │
│ Professional Svcs  ██████████████░░░░░░  R 8.2M  / R 12M   68% │
│ Operations         █████████░░░░░░░░░░░  R 4.5M  / R 10M   45% │
│ Capital Projects   ███████░░░░░░░░░░░░░  R 2.8M  / R 8M    35% │
│                                                                 │
├─────────────────────────────────────────────────────────────────┤
│ ⚠ Budget Alerts                                         View All│
├─────────────────────────────────────────────────────────────────┤
│ 🔴 IT Hardware exceeds budget by R 245,000                     │
│ 🟠 Professional Services at 92% utilization                    │
│ 🟡 Travel budget at 78% with 4 months remaining                │
└─────────────────────────────────────────────────────────────────┘
```

### 5.2 Budget Transfer Form

```
┌─────────────────────────────────────────────────────────────────┐
│ Request Budget Transfer                                    [X]  │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  TRANSFER FROM                                                   │
│  ┌────────────────────────────┐  ┌────────────────────────────┐ │
│  │ Cost Centre         ▼     │  │ Category              ▼   │ │
│  │ Operations Division       │  │ Travel & Subsistence      │ │
│  └────────────────────────────┘  └────────────────────────────┘ │
│  Available: R 850,000                                           │
│                                                                  │
│  TRANSFER TO                                                     │
│  ┌────────────────────────────┐  ┌────────────────────────────┐ │
│  │ Cost Centre         ▼     │  │ Category              ▼   │ │
│  │ IT Division               │  │ IT & Technology            │ │
│  └────────────────────────────┘  └────────────────────────────┘ │
│  Current Allocation: R 15,000,000                               │
│                                                                  │
│  ┌────────────────────────────┐                                 │
│  │ Amount              R     │                                 │
│  │ 250,000                   │                                 │
│  └────────────────────────────┘                                 │
│                                                                  │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │ Justification                                              │ │
│  │ Additional server infrastructure required for new ERP...  │ │
│  └────────────────────────────────────────────────────────────┘ │
│                                                                  │
│                          [Cancel]  [Submit for Approval]        │
└─────────────────────────────────────────────────────────────────┘
```

---

## 6. Data Requirements

### 6.1 Budget Entity

| Field | Type | Description |
|-------|------|-------------|
| budget_id | VARCHAR(20) | Primary key |
| financial_year | VARCHAR(9) | e.g., "2025/2026" |
| cost_centre_id | VARCHAR(20) | Foreign key |
| category_id | VARCHAR(20) | Budget category |
| allocated_amount | DECIMAL(15,2) | Original allocation |
| committed_amount | DECIMAL(15,2) | Reserved funds |
| expended_amount | DECIMAL(15,2) | Actual spend |
| transferred_in | DECIMAL(15,2) | Transfers received |
| transferred_out | DECIMAL(15,2) | Transfers made |
| status | ENUM | Active/Frozen/Closed |

### 6.2 Commitment Entity

| Field | Type | Description |
|-------|------|-------------|
| commitment_id | VARCHAR(20) | Primary key |
| budget_id | VARCHAR(20) | Foreign key |
| source_type | ENUM | Requisition/PO/Contract |
| source_id | VARCHAR(20) | Reference ID |
| amount | DECIMAL(15,2) | Committed amount |
| status | ENUM | Active/Released/Converted |
| created_date | DATETIME | Creation timestamp |
| released_date | DATETIME | Release timestamp |

### 6.3 Budget Transfer Entity

| Field | Type | Description |
|-------|------|-------------|
| transfer_id | VARCHAR(20) | Primary key |
| from_budget_id | VARCHAR(20) | Source budget |
| to_budget_id | VARCHAR(20) | Destination budget |
| amount | DECIMAL(15,2) | Transfer amount |
| justification | TEXT | Reason for transfer |
| status | ENUM | Pending/Approved/Rejected |
| requested_by | VARCHAR(20) | Requester |
| approved_by | VARCHAR(20) | Approver |
| request_date | DATETIME | Request timestamp |
| approval_date | DATETIME | Approval timestamp |

---

## 7. Use Cases

### UC-01: Check Budget Availability

**Actor:** System (triggered by requisition)

**Main Flow:**
1. User creates/submits requisition
2. System identifies cost centre and category
3. System retrieves budget record
4. System calculates available balance
5. If sufficient, system allows requisition
6. If insufficient, system blocks with warning
7. System logs budget check

### UC-02: Process Budget Transfer

**Actor:** Cost Centre Manager

**Main Flow:**
1. Manager navigates to Budget Transfers
2. Manager selects source cost centre/category
3. System displays available balance
4. Manager selects destination cost centre/category
5. Manager enters transfer amount
6. Manager provides justification
7. Manager submits transfer request
8. System routes to appropriate approver
9. Approver reviews and approves/rejects
10. System updates budget allocations
11. System notifies requester of outcome

---

## 8. Business Rules

### BR-01: Budget Enforcement
- No expenditure without available budget
- System must block over-budget transactions
- Override requires CFO approval

### BR-02: Commitment Rules
- Commitments reduce available balance
- Commitments auto-release if source cancelled
- Commitments convert to expenditure on payment

### BR-03: Transfer Restrictions
- Cannot transfer from committed funds
- Cannot transfer between financial years
- Maximum single transfer: R 5,000,000

### BR-04: Year-End Processing
- Uncommitted funds lapse at year-end
- Committed funds may carry forward
- Requires Finance Manager approval

### BR-05: Variance Thresholds
- > 10% variance requires explanation
- > 20% variance requires corrective action plan
- Budget exceeded requires immediate escalation

---

**End of Document**
