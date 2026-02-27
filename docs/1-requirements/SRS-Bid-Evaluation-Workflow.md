# Software Requirements Specification (SRS)
# Bid Evaluation Workflow Module

**Document Version:** 1.0
**Date:** 27 February 2026
**Project:** GovProcure eProcurement Platform
**Module:** Bid Evaluation & Award Management

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
9. [Appendices](#9-appendices)

---

## 1. Introduction

### 1.1 Purpose

This Software Requirements Specification (SRS) document describes the functional and non-functional requirements for the Bid Evaluation Workflow module of the GovProcure eProcurement Platform. This module enables procurement officers to evaluate, score, compare, and award tenders in compliance with South African public procurement regulations.

### 1.2 Scope

The Bid Evaluation Workflow module encompasses:

- Bid submission tracking and management
- Multi-criteria bid scoring and evaluation
- B-BBEE compliance verification and point allocation
- Comparative bid analysis
- Award recommendation and approval workflow
- Award letter generation and bidder notification
- Audit trail and reporting

### 1.3 Definitions, Acronyms, and Abbreviations

| Term | Definition |
|------|------------|
| B-BBEE | Broad-Based Black Economic Empowerment |
| CSD | Central Supplier Database |
| RFP | Request for Proposal |
| RFQ | Request for Quotation |
| SCM | Supply Chain Management |
| PPPFA | Preferential Procurement Policy Framework Act |
| CFO | Chief Financial Officer |
| CPO | Chief Procurement Officer |

### 1.4 References

- PPPFA Act No. 5 of 2000
- B-BBEE Act No. 53 of 2003
- Treasury Regulation 16A
- SCM Policy Guidelines
- National Treasury Instruction Notes

### 1.5 Overview

This document is organized into sections covering system description, functional requirements, non-functional requirements, interface specifications, data models, use cases, and business rules.

---

## 2. Overall Description

### 2.1 Product Perspective

The Bid Evaluation Workflow is a subsystem of the GovProcure eProcurement Platform. It interfaces with:

- **Tender Management Module**: Receives tender and bid submission data
- **Supplier Management Module**: Retrieves supplier verification status
- **Document Management System**: Stores and retrieves bid documents
- **Workflow Engine**: Manages approval routing
- **Notification Service**: Sends email/SMS notifications
- **Reporting Module**: Generates evaluation reports

### 2.2 Product Functions

```
┌─────────────────────────────────────────────────────────────────┐
│                    BID EVALUATION WORKFLOW                       │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐     │
│  │  View    │ → │  Score   │ → │ Compare  │ → │  Award   │     │
│  │  Bids    │   │  Bids    │   │  Bids    │   │  Tender  │     │
│  └──────────┘   └──────────┘   └──────────┘   └──────────┘     │
│       │              │              │              │            │
│       ▼              ▼              ▼              ▼            │
│  ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐     │
│  │ Document │   │ Scoring  │   │ Ranking  │   │ Approval │     │
│  │ Review   │   │ Matrix   │   │ Report   │   │ Workflow │     │
│  └──────────┘   └──────────┘   └──────────┘   └──────────┘     │
│                                                     │           │
│                                                     ▼           │
│                                              ┌──────────┐       │
│                                              │  Notify  │       │
│                                              │ Bidders  │       │
│                                              └──────────┘       │
└─────────────────────────────────────────────────────────────────┘
```

### 2.3 User Classes and Characteristics

| User Class | Description | Access Level |
|------------|-------------|--------------|
| Procurement Officer | Conducts bid evaluation and scoring | Full evaluation access |
| Evaluation Committee Member | Reviews and approves scores | Review and approve |
| SCM Manager | Oversees evaluation process | Full access + override |
| CFO | Approves high-value awards | Approval only |
| CPO | Final award authority | Full access + award |
| Auditor | Reviews evaluation records | Read-only access |

### 2.4 Operating Environment

- **Client**: Modern web browsers (Chrome 90+, Firefox 88+, Edge 90+, Safari 14+)
- **Server**: Cloud-hosted application server
- **Database**: Relational database with audit logging
- **Integration**: REST APIs for external system integration

### 2.5 Design and Implementation Constraints

- Must comply with PPPFA scoring methodology
- B-BBEE points must be calculated per gazetted regulations
- All actions must be logged for audit purposes
- Documents must be stored in compliance with POPIA
- System must support concurrent multi-user evaluation

### 2.6 Assumptions and Dependencies

- Bids have been submitted through the Tender Management Module
- Supplier CSD verification is available via integration
- Users have appropriate role-based access configured
- Email/SMS notification services are operational

---

## 3. Functional Requirements

### 3.1 Bid List Management

#### FR-3.1.1 View Bid List
| ID | FR-3.1.1 |
|----|----------|
| **Description** | System shall display all bids received for a tender |
| **Input** | Tender ID |
| **Output** | List of bids with summary information |
| **Priority** | High |

**Acceptance Criteria:**
- Display bidder company name
- Display bid amount
- Display submission date/time
- Display B-BBEE level
- Display scoring status (Scored/Not Scored)
- Display total score (if scored)
- Sort by total score (descending) by default
- Highlight missing documents

#### FR-3.1.2 Filter and Search Bids
| ID | FR-3.1.2 |
|----|----------|
| **Description** | System shall allow filtering and searching of bids |
| **Input** | Filter criteria (status, B-BBEE level, amount range) |
| **Output** | Filtered bid list |
| **Priority** | Medium |

**Filter Options:**
- Scoring status: All / Scored / Not Scored
- B-BBEE Level: Level 1-8
- Bid amount range
- Submission date range

### 3.2 Bid Detail View

#### FR-3.2.1 View Bid Details
| ID | FR-3.2.1 |
|----|----------|
| **Description** | System shall display comprehensive bid information |
| **Input** | Bid ID |
| **Output** | Complete bid details |
| **Priority** | High |

**Display Elements:**
- Bidder company information
- Bid reference number
- Bid amount
- Submission timestamp
- B-BBEE certification level
- B-BBEE points allocation
- Document checklist with status
- Scoring breakdown (if scored)
- Score history and modifications

#### FR-3.2.2 Document Verification Checklist
| ID | FR-3.2.2 |
|----|----------|
| **Description** | System shall display document submission status |
| **Input** | Bid ID |
| **Output** | Document checklist with verification status |
| **Priority** | High |

**Required Documents:**
| Document | Verification |
|----------|--------------|
| CSD Registration | Auto-verified via CSD API |
| Tax Clearance Certificate | Manual verification |
| B-BBEE Certificate | Manual verification |
| Technical Proposal | Completeness check |
| Financial Proposal | Completeness check |

### 3.3 Bid Scoring

#### FR-3.3.1 Score Bid
| ID | FR-3.3.1 |
|----|----------|
| **Description** | System shall allow scoring of bids against defined criteria |
| **Input** | Scores for each criterion (0-100) |
| **Output** | Weighted total score |
| **Priority** | High |

**Scoring Criteria:**

| Criterion | Weight | Description |
|-----------|--------|-------------|
| Technical Capability | 40% | Quality of technical proposal, solution fit, innovation |
| Price Competitiveness | 30% | Value for money relative to other bids |
| Experience & Track Record | 15% | Past performance, references, industry expertise |
| Methodology & Approach | 15% | Implementation plan, risk management, governance |
| B-BBEE Points | +0-20 | Additional points based on B-BBEE level |

**B-BBEE Point Allocation (80/20 Preference System):**

| B-BBEE Level | Points |
|--------------|--------|
| Level 1 | 20 |
| Level 2 | 18 |
| Level 3 | 14 |
| Level 4 | 12 |
| Level 5 | 8 |
| Level 6 | 6 |
| Level 7 | 4 |
| Level 8 | 2 |
| Non-compliant | 0 |

#### FR-3.3.2 Score Calculation
| ID | FR-3.3.2 |
|----|----------|
| **Description** | System shall calculate total score using weighted formula |
| **Formula** | `Total = (Technical × 0.4) + (Price × 0.3) + (Experience × 0.15) + (Methodology × 0.15) + B-BBEE_Points` |
| **Priority** | High |

#### FR-3.3.3 Save and Update Scores
| ID | FR-3.3.3 |
|----|----------|
| **Description** | System shall persist scores with audit trail |
| **Input** | Score values, evaluator ID, timestamp |
| **Output** | Confirmation, updated bid record |
| **Priority** | High |

**Audit Fields:**
- Scored by (user ID)
- Scored at (timestamp)
- Previous score (if update)
- Modification reason (if update)

### 3.4 Bid Comparison

#### FR-3.4.1 Generate Comparison Matrix
| ID | FR-3.4.1 |
|----|----------|
| **Description** | System shall generate side-by-side bid comparison |
| **Input** | Tender ID |
| **Output** | Comparison table with rankings |
| **Priority** | High |

**Comparison Table Columns:**
- Rank
- Bidder Name
- Bid Amount
- Technical Score
- Price Score
- Experience Score
- Methodology Score
- B-BBEE Points
- Total Score
- Recommendation indicator

#### FR-3.4.2 Rank Bids
| ID | FR-3.4.2 |
|----|----------|
| **Description** | System shall rank bids by total score |
| **Input** | All scored bids for tender |
| **Output** | Ranked list with recommended winner |
| **Priority** | High |

**Ranking Rules:**
1. Sort by total score (descending)
2. In case of tie, higher B-BBEE level wins
3. If still tied, lower bid amount wins
4. Highlight recommended winner (rank 1)

#### FR-3.4.3 Export Comparison Report
| ID | FR-3.4.3 |
|----|----------|
| **Description** | System shall export comparison as PDF/Excel |
| **Input** | Tender ID, export format |
| **Output** | Downloadable report file |
| **Priority** | Medium |

### 3.5 Award Process

#### FR-3.5.1 Initiate Award
| ID | FR-3.5.1 |
|----|----------|
| **Description** | System shall initiate award process for recommended bidder |
| **Input** | Tender ID, selected bid ID |
| **Output** | Award initiation confirmation |
| **Priority** | High |

**Pre-conditions:**
- All bids must be scored
- Selected bid must have highest score
- All mandatory documents verified

#### FR-3.5.2 Capture Award Justification
| ID | FR-3.5.2 |
|----|----------|
| **Description** | System shall require justification for award |
| **Input** | Justification text, contract details |
| **Output** | Stored justification |
| **Priority** | High |

**Required Fields:**
- Award justification (text, min 50 characters)
- Contract start date
- Contract duration
- Special conditions (optional)

#### FR-3.5.3 Approval Workflow
| ID | FR-3.5.3 |
|----|----------|
| **Description** | System shall route award for approval based on value |
| **Input** | Award details, tender value |
| **Output** | Approval request notification |
| **Priority** | High |

**Approval Thresholds:**

| Contract Value | Approval Required |
|----------------|-------------------|
| < R 500,000 | SCM Manager |
| R 500,000 - R 2,000,000 | SCM Manager + CFO |
| > R 2,000,000 | SCM Manager + CFO + CPO |

#### FR-3.5.4 CFO Approval Request
| ID | FR-3.5.4 |
|----|----------|
| **Description** | System shall send approval request to CFO |
| **Input** | Award details |
| **Output** | Approval request with reference number |
| **Priority** | High |

**Request Contains:**
- Request reference number
- Tender details
- Recommended bidder
- Contract value
- Evaluation summary
- Justification

#### FR-3.5.5 Confirm Award
| ID | FR-3.5.5 |
|----|----------|
| **Description** | System shall finalize award upon approval |
| **Input** | Approval confirmation |
| **Output** | Award reference number, status update |
| **Priority** | High |

**Post-Award Actions:**
- Generate award reference number
- Update tender status to "Awarded"
- Update bid status to "Successful"
- Update other bids to "Unsuccessful"
- Trigger notification workflow

### 3.6 Award Documentation

#### FR-3.6.1 Generate Award Letter
| ID | FR-3.6.1 |
|----|----------|
| **Description** | System shall generate formal award letter |
| **Input** | Award details, template |
| **Output** | PDF award letter |
| **Priority** | High |

**Letter Contents:**
- Department letterhead
- Date and reference
- Supplier address
- Award notification text
- Contract details (value, duration, start date)
- Acceptance requirements
- Contact information
- Authorized signature block

#### FR-3.6.2 Generate Regret Letters
| ID | FR-3.6.2 |
|----|----------|
| **Description** | System shall generate regret letters for unsuccessful bidders |
| **Input** | Tender ID, unsuccessful bid IDs |
| **Output** | PDF regret letters |
| **Priority** | Medium |

**Letter Contents:**
- Department letterhead
- Date and reference
- Supplier address
- Regret notification text
- Brief feedback (optional)
- Appeal process information

### 3.7 Bidder Notification

#### FR-3.7.1 Notify Successful Bidder
| ID | FR-3.7.1 |
|----|----------|
| **Description** | System shall notify successful bidder |
| **Input** | Award details, contact information |
| **Output** | Email notification with award letter |
| **Priority** | High |

**Notification Channels:**
- Email (primary)
- SMS (optional)
- System notification

#### FR-3.7.2 Notify Unsuccessful Bidders
| ID | FR-3.7.2 |
|----|----------|
| **Description** | System shall notify all unsuccessful bidders |
| **Input** | Tender ID, regret letters |
| **Output** | Email notifications |
| **Priority** | High |

#### FR-3.7.3 Notification Tracking
| ID | FR-3.7.3 |
|----|----------|
| **Description** | System shall track notification delivery status |
| **Input** | Notification ID |
| **Output** | Delivery status (Sent/Delivered/Failed) |
| **Priority** | Medium |

### 3.8 Reporting and Audit

#### FR-3.8.1 Evaluation Report
| ID | FR-3.8.1 |
|----|----------|
| **Description** | System shall generate comprehensive evaluation report |
| **Input** | Tender ID |
| **Output** | PDF evaluation report |
| **Priority** | High |

**Report Sections:**
1. Executive Summary
2. Tender Overview
3. Bids Received
4. Evaluation Methodology
5. Scoring Results
6. Comparison Matrix
7. Recommendation
8. Approval History
9. Appendices (score sheets)

#### FR-3.8.2 Audit Trail
| ID | FR-3.8.2 |
|----|----------|
| **Description** | System shall maintain complete audit trail |
| **Input** | All user actions |
| **Output** | Immutable audit log |
| **Priority** | High |

**Logged Events:**
- Bid viewed
- Score entered/modified
- Comparison generated
- Award initiated
- Approval requested
- Approval granted/rejected
- Letters generated
- Notifications sent

---

## 4. Non-Functional Requirements

### 4.1 Performance Requirements

| ID | Requirement | Target |
|----|-------------|--------|
| NFR-4.1.1 | Bid list load time | < 2 seconds |
| NFR-4.1.2 | Bid detail load time | < 1 second |
| NFR-4.1.3 | Score save response | < 500ms |
| NFR-4.1.4 | Comparison generation | < 3 seconds |
| NFR-4.1.5 | Report generation | < 10 seconds |
| NFR-4.1.6 | Concurrent users | 100+ simultaneous |

### 4.2 Security Requirements

| ID | Requirement |
|----|-------------|
| NFR-4.2.1 | Role-based access control for all functions |
| NFR-4.2.2 | Encryption of sensitive data at rest and in transit |
| NFR-4.2.3 | Session timeout after 30 minutes of inactivity |
| NFR-4.2.4 | Multi-factor authentication for approvals |
| NFR-4.2.5 | IP-based access restrictions for admin functions |
| NFR-4.2.6 | POPIA compliance for personal data handling |

### 4.3 Reliability Requirements

| ID | Requirement | Target |
|----|-------------|--------|
| NFR-4.3.1 | System availability | 99.5% uptime |
| NFR-4.3.2 | Data backup frequency | Every 4 hours |
| NFR-4.3.3 | Recovery Point Objective (RPO) | 4 hours |
| NFR-4.3.4 | Recovery Time Objective (RTO) | 2 hours |
| NFR-4.3.5 | Audit log retention | 7 years |

### 4.4 Usability Requirements

| ID | Requirement |
|----|-------------|
| NFR-4.4.1 | Responsive design for desktop and tablet |
| NFR-4.4.2 | WCAG 2.1 AA accessibility compliance |
| NFR-4.4.3 | Consistent navigation and UI patterns |
| NFR-4.4.4 | Contextual help and tooltips |
| NFR-4.4.5 | Error messages with resolution guidance |
| NFR-4.4.6 | Confirmation dialogs for destructive actions |

### 4.5 Scalability Requirements

| ID | Requirement |
|----|-------------|
| NFR-4.5.1 | Support 500+ tenders per year |
| NFR-4.5.2 | Support 50+ bids per tender |
| NFR-4.5.3 | Support 1000+ registered suppliers |
| NFR-4.5.4 | Horizontal scaling for peak periods |

---

## 5. User Interface Requirements

### 5.1 Bid List View

```
┌─────────────────────────────────────────────────────────────────┐
│ Evaluate Bids: TND-2026-007                              [X]    │
├─────────────────────────────────────────────────────────────────┤
│ Enterprise Resource Planning (ERP) System Implementation        │
│ Estimated Value: R 2,500,000 · 6 bids received                  │
│                                                                  │
│ [Compare All Bids] [Export Report]          [Proceed to Award →]│
├─────────────────────────────────────────────────────────────────┤
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ [SA] SAP Africa (Pty) Ltd                    92  Level 1   │ │
│ │      R 2,340,000 · 25 Feb 2026              SCORE          │ │
│ └─────────────────────────────────────────────────────────────┘ │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ [DT] DataTech Solutions                      88  Level 2   │ │
│ │      R 2,150,000 · 24 Feb 2026              SCORE          │ │
│ └─────────────────────────────────────────────────────────────┘ │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ [PS] ProcureWise Systems                     72  Level 4   │ │
│ │      R 2,680,000 · 23 Feb 2026              SCORE          │ │
│ └─────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

### 5.2 Bid Detail View

```
┌─────────────────────────────────────────────────────────────────┐
│ Bid Details: SAP Africa (Pty) Ltd                        [X]    │
├─────────────────────────────────────────────────────────────────┤
│ ┌────┐                                              ┌────────┐  │
│ │ SA │  SAP Africa (Pty) Ltd                        │   92   │  │
│ └────┘  Bid ID: BID-001 · Tender: TND-2026-007      │ TOTAL  │  │
│         [B-BBEE Level 1]                            └────────┘  │
├─────────────────────────────────────────────────────────────────┤
│ ┌─────────────────────┐  ┌─────────────────────┐                │
│ │ Bid Amount          │  │ Submitted           │                │
│ │ R 2,340,000         │  │ 25 Feb 2026         │                │
│ └─────────────────────┘  └─────────────────────┘                │
├─────────────────────────────────────────────────────────────────┤
│ SCORING BREAKDOWN (80 Technical + 20 B-BBEE)                    │
│                                                                  │
│ Technical Capability (40%)              92/100  ████████████░░  │
│ Price Competitiveness (30%)             85/100  ██████████░░░░  │
│ Experience & Track Record (15%)         95/100  █████████████░  │
│ Methodology & Approach (15%)            88/100  ███████████░░░  │
│ ─────────────────────────────────────────────────               │
│ B-BBEE Points (Level 1)                         +20             │
├─────────────────────────────────────────────────────────────────┤
│ DOCUMENT CHECKLIST                                               │
│ ✓ CSD Registration    ✓ Tax Clearance    ✓ B-BBEE Certificate  │
│ ✓ Technical Proposal  ✓ Financial Proposal                      │
├─────────────────────────────────────────────────────────────────┤
│ [← Back to List]  [Download Documents]  [Score This Bid]        │
└─────────────────────────────────────────────────────────────────┘
```

### 5.3 Scoring Form

```
┌─────────────────────────────────────────────────────────────────┐
│ Score Bid: SAP Africa (Pty) Ltd                          [X]    │
├─────────────────────────────────────────────────────────────────┤
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ ℹ Scoring Criteria: Technical (40%) + Price (30%) +        │ │
│ │   Experience (15%) + Methodology (15%) + B-BBEE Points     │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                  │
│ Technical Capability (0-100)                    Weight: 40%     │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ [92                                                       ] │ │
│ └─────────────────────────────────────────────────────────────┘ │
│ Assess technical proposal quality, solution fit, and innovation │
│                                                                  │
│ Price Competitiveness (0-100)                   Weight: 30%     │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ [85                                                       ] │ │
│ └─────────────────────────────────────────────────────────────┘ │
│ Score relative to other bids and budget                         │
│                                                                  │
│ Experience & Track Record (0-100)               Weight: 15%     │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ [95                                                       ] │ │
│ └─────────────────────────────────────────────────────────────┘ │
│ Past project delivery, client references, industry expertise    │
│                                                                  │
│ Methodology & Approach (0-100)                  Weight: 15%     │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ [88                                                       ] │ │
│ └─────────────────────────────────────────────────────────────┘ │
│ Implementation plan, risk management, project governance        │
│                                                                  │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ B-BBEE Points (Auto-calculated)                       +20  │ │
│ │ Level 1 contributor                                         │ │
│ └─────────────────────────────────────────────────────────────┘ │
├─────────────────────────────────────────────────────────────────┤
│                                      [Cancel]  [Save Score]     │
└─────────────────────────────────────────────────────────────────┘
```

### 5.4 Comparison Matrix

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ Bid Comparison: TND-2026-007                                          [X]   │
├─────────────────────────────────────────────────────────────────────────────┤
│ Enterprise Resource Planning (ERP) System Implementation                     │
│ Comparing 6 bids · Ranked by total score                                    │
├──────┬─────────────────────┬─────────────┬──────┬──────┬─────┬──────┬──────┤
│ Rank │ Bidder              │ Amount      │ Tech │ Price│ Exp │ Meth │Total │
├──────┼─────────────────────┼─────────────┼──────┼──────┼─────┼──────┼──────┤
│  1   │ SAP Africa ✓        │ R 2,340,000 │  92  │  85  │  95 │  88  │  92  │
│  2   │ GovTech Solutions   │ R 2,280,000 │  88  │  88  │  85 │  90  │  91  │
│  3   │ DataTech Solutions  │ R 2,150,000 │  85  │  92  │  80 │  82  │  88  │
│  4   │ Enterprise Corp     │ R 2,450,000 │  82  │  80  │  78 │  85  │  82  │
│  5   │ ProcureWise Systems │ R 2,680,000 │  78  │  70  │  75 │  80  │  72  │
│  6   │ Digital Systems ZA  │ R 2,590,000 │  75  │  75  │  70 │  72  │  71  │
└──────┴─────────────────────┴─────────────┴──────┴──────┴─────┴──────┴──────┘
│                                                                              │
│ ┌──────────────────────────────────────────────────────────────────────────┐│
│ │ Recommendation: Based on the evaluation criteria, SAP Africa (Pty) Ltd  ││
│ │ achieved the highest score of 92 points and is recommended for award.   ││
│ └──────────────────────────────────────────────────────────────────────────┘│
├──────────────────────────────────────────────────────────────────────────────┤
│ [← Back]                    [Export Comparison]        [Proceed to Award]   │
└──────────────────────────────────────────────────────────────────────────────┘
```

---

## 6. Data Requirements

### 6.1 Data Model

#### 6.1.1 Tender Entity

| Field | Type | Description |
|-------|------|-------------|
| tender_id | VARCHAR(20) | Primary key (e.g., TND-2026-007) |
| title | VARCHAR(255) | Tender title |
| description | TEXT | Full description |
| estimated_value | DECIMAL(15,2) | Estimated contract value |
| category | VARCHAR(100) | Procurement category |
| department | VARCHAR(100) | Issuing department |
| status | ENUM | Open, Evaluation, Awarded, Cancelled |
| closing_date | DATE | Bid submission deadline |
| created_at | TIMESTAMP | Record creation time |
| updated_at | TIMESTAMP | Last update time |

#### 6.1.2 Bid Entity

| Field | Type | Description |
|-------|------|-------------|
| bid_id | VARCHAR(20) | Primary key (e.g., BID-001) |
| tender_id | VARCHAR(20) | Foreign key to Tender |
| supplier_id | VARCHAR(20) | Foreign key to Supplier |
| company_name | VARCHAR(255) | Bidder company name |
| bid_amount | DECIMAL(15,2) | Bid value |
| bbbee_level | INT | B-BBEE level (1-8) |
| bbbee_points | INT | Calculated B-BBEE points |
| submitted_at | TIMESTAMP | Submission timestamp |
| status | ENUM | Submitted, Under Evaluation, Successful, Unsuccessful |
| created_at | TIMESTAMP | Record creation time |

#### 6.1.3 Bid Score Entity

| Field | Type | Description |
|-------|------|-------------|
| score_id | INT | Primary key |
| bid_id | VARCHAR(20) | Foreign key to Bid |
| technical_score | INT | Technical capability score (0-100) |
| price_score | INT | Price competitiveness score (0-100) |
| experience_score | INT | Experience score (0-100) |
| methodology_score | INT | Methodology score (0-100) |
| total_score | INT | Calculated total score |
| scored_by | VARCHAR(50) | User ID of evaluator |
| scored_at | TIMESTAMP | Scoring timestamp |
| version | INT | Score version for audit |

#### 6.1.4 Bid Document Entity

| Field | Type | Description |
|-------|------|-------------|
| document_id | INT | Primary key |
| bid_id | VARCHAR(20) | Foreign key to Bid |
| document_type | ENUM | CSD, Tax, BBBEE, Technical, Financial |
| file_name | VARCHAR(255) | Original file name |
| file_path | VARCHAR(500) | Storage path |
| verified | BOOLEAN | Verification status |
| verified_by | VARCHAR(50) | Verifier user ID |
| verified_at | TIMESTAMP | Verification timestamp |

#### 6.1.5 Award Entity

| Field | Type | Description |
|-------|------|-------------|
| award_id | VARCHAR(20) | Primary key (e.g., AWD-2026-001) |
| tender_id | VARCHAR(20) | Foreign key to Tender |
| bid_id | VARCHAR(20) | Foreign key to winning Bid |
| contract_value | DECIMAL(15,2) | Final contract value |
| justification | TEXT | Award justification |
| contract_start | DATE | Contract start date |
| contract_end | DATE | Contract end date |
| status | ENUM | Pending Approval, Approved, Rejected |
| awarded_by | VARCHAR(50) | CPO user ID |
| awarded_at | TIMESTAMP | Award timestamp |

#### 6.1.6 Approval Entity

| Field | Type | Description |
|-------|------|-------------|
| approval_id | VARCHAR(20) | Primary key (e.g., APR-2026-001) |
| award_id | VARCHAR(20) | Foreign key to Award |
| approver_role | ENUM | SCM Manager, CFO, CPO |
| approver_id | VARCHAR(50) | Approver user ID |
| status | ENUM | Pending, Approved, Rejected |
| comments | TEXT | Approval comments |
| decided_at | TIMESTAMP | Decision timestamp |

#### 6.1.7 Audit Log Entity

| Field | Type | Description |
|-------|------|-------------|
| log_id | BIGINT | Primary key |
| entity_type | VARCHAR(50) | Entity being audited |
| entity_id | VARCHAR(50) | Entity identifier |
| action | VARCHAR(50) | Action performed |
| old_value | JSON | Previous state |
| new_value | JSON | New state |
| user_id | VARCHAR(50) | Acting user |
| ip_address | VARCHAR(45) | User IP address |
| timestamp | TIMESTAMP | Action timestamp |

### 6.2 Data Dictionary

| Term | Definition |
|------|------------|
| Bid Amount | The total value quoted by the bidder for the tender |
| B-BBEE Level | The bidder's Broad-Based Black Economic Empowerment certification level |
| Technical Score | Assessment of the bidder's technical capability and solution quality |
| Price Score | Assessment of price competitiveness relative to other bids |
| Total Score | Weighted sum of all scoring criteria plus B-BBEE points |
| Award Reference | Unique identifier assigned when tender is awarded |

---

## 7. Use Cases

### UC-01: Evaluate Tender Bids

**Actor:** Procurement Officer

**Preconditions:**
- User is logged in with Procurement Officer role
- Tender exists with status "Under Evaluation"
- At least one bid has been submitted

**Main Flow:**
1. User navigates to Tenders section
2. User clicks "Evaluate Bids" on tender card
3. System displays list of all bids with summary information
4. User clicks on a bid to view details
5. System displays bid details including documents and scores
6. User clicks "Score This Bid"
7. System displays scoring form with criteria
8. User enters scores for each criterion
9. User clicks "Save Score"
10. System calculates total score and saves
11. System displays updated bid details with scores
12. User repeats steps 4-11 for remaining bids

**Postconditions:**
- All bids are scored
- Scores are persisted with audit trail

**Alternative Flows:**
- 4a. Bid has missing documents: System displays warning indicator
- 9a. Score values invalid: System displays validation error

---

### UC-02: Compare Bids

**Actor:** Procurement Officer

**Preconditions:**
- At least 2 bids have been scored
- User has evaluation access

**Main Flow:**
1. User clicks "Compare All Bids" from bid list
2. System generates comparison matrix
3. System ranks bids by total score
4. System highlights recommended winner
5. User reviews comparison
6. User clicks "Export Comparison" to download report
7. System generates PDF and initiates download

**Postconditions:**
- Comparison report generated and downloaded

---

### UC-03: Award Tender

**Actor:** Procurement Officer, CFO, CPO

**Preconditions:**
- All bids are scored
- Comparison has been reviewed
- User has award initiation access

**Main Flow:**
1. User clicks "Proceed to Award" from comparison
2. System displays award form with recommended bidder
3. User enters award justification
4. User selects contract start date
5. User clicks "Request CFO Approval" (if required)
6. System sends approval request to CFO
7. CFO receives notification and reviews
8. CFO approves the award
9. User receives approval notification
10. User clicks "Confirm Award"
11. System generates award reference number
12. System updates tender and bid statuses
13. System displays award confirmation

**Postconditions:**
- Tender status updated to "Awarded"
- Winning bid status updated to "Successful"
- Other bids updated to "Unsuccessful"
- Award record created with reference number

**Alternative Flows:**
- 5a. Contract value below threshold: Skip CFO approval
- 8a. CFO rejects: User notified, award not processed
- 8b. CFO requests changes: User modifies and resubmits

---

### UC-04: Notify Bidders

**Actor:** Procurement Officer

**Preconditions:**
- Tender has been awarded
- Award is confirmed

**Main Flow:**
1. User clicks "Notify All Bidders" from award confirmation
2. System generates award letter for successful bidder
3. System generates regret letters for unsuccessful bidders
4. System sends notifications via email
5. System displays notification status for each bidder
6. System logs all notifications sent

**Postconditions:**
- All bidders notified
- Notification delivery tracked
- Audit log updated

---

### UC-05: Generate Award Letter

**Actor:** Procurement Officer

**Preconditions:**
- Tender has been awarded

**Main Flow:**
1. User clicks "Generate Award Letter"
2. System populates letter template with award details
3. System displays letter preview
4. User reviews letter content
5. User clicks "Download PDF"
6. System generates PDF document
7. System initiates file download

**Alternative Flow:**
- 5a. User clicks "Send to Supplier": System emails letter directly

**Postconditions:**
- Award letter generated and available

---

## 8. Business Rules

### BR-01: Scoring Weights
All bids must be scored using the standard weighting:
- Technical Capability: 40%
- Price Competitiveness: 30%
- Experience & Track Record: 15%
- Methodology & Approach: 15%

### BR-02: B-BBEE Points
B-BBEE points are added to the weighted score (80/20 preference system):
- Level 1: +20 points
- Level 2: +18 points
- Level 3: +14 points
- Level 4: +12 points
- Level 5: +8 points
- Level 6: +6 points
- Level 7: +4 points
- Level 8: +2 points

### BR-03: Minimum Score Threshold
Bids must achieve a minimum technical score of 60/100 to be considered for award.

### BR-04: Document Compliance
All mandatory documents must be verified before a bid can be awarded:
- CSD Registration (mandatory)
- Tax Clearance Certificate (mandatory)
- B-BBEE Certificate (mandatory for points)
- Technical Proposal (mandatory)
- Financial Proposal (mandatory)

### BR-05: Approval Thresholds
Awards require approval based on contract value:
- < R 500,000: SCM Manager approval
- R 500,000 - R 2,000,000: SCM Manager + CFO approval
- > R 2,000,000: SCM Manager + CFO + CPO approval

### BR-06: Tie-Breaking
In case of equal total scores:
1. Higher B-BBEE level wins
2. If still tied, lower bid amount wins
3. If still tied, earlier submission wins

### BR-07: Score Modification
Score modifications require:
- Written justification
- Supervisor approval (for changes > 10 points)
- Full audit trail retention

### BR-08: Notification Timeline
- Successful bidder: Notified within 24 hours of award
- Unsuccessful bidders: Notified within 48 hours of award

### BR-09: Audit Retention
All evaluation records must be retained for minimum 7 years per PFMA requirements.

### BR-10: Conflict of Interest
Evaluators must declare any conflicts of interest before participating in evaluation. Conflicted evaluators are excluded from scoring related bids.

---

## 9. Appendices

### Appendix A: Score Calculation Example

**Bidder:** SAP Africa (Pty) Ltd
**B-BBEE Level:** 1

| Criterion | Score | Weight | Weighted |
|-----------|-------|--------|----------|
| Technical | 92 | 40% | 36.8 |
| Price | 85 | 30% | 25.5 |
| Experience | 95 | 15% | 14.25 |
| Methodology | 88 | 15% | 13.2 |
| **Subtotal** | | | **89.75** |
| B-BBEE Points | Level 1 | | +20 |
| **Total Score** | | | **109.75 → 92** |

*Note: Total capped at 100 for display, but full score used for ranking*

### Appendix B: Status Transitions

```
Tender Status Flow:
Open → Under Evaluation → Awarded
                      ↘ Cancelled

Bid Status Flow:
Submitted → Under Evaluation → Successful
                            ↘ Unsuccessful

Award Status Flow:
Pending Approval → Approved → Completed
                ↘ Rejected
```

### Appendix C: Notification Templates

**Award Notification (Email Subject):**
```
TENDER AWARD NOTIFICATION: [Tender Reference] - [Tender Title]
```

**Regret Notification (Email Subject):**
```
TENDER OUTCOME NOTIFICATION: [Tender Reference] - [Tender Title]
```

---

## Document History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | 27 Feb 2026 | System | Initial version |

---

**End of Document**
