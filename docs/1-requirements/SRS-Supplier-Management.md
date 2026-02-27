# Software Requirements Specification (SRS)
# Supplier Management Module

**Document Version:** 1.0
**Date:** 27 February 2026
**Project:** GovProcure eProcurement Platform
**Module:** Supplier Management & Vetting

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

This SRS document describes the requirements for the Supplier Management module of the GovProcure eProcurement Platform. This module enables the organization to register, vet, manage, and monitor suppliers throughout their lifecycle.

### 1.2 Scope

The Supplier Management module encompasses:

- Supplier registration and onboarding
- CSD verification integration
- B-BBEE certificate management
- Supplier performance tracking
- Supplier status management (Active, Preferred, Suspended, Blacklisted)
- Supplier risk assessment
- Supplier communication and notifications

### 1.3 Definitions

| Term | Definition |
|------|------------|
| CSD | Central Supplier Database (National Treasury) |
| B-BBEE | Broad-Based Black Economic Empowerment |
| Preferred Supplier | Supplier with proven track record and preferred status |
| Vetting | Process of verifying supplier credentials and compliance |

---

## 2. Overall Description

### 2.1 Product Perspective

```
┌─────────────────────────────────────────────────────────────────┐
│                  SUPPLIER LIFECYCLE MANAGEMENT                   │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐     │
│  │ Register │ → │   Vet    │ → │  Active  │ → │ Preferred│     │
│  │ Supplier │   │ Supplier │   │ Supplier │   │ Supplier │     │
│  └──────────┘   └──────────┘   └──────────┘   └──────────┘     │
│                      │                             │            │
│                      ▼                             ▼            │
│                 ┌──────────┐                 ┌──────────┐       │
│                 │ Rejected │                 │Suspended │       │
│                 └──────────┘                 └──────────┘       │
│                                                    │            │
│                                                    ▼            │
│                                              ┌──────────┐       │
│                                              │Blacklist │       │
│                                              └──────────┘       │
└─────────────────────────────────────────────────────────────────┘
```

### 2.2 User Classes

| User Class | Description | Access Level |
|------------|-------------|--------------|
| Supplier | External party registering on system | Self-service registration |
| Procurement Officer | Manages supplier records | Full CRUD access |
| SCM Manager | Approves supplier status changes | Approve/Reject |
| Auditor | Reviews supplier compliance | Read-only access |

---

## 3. Functional Requirements

### 3.1 Supplier Registration

#### FR-3.1.1 Self-Service Registration
| ID | FR-3.1.1 |
|----|----------|
| **Description** | System shall allow suppliers to register online |
| **Priority** | High |

**Registration Fields:**

| Field | Type | Required | Validation |
|-------|------|----------|------------|
| Company Name | Text | Yes | Min 3 chars |
| Trading Name | Text | No | - |
| Registration Number | Text | Yes | Format: YYYY/NNNNNN/NN |
| VAT Number | Text | Yes | 10 digits |
| CSD Number | Text | Yes | Format: NNNN-NNNN |
| B-BBEE Level | Select | Yes | Level 1-8 or Non-compliant |
| B-BBEE Certificate | File | Yes | PDF, max 5MB |
| Tax Clearance | File | Yes | PDF, max 5MB |
| Company Address | Text | Yes | - |
| Contact Person | Text | Yes | - |
| Email | Email | Yes | Valid email format |
| Phone | Text | Yes | Valid phone format |
| Bank Name | Text | Yes | From approved list |
| Account Number | Text | Yes | - |
| Branch Code | Text | Yes | 6 digits |
| Categories | Multi-select | Yes | At least one |

#### FR-3.1.2 CSD Verification
| ID | FR-3.1.2 |
|----|----------|
| **Description** | System shall verify supplier against CSD |
| **Priority** | High |

**Verification Process:**
1. Query CSD API with supplier registration number
2. Validate company name matches
3. Validate tax compliance status
4. Retrieve B-BBEE information
5. Store verification timestamp

#### FR-3.1.3 Document Upload
| ID | FR-3.1.3 |
|----|----------|
| **Description** | System shall accept document uploads |
| **Priority** | High |

**Required Documents:**
- B-BBEE Certificate (valid, not expired)
- Tax Clearance Certificate (valid)
- Company Registration (CIPC)
- Bank Confirmation Letter
- Directors' ID Documents

### 3.2 Supplier Vetting

#### FR-3.2.1 Vetting Checklist
| ID | FR-3.2.1 |
|----|----------|
| **Description** | System shall provide vetting checklist |
| **Priority** | High |

**Checklist Items:**
| Item | Verification Method |
|------|---------------------|
| CSD Registration | Automated API check |
| Tax Compliance | Automated SARS check |
| B-BBEE Certificate | Manual review |
| Company Registration | Manual CIPC check |
| Bank Details | Manual confirmation |
| Directors' Background | Manual check |
| No Conflict of Interest | Declaration form |

#### FR-3.2.2 Vetting Status
| ID | FR-3.2.2 |
|----|----------|
| **Description** | System shall track vetting status |
| **Priority** | High |

**Statuses:**
- Pending Vetting
- Vetting In Progress
- Vetting Complete - Approved
- Vetting Complete - Rejected
- Vetting Expired (re-vetting required)

### 3.3 Supplier Status Management

#### FR-3.3.1 Status Categories
| ID | FR-3.3.1 |
|----|----------|
| **Description** | System shall manage supplier status |
| **Priority** | High |

| Status | Description | Can Transact |
|--------|-------------|--------------|
| Pending Vetting | Awaiting verification | No |
| Active | Verified, can transact | Yes |
| Preferred | High-performing supplier | Yes (priority) |
| Suspended | Temporary restriction | No |
| Blacklisted | Permanent restriction | No |
| Inactive | No activity > 24 months | No |

#### FR-3.3.2 Status Transitions
| ID | FR-3.3.2 |
|----|----------|
| **Description** | System shall enforce status transition rules |
| **Priority** | High |

**Allowed Transitions:**
```
Pending → Active (on approval)
Pending → Rejected
Active → Preferred (on qualification)
Active → Suspended (on violation)
Active → Inactive (auto, after 24 months)
Preferred → Active (on demotion)
Preferred → Suspended (on violation)
Suspended → Active (on reinstatement)
Suspended → Blacklisted (on severe violation)
Inactive → Active (on reactivation)
```

#### FR-3.3.3 Status Change Approval
| ID | FR-3.3.3 |
|----|----------|
| **Description** | System shall require approval for status changes |
| **Priority** | High |

| Transition | Approver Required |
|------------|-------------------|
| → Preferred | SCM Manager |
| → Suspended | SCM Manager |
| → Blacklisted | SCM Manager + CPO |
| → Reinstated | SCM Manager |

### 3.4 Supplier Performance

#### FR-3.4.1 Performance Metrics
| ID | FR-3.4.1 |
|----|----------|
| **Description** | System shall track supplier performance |
| **Priority** | High |

**Metrics:**
| Metric | Calculation |
|--------|-------------|
| On-Time Delivery % | (On-time deliveries / Total deliveries) × 100 |
| Quality Rating | Average of quality scores (1-5) |
| Order Fulfillment % | (Complete orders / Total orders) × 100 |
| Response Time | Average days to respond to RFQs |
| Total Spend | Sum of all PO values |
| Order Count | Number of completed orders |

#### FR-3.4.2 Performance Dashboard
| ID | FR-3.4.2 |
|----|----------|
| **Description** | System shall display supplier performance |
| **Priority** | Medium |

**Dashboard Elements:**
- Performance score (composite)
- Trend indicators (up/down)
- Order history chart
- Quality trend chart
- Comparison to category average

#### FR-3.4.3 Performance Alerts
| ID | FR-3.4.3 |
|----|----------|
| **Description** | System shall generate performance alerts |
| **Priority** | Medium |

**Alert Triggers:**
- On-time delivery < 80%
- Quality rating < 3.0
- Multiple complaints received
- Contract violation reported

### 3.5 Supplier Search and Filter

#### FR-3.5.1 Search Suppliers
| ID | FR-3.5.1 |
|----|----------|
| **Description** | System shall provide supplier search |
| **Priority** | High |

**Search Fields:**
- Company name
- CSD number
- Category
- B-BBEE level

#### FR-3.5.2 Filter Suppliers
| ID | FR-3.5.2 |
|----|----------|
| **Description** | System shall provide supplier filters |
| **Priority** | High |

**Filter Options:**
- Status (All, Active, Preferred, Pending, Suspended, Blacklisted)
- B-BBEE Level
- Category
- Performance rating
- Spend range

### 3.6 Supplier Communication

#### FR-3.6.1 Send Notifications
| ID | FR-3.6.1 |
|----|----------|
| **Description** | System shall send supplier notifications |
| **Priority** | Medium |

**Notification Types:**
- Registration confirmation
- Vetting status update
- Document expiry reminder
- Tender invitation
- PO notification
- Status change notification

---

## 4. Non-Functional Requirements

### 4.1 Performance

| ID | Requirement | Target |
|----|-------------|--------|
| NFR-4.1.1 | Supplier list load | < 2 seconds |
| NFR-4.1.2 | CSD verification | < 5 seconds |
| NFR-4.1.3 | Search response | < 1 second |
| NFR-4.1.4 | Document upload | < 30 seconds (5MB) |

### 4.2 Security

| ID | Requirement |
|----|-------------|
| NFR-4.2.1 | Encrypt bank details at rest |
| NFR-4.2.2 | Mask account numbers in display |
| NFR-4.2.3 | Audit all status changes |
| NFR-4.2.4 | POPIA compliance for personal data |

---

## 5. User Interface Requirements

### 5.1 Supplier Card Grid

```
┌─────────────────────────────────────────────────────────────────┐
│ Supplier Management                                              │
│ [Search suppliers...        ] [Filter] [Export] [+ Add Supplier]│
├─────────────────────────────────────────────────────────────────┤
│ [All Suppliers (148)] [Preferred (32)] [Pending (11)] [Suspended]│
├─────────────────────────────────────────────────────────────────┤
│ ┌──────────────────┐ ┌──────────────────┐ ┌──────────────────┐  │
│ │ [DC]             │ │ [TS]             │ │ [BR]             │  │
│ │ Datacom Solutions│ │ TechServ Group   │ │ BuildRight Ltd   │  │
│ │ IT Hardware      │ │ Prof. Services   │ │ Facilities       │  │
│ │ CSD: 0041-2887   │ │ CSD: 0083-1144   │ │ CSD: 0022-5531   │  │
│ │ ────────────────│ │ ────────────────│ │ ────────────────│  │
│ │ [Preferred]      │ │ [Preferred]      │ │ [Active]         │  │
│ │ B-BBEE Lvl 2     │ │ B-BBEE Lvl 1     │ │ B-BBEE Lvl 4     │  │
│ │ ────────────────│ │ ────────────────│ │ ────────────────│  │
│ │ R890k │ 98% │ 34 │ │ R1.2M │ 91% │ 22 │ │ R340k │ 85% │ 11 │  │
│ │ Spend │ OTD │ Ord│ │ Spend │ OTD │ Ord│ │ Spend │ OTD │ Ord│  │
│ │ ────────────────│ │ ────────────────│ │ ────────────────│  │
│ │ ★★★★★ 5.0       │ │ ★★★★☆ 4.2       │ │ ★★★★☆ 3.9       │  │
│ │ [View Profile]   │ │ [View Profile]   │ │ [View Profile]   │  │
│ └──────────────────┘ └──────────────────┘ └──────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

### 5.2 Supplier Profile

```
┌─────────────────────────────────────────────────────────────────┐
│ Supplier Profile: Datacom Solutions (Pty) Ltd            [Edit] │
├─────────────────────────────────────────────────────────────────┤
│ ┌────┐                                                          │
│ │ DC │  Datacom Solutions (Pty) Ltd     [Preferred Supplier]    │
│ └────┘  IT Hardware & Software                                  │
│         CSD: 0041-2887 · VAT: 4620183751                       │
│         B-BBEE Level 2                                          │
├─────────────────────────────────────────────────────────────────┤
│ PERFORMANCE SUMMARY                                              │
│ ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────────┐│
│ │  R 890,000  │ │     98%     │ │     34      │ │    ★ 5.0    ││
│ │ Total Spend │ │  On-Time    │ │   Orders    │ │   Rating    ││
│ └─────────────┘ └─────────────┘ └─────────────┘ └─────────────┘│
├─────────────────────────────────────────────────────────────────┤
│ COMPANY DETAILS                    │ DOCUMENTS                  │
│ ──────────────────────────────────│ ──────────────────────────│
│ Registration: 2018/043721/07       │ ✓ CSD Registration        │
│ Address: 14 Electron Ave,          │ ✓ Tax Clearance (Valid)   │
│          Technopark, Pretoria      │ ✓ B-BBEE Certificate      │
│ Contact: John Smith                │ ✓ Bank Confirmation       │
│ Email: accounts@datacom.co.za      │                           │
│ Phone: 012 345 6789                │ [Upload Document]         │
├─────────────────────────────────────────────────────────────────┤
│ BANK DETAILS                                                     │
│ Bank: First National Bank · Account: ****7024 · Branch: 251345 │
├─────────────────────────────────────────────────────────────────┤
│ [Order History] [Performance Report] [Suspend Supplier]         │
└─────────────────────────────────────────────────────────────────┘
```

---

## 6. Data Requirements

### 6.1 Data Model

#### 6.1.1 Supplier Entity

| Field | Type | Description |
|-------|------|-------------|
| supplier_id | VARCHAR(20) | Primary key |
| company_name | VARCHAR(255) | Legal company name |
| trading_name | VARCHAR(255) | Trading as name |
| registration_number | VARCHAR(20) | CIPC registration |
| vat_number | VARCHAR(10) | VAT number |
| csd_number | VARCHAR(10) | CSD reference |
| bbbee_level | INT | B-BBEE level (1-8) |
| bbbee_expiry | DATE | Certificate expiry |
| status | ENUM | Pending, Active, Preferred, Suspended, Blacklisted |
| address | TEXT | Physical address |
| contact_person | VARCHAR(100) | Primary contact |
| email | VARCHAR(255) | Contact email |
| phone | VARCHAR(20) | Contact phone |
| bank_name | VARCHAR(100) | Bank name |
| account_number | VARCHAR(20) | Bank account (encrypted) |
| branch_code | VARCHAR(6) | Bank branch code |
| created_at | TIMESTAMP | Registration date |
| verified_at | TIMESTAMP | CSD verification date |
| last_order_date | DATE | Last transaction date |

#### 6.1.2 Supplier Category Entity

| Field | Type | Description |
|-------|------|-------------|
| id | INT | Primary key |
| supplier_id | VARCHAR(20) | Foreign key |
| category_code | VARCHAR(20) | Category reference |
| category_name | VARCHAR(100) | Category name |

#### 6.1.3 Supplier Performance Entity

| Field | Type | Description |
|-------|------|-------------|
| id | INT | Primary key |
| supplier_id | VARCHAR(20) | Foreign key |
| period | DATE | Month/Year |
| total_orders | INT | Orders in period |
| on_time_orders | INT | On-time deliveries |
| quality_score | DECIMAL(3,2) | Average quality rating |
| total_spend | DECIMAL(15,2) | Spend in period |

---

## 7. Use Cases

### UC-01: Register New Supplier

**Actor:** Supplier (External)

**Main Flow:**
1. Supplier navigates to registration portal
2. Supplier completes registration form
3. Supplier uploads required documents
4. System validates CSD number
5. System creates supplier record with "Pending Vetting" status
6. System notifies Procurement Officer
7. Supplier receives confirmation email

---

### UC-02: Vet Supplier

**Actor:** Procurement Officer

**Main Flow:**
1. Officer views pending suppliers list
2. Officer selects supplier to vet
3. System displays vetting checklist
4. Officer verifies each checklist item
5. Officer marks items as verified
6. Officer approves or rejects supplier
7. System updates supplier status
8. System notifies supplier of outcome

---

### UC-03: Suspend Supplier

**Actor:** SCM Manager

**Preconditions:**
- Supplier is currently Active or Preferred
- Valid reason for suspension exists

**Main Flow:**
1. Manager navigates to supplier profile
2. Manager clicks "Suspend Supplier"
3. System displays suspension form
4. Manager selects suspension reason
5. Manager enters suspension period
6. Manager submits suspension
7. System updates supplier status
8. System blocks new transactions
9. System notifies supplier

---

## 8. Business Rules

### BR-01: CSD Verification Required
- All suppliers must have valid CSD registration
- CSD status must be verified before activation
- Re-verification required annually

### BR-02: B-BBEE Certificate Validity
- B-BBEE certificate must be current (not expired)
- System alerts 60 days before expiry
- Supplier downgraded if certificate expires

### BR-03: Preferred Supplier Criteria
To achieve Preferred status:
- Minimum 12 months as Active supplier
- On-time delivery rate ≥ 95%
- Quality rating ≥ 4.0
- No compliance violations
- B-BBEE Level 1-3

### BR-04: Automatic Suspension Triggers
- Tax clearance expired > 30 days
- B-BBEE certificate expired > 60 days
- On-time delivery < 70% (3 consecutive months)
- Quality rating < 2.5

### BR-05: Blacklist Criteria
- Fraud or corruption proven
- Persistent non-compliance
- Court order or debarment
- Requires CPO approval

---

**End of Document**
