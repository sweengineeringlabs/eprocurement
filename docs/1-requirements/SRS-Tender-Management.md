# Software Requirements Specification (SRS)
# Tender Management Module

**Document Version:** 1.0
**Date:** 27 February 2026
**Project:** GovProcure eProcurement Platform
**Module:** Tender Creation, Publishing & Management

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

This SRS document describes the requirements for the Tender Management module of the GovProcure eProcurement Platform. This module handles the complete tender lifecycle from creation to bid receipt, excluding evaluation (covered in SRS-Bid-Evaluation-Workflow.md).

### 1.2 Scope

The Tender Management module encompasses:

- Tender creation and configuration
- Document preparation and attachment
- Tender approval workflow
- Publication and advertisement
- Supplier notification
- Bid submission portal
- Clarification management
- Tender amendments
- Bid opening ceremony

### 1.3 Definitions

| Term | Definition |
|------|------------|
| RFQ | Request for Quotation |
| RFP | Request for Proposal |
| RFI | Request for Information |
| ITT | Invitation to Tender |
| Bid Bond | Financial guarantee from bidder |
| Clarification | Q&A during tender period |

---

## 2. Overall Description

### 2.1 Product Perspective

```
┌─────────────────────────────────────────────────────────────────┐
│                    TENDER LIFECYCLE                              │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐     │
│  │ Create   │ → │ Approve  │ → │ Publish  │ → │ Receive  │     │
│  │ Tender   │   │ Tender   │   │ Tender   │   │  Bids    │     │
│  └──────────┘   └──────────┘   └──────────┘   └──────────┘     │
│       │              │              │              │            │
│       ▼              ▼              ▼              ▼            │
│  ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐     │
│  │  Draft   │   │ Approved │   │   Open   │   │  Closed  │     │
│  │          │   │          │   │          │   │          │     │
│  └──────────┘   └──────────┘   └──────────┘   └──────────┘     │
│                                                                  │
│  Related: Bid Evaluation → Award → Contract (separate modules)  │
└─────────────────────────────────────────────────────────────────┘
```

### 2.2 Tender Types

| Type | Description | Typical Value |
|------|-------------|---------------|
| RFQ | Quick quote for standard items | < R 500,000 |
| RFP | Detailed proposal with methodology | R 500,000 - R 10M |
| Open Tender | Public competitive bid | > R 10M |
| Limited Tender | Invited suppliers only | Any value |
| Emergency | Expedited process | Critical need |

### 2.3 User Classes

| User Class | Description | Access Level |
|------------|-------------|--------------|
| Procurement Officer | Creates and manages tenders | Full access |
| Procurement Manager | Approves tenders | Approve |
| Legal Officer | Reviews terms and conditions | Review |
| Supplier | Submits bids | Bid submission |

---

## 3. Functional Requirements

### 3.1 Tender Creation

#### FR-3.1.1 Create Tender
| ID | FR-3.1.1 |
|----|----------|
| **Description** | System shall allow creation of new tenders |
| **Priority** | High |

**Tender Fields:**
| Field | Type | Required |
|-------|------|----------|
| Tender Number | Auto-generated | Yes |
| Title | Text | Yes |
| Description | Rich Text | Yes |
| Tender Type | Select | Yes |
| Category | Select | Yes |
| Estimated Value | Currency | Yes |
| Publication Date | Date | Yes |
| Closing Date | Date | Yes |
| Closing Time | Time | Yes |
| Briefing Session | Date/Time | No |
| Briefing Mandatory | Boolean | No |

#### FR-3.1.2 Tender from Requisition
| ID | FR-3.1.2 |
|----|----------|
| **Description** | System shall create tender from approved requisition |
| **Priority** | High |

**Auto-populated Fields:**
- Item specifications
- Quantities
- Delivery requirements
- Budget reference
- Cost centre

#### FR-3.1.3 Tender Templates
| ID | FR-3.1.3 |
|----|----------|
| **Description** | System shall support tender templates |
| **Priority** | Medium |

**Template Components:**
- Standard terms and conditions
- Evaluation criteria structure
- Required document checklist
- Pricing schedule format

### 3.2 Document Management

#### FR-3.2.1 Tender Documents
| ID | FR-3.2.1 |
|----|----------|
| **Description** | System shall manage tender documents |
| **Priority** | High |

**Document Types:**
| Document | Description | Required |
|----------|-------------|----------|
| Tender Notice | Official advertisement | Yes |
| Specifications | Technical requirements | Yes |
| Terms & Conditions | Legal terms | Yes |
| Pricing Schedule | Bid pricing format | Yes |
| Site Plans | Location drawings | If applicable |
| Sample Forms | Returnable forms | Yes |

#### FR-3.2.2 Document Versioning
| ID | FR-3.2.2 |
|----|----------|
| **Description** | System shall version control documents |
| **Priority** | Medium |

**Version Control:**
- Track all document versions
- Show version history
- Highlight changes between versions
- Notify subscribers of updates

### 3.3 Tender Configuration

#### FR-3.3.1 Evaluation Criteria
| ID | FR-3.3.1 |
|----|----------|
| **Description** | System shall define evaluation criteria |
| **Priority** | High |

**Criteria Configuration:**
| Criterion | Weight Range | Description |
|-----------|--------------|-------------|
| Price | 0-100% | Pricing competitiveness |
| Technical | 0-100% | Technical capability |
| Experience | 0-100% | Relevant experience |
| B-BBEE | Per PPPFA | Preferential points |

#### FR-3.3.2 Submission Requirements
| ID | FR-3.3.2 |
|----|----------|
| **Description** | System shall configure bid requirements |
| **Priority** | High |

**Configurable Requirements:**
- Mandatory documents checklist
- Bid bond requirement and amount
- Validity period
- Maximum file sizes
- Accepted file formats

#### FR-3.3.3 Supplier Eligibility
| ID | FR-3.3.3 |
|----|----------|
| **Description** | System shall define supplier eligibility |
| **Priority** | High |

**Eligibility Criteria:**
- Minimum B-BBEE level
- Required certifications
- Registration requirements (CSD, etc.)
- Geographic restrictions
- Experience prerequisites

### 3.4 Tender Approval

#### FR-3.4.1 Approval Workflow
| ID | FR-3.4.1 |
|----|----------|
| **Description** | System shall route tender for approval |
| **Priority** | High |

**Approval Matrix:**
| Tender Value | Approvers |
|--------------|-----------|
| < R 500,000 | Procurement Manager |
| R 500,000 - R 5M | Procurement Manager + Legal |
| > R 5M | Procurement Manager + Legal + CFO |

#### FR-3.4.2 Approval Actions
| ID | FR-3.4.2 |
|----|----------|
| **Description** | System shall support approval actions |
| **Priority** | High |

**Actions:**
- Approve (proceed to publication)
- Return for revision (with comments)
- Reject (with reason)
- Escalate (to higher authority)

### 3.5 Publication

#### FR-3.5.1 Publish Tender
| ID | FR-3.5.1 |
|----|----------|
| **Description** | System shall publish approved tenders |
| **Priority** | High |

**Publication Channels:**
| Channel | Description |
|---------|-------------|
| Portal | GovProcure public portal |
| eTender | Government eTender portal |
| Email | Registered suppliers |
| Notice Board | Internal publication |

#### FR-3.5.2 Supplier Notification
| ID | FR-3.5.2 |
|----|----------|
| **Description** | System shall notify relevant suppliers |
| **Priority** | High |

**Notification Criteria:**
- Suppliers registered for category
- Suppliers with required B-BBEE level
- Suppliers in geographic area
- Previously awarded suppliers

#### FR-3.5.3 Public Portal Display
| ID | FR-3.5.3 |
|----|----------|
| **Description** | System shall display tender on public portal |
| **Priority** | High |

**Portal Features:**
- Searchable tender listing
- Filter by category, value, date
- Download tender documents
- Register interest
- Subscribe to updates

### 3.6 Clarifications

#### FR-3.6.1 Submit Clarification
| ID | FR-3.6.1 |
|----|----------|
| **Description** | System shall accept clarification requests |
| **Priority** | High |

**Clarification Fields:**
| Field | Type | Required |
|-------|------|----------|
| Subject | Text | Yes |
| Question | Text | Yes |
| Reference | Text | No |
| Attachments | File | No |

#### FR-3.6.2 Respond to Clarification
| ID | FR-3.6.2 |
|----|----------|
| **Description** | System shall manage clarification responses |
| **Priority** | High |

**Response Process:**
1. Question received and logged
2. Assigned to technical team
3. Response drafted
4. Response approved
5. Published to all bidders

#### FR-3.6.3 Clarification Distribution
| ID | FR-3.6.3 |
|----|----------|
| **Description** | System shall distribute clarifications |
| **Priority** | High |

**Distribution Rules:**
- All clarifications shared equally
- Supplier identity anonymized
- Published as addendum
- Email notification sent

### 3.7 Amendments

#### FR-3.7.1 Tender Amendment
| ID | FR-3.7.1 |
|----|----------|
| **Description** | System shall support tender amendments |
| **Priority** | Medium |

**Amendment Types:**
- Specification changes
- Closing date extension
- Document corrections
- Requirement modifications

#### FR-3.7.2 Amendment Notification
| ID | FR-3.7.2 |
|----|----------|
| **Description** | System shall notify of amendments |
| **Priority** | High |

**Notification Content:**
- Amendment number
- Summary of changes
- Revised documents
- New closing date (if applicable)

### 3.8 Bid Submission

#### FR-3.8.1 Online Bid Submission
| ID | FR-3.8.1 |
|----|----------|
| **Description** | System shall accept electronic bids |
| **Priority** | High |

**Submission Process:**
1. Supplier logs into portal
2. Supplier selects tender
3. Supplier completes pricing schedule
4. Supplier uploads documents
5. Supplier reviews submission
6. System validates completeness
7. Supplier confirms submission
8. System issues receipt

#### FR-3.8.2 Bid Encryption
| ID | FR-3.8.2 |
|----|----------|
| **Description** | System shall encrypt submitted bids |
| **Priority** | High |

**Security Measures:**
- Bids encrypted at submission
- Cannot be accessed until closing
- Dual-key decryption required
- Tamper detection

#### FR-3.8.3 Submission Validation
| ID | FR-3.8.3 |
|----|----------|
| **Description** | System shall validate bid submissions |
| **Priority** | High |

**Validation Checks:**
- All mandatory documents attached
- Pricing schedule complete
- Files within size limits
- Submission before deadline

#### FR-3.8.4 Late Bid Handling
| ID | FR-3.8.4 |
|----|----------|
| **Description** | System shall reject late submissions |
| **Priority** | High |

**Late Bid Rules:**
- System closes at exact deadline
- No submissions accepted after
- Late attempts logged
- Supplier notified of rejection

### 3.9 Bid Opening

#### FR-3.9.1 Bid Opening Ceremony
| ID | FR-3.9.1 |
|----|----------|
| **Description** | System shall facilitate bid opening |
| **Priority** | High |

**Opening Process:**
1. Authorized users present
2. Dual authorization to decrypt
3. Bids revealed sequentially
4. Basic details recorded
5. Opening register generated
6. Signed by witnesses

#### FR-3.9.2 Opening Register
| ID | FR-3.9.2 |
|----|----------|
| **Description** | System shall generate opening register |
| **Priority** | High |

**Register Contents:**
| Field | Description |
|-------|-------------|
| Bidder Name | Company name |
| Bid Amount | Total price |
| Submission Time | Timestamp |
| Documents | Checklist status |
| Witnesses | Opening witnesses |

### 3.10 Tender Cancellation

#### FR-3.10.1 Cancel Tender
| ID | FR-3.10.1 |
|----|----------|
| **Description** | System shall support tender cancellation |
| **Priority** | Medium |

**Cancellation Reasons:**
- No responsive bids
- Fundamental specification error
- Budget no longer available
- Requirement cancelled
- Force majeure

#### FR-3.10.2 Cancellation Notification
| ID | FR-3.10.2 |
|----|----------|
| **Description** | System shall notify cancellation |
| **Priority** | High |

**Notification Content:**
- Cancellation notice
- Reason for cancellation
- Bid bond release instructions
- Re-tender information (if applicable)

---

## 4. Non-Functional Requirements

### 4.1 Performance

| ID | Requirement | Target |
|----|-------------|--------|
| NFR-4.1.1 | Tender creation | < 5 minutes |
| NFR-4.1.2 | Document upload | 50MB in < 30 seconds |
| NFR-4.1.3 | Bid submission | < 2 minutes |
| NFR-4.1.4 | Concurrent submissions | 100+ simultaneous |

### 4.2 Security

| ID | Requirement |
|----|-------------|
| NFR-4.2.1 | Bid encryption (AES-256) |
| NFR-4.2.2 | Dual authorization for opening |
| NFR-4.2.3 | Complete audit trail |
| NFR-4.2.4 | IP logging for submissions |

### 4.3 Availability

| ID | Requirement |
|----|-------------|
| NFR-4.3.1 | 99.9% uptime during tender periods |
| NFR-4.3.2 | No planned maintenance during closing |
| NFR-4.3.3 | Failover for submission system |

---

## 5. User Interface Requirements

### 5.1 Tender Creation Form

```
┌─────────────────────────────────────────────────────────────────┐
│ Create New Tender                                          [X]  │
├─────────────────────────────────────────────────────────────────┤
│ Step 1 of 4: Basic Information                                  │
│                                                                  │
│  Tender Title *                                                 │
│  ┌────────────────────────────────────────────────────────────┐│
│  │ Enterprise Resource Planning System Implementation        ││
│  └────────────────────────────────────────────────────────────┘│
│                                                                  │
│  ┌────────────────────────┐  ┌────────────────────────────────┐│
│  │ Tender Type       ▼   │  │ Category                  ▼   ││
│  │ Request for Proposal  │  │ IT Systems & Software         ││
│  └────────────────────────┘  └────────────────────────────────┘│
│                                                                  │
│  ┌────────────────────────┐  ┌────────────────────────────────┐│
│  │ Estimated Value   R   │  │ Currency               ▼      ││
│  │ 2,500,000             │  │ ZAR - South African Rand      ││
│  └────────────────────────┘  └────────────────────────────────┘│
│                                                                  │
│  Description *                                                  │
│  ┌────────────────────────────────────────────────────────────┐│
│  │ The Department requires a comprehensive ERP solution...   ││
│  │                                                            ││
│  │                                                            ││
│  └────────────────────────────────────────────────────────────┘│
│                                                                  │
│                              [Cancel]  [Save Draft]  [Next →]   │
└─────────────────────────────────────────────────────────────────┘
```

### 5.2 Tender Listing (Public Portal)

```
┌─────────────────────────────────────────────────────────────────┐
│ Open Tenders                                    [Search...   🔍]│
├─────────────────────────────────────────────────────────────────┤
│ Category: [All ▼]  Value: [All ▼]  Closing: [Next 30 days ▼]   │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ TND-2026-007                              Closes: 15 Mar    │ │
│ │ Enterprise Resource Planning System Implementation          │ │
│ │                                                             │ │
│ │ Category: IT Systems    Value: R 2,500,000    Type: RFP    │ │
│ │ B-BBEE: Level 4+ required                                  │ │
│ │                                                             │ │
│ │ Briefing: 28 Feb 2026, 10:00 (Mandatory)                   │ │
│ │                                                             │ │
│ │         [View Details]  [Download Documents]  [Submit Bid] │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                  │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ TND-2026-008                              Closes: 20 Mar    │ │
│ │ Office Furniture Supply and Delivery                        │ │
│ │                                                             │ │
│ │ Category: Furniture     Value: R 450,000      Type: RFQ    │ │
│ │ B-BBEE: Level 2+ preferred                                 │ │
│ │                                                             │ │
│ │         [View Details]  [Download Documents]  [Submit Bid] │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                  │
│ Showing 1-10 of 23 tenders                    [< Prev] [Next >] │
└─────────────────────────────────────────────────────────────────┘
```

### 5.3 Bid Submission Portal

```
┌─────────────────────────────────────────────────────────────────┐
│ Submit Bid: TND-2026-007                                   [X]  │
│ Enterprise Resource Planning System Implementation              │
├─────────────────────────────────────────────────────────────────┤
│ Closing: 15 March 2026, 11:00 SAST          Time remaining: 5d  │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│ 1. PRICING SCHEDULE                                    ✓ Done   │
│ ┌─────────────────────────────────────────────────────────────┐│
│ │ Software Licenses        R 1,200,000                       ││
│ │ Implementation Services  R 800,000                         ││
│ │ Training                 R 150,000                         ││
│ │ Annual Support (Year 1)  R 190,000                         ││
│ │ ─────────────────────────────────────                      ││
│ │ TOTAL (excl VAT)         R 2,340,000                       ││
│ └─────────────────────────────────────────────────────────────┘│
│                                                                  │
│ 2. REQUIRED DOCUMENTS                                           │
│ ┌─────────────────────────────────────────────────────────────┐│
│ │ ✓ Company Registration (CIPC)           company_reg.pdf    ││
│ │ ✓ B-BBEE Certificate                    bbbee_cert.pdf     ││
│ │ ✓ Tax Clearance Certificate             tax_clear.pdf      ││
│ │ ✓ Technical Proposal                    tech_proposal.pdf  ││
│ │ ○ Reference Letters (3 required)        [Upload]           ││
│ │ ✓ Bid Bond / Guarantee                  bid_bond.pdf       ││
│ └─────────────────────────────────────────────────────────────┘│
│                                                                  │
│ ⚠ 1 document still required                                    │
│                                                                  │
│ ☐ I confirm all information is accurate and binding            │
│                                                                  │
│                              [Save Draft]  [Submit Bid]         │
└─────────────────────────────────────────────────────────────────┘
```

---

## 6. Data Requirements

### 6.1 Tender Entity

| Field | Type | Description |
|-------|------|-------------|
| tender_id | VARCHAR(20) | Primary key (TND-YYYY-NNN) |
| title | VARCHAR(200) | Tender title |
| description | TEXT | Full description |
| tender_type | ENUM | RFQ/RFP/ITT/Limited/Emergency |
| category_id | VARCHAR(20) | Category reference |
| estimated_value | DECIMAL(15,2) | Estimated contract value |
| currency | VARCHAR(3) | Currency code |
| publication_date | DATETIME | When published |
| closing_date | DATETIME | Submission deadline |
| briefing_date | DATETIME | Briefing session date |
| briefing_mandatory | BOOLEAN | Briefing required |
| status | ENUM | Draft/Pending/Open/Closed/Cancelled |
| created_by | VARCHAR(20) | Creator |
| approved_by | VARCHAR(20) | Approver |

### 6.2 Tender Document Entity

| Field | Type | Description |
|-------|------|-------------|
| document_id | VARCHAR(20) | Primary key |
| tender_id | VARCHAR(20) | Foreign key |
| document_type | ENUM | Notice/Specs/T&C/Pricing/Other |
| title | VARCHAR(200) | Document title |
| file_path | VARCHAR(500) | Storage path |
| file_size | INT | Size in bytes |
| version | INT | Version number |
| uploaded_by | VARCHAR(20) | Uploader |
| uploaded_date | DATETIME | Upload timestamp |

### 6.3 Bid Submission Entity

| Field | Type | Description |
|-------|------|-------------|
| bid_id | VARCHAR(20) | Primary key |
| tender_id | VARCHAR(20) | Foreign key |
| supplier_id | VARCHAR(20) | Bidder |
| total_amount | DECIMAL(15,2) | Bid total |
| submission_time | DATETIME | Submission timestamp |
| ip_address | VARCHAR(45) | Submitter IP |
| status | ENUM | Draft/Submitted/Opened/Evaluated |
| encrypted_data | BLOB | Encrypted bid content |
| decryption_key | VARCHAR(500) | Encryption key (stored separately) |

### 6.4 Clarification Entity

| Field | Type | Description |
|-------|------|-------------|
| clarification_id | VARCHAR(20) | Primary key |
| tender_id | VARCHAR(20) | Foreign key |
| supplier_id | VARCHAR(20) | Questioner |
| question | TEXT | Clarification question |
| response | TEXT | Official response |
| question_date | DATETIME | Question timestamp |
| response_date | DATETIME | Response timestamp |
| responded_by | VARCHAR(20) | Responder |
| published | BOOLEAN | Published to all |

---

## 7. Use Cases

### UC-01: Create and Publish Tender

**Actor:** Procurement Officer

**Preconditions:**
- Approved requisition exists
- User has tender creation rights

**Main Flow:**
1. Officer creates new tender
2. Officer populates tender details
3. Officer configures evaluation criteria
4. Officer uploads tender documents
5. Officer submits for approval
6. Manager reviews tender
7. Manager approves tender
8. System publishes to portal
9. System notifies registered suppliers

**Postconditions:**
- Tender visible on public portal
- Suppliers can download documents
- Bid submission enabled

---

### UC-02: Submit Bid

**Actor:** Supplier

**Preconditions:**
- Supplier registered on portal
- Tender is open

**Main Flow:**
1. Supplier views tender on portal
2. Supplier downloads tender documents
3. Supplier prepares bid offline
4. Supplier logs into submission portal
5. Supplier completes pricing schedule
6. Supplier uploads required documents
7. System validates completeness
8. Supplier confirms submission
9. System encrypts and stores bid
10. System issues submission receipt

**Postconditions:**
- Bid stored encrypted
- Supplier receives confirmation
- Bid counts toward tender

---

### UC-03: Manage Clarifications

**Actor:** Procurement Officer

**Main Flow:**
1. Supplier submits clarification request
2. System logs and assigns to Officer
3. Officer consults technical team
4. Officer drafts response
5. Manager approves response
6. System publishes as addendum
7. System notifies all registered suppliers

**Postconditions:**
- Clarification published
- All suppliers have equal access

---

### UC-04: Open Bids

**Actor:** Procurement Manager, Procurement Officer

**Preconditions:**
- Tender closing time passed
- At least 2 authorized officials present

**Main Flow:**
1. Manager initiates bid opening
2. Officer provides second authorization
3. System decrypts first bid
4. System displays bidder and amount
5. Officer records in opening register
6. Steps 3-5 repeat for all bids
7. Officials sign opening register
8. System generates opening report

**Postconditions:**
- All bids decrypted and recorded
- Opening register signed
- Ready for evaluation

---

## 8. Business Rules

### BR-01: Tender Timelines
- Minimum tender period: 21 days (standard)
- Minimum tender period: 14 days (limited)
- Emergency tender: 7 days minimum
- Briefing must be 7+ days before closing

### BR-02: Publication Requirements
- Tenders > R 500,000 must be published on eTender
- All tenders published on GovProcure portal
- Local newspapers for tenders > R 10M

### BR-03: Submission Rules
- No late submissions accepted
- Incomplete submissions marked for review
- Missing mandatory documents = non-responsive
- Bid bonds returned to unsuccessful bidders

### BR-04: Clarification Window
- Questions accepted until 7 days before closing
- Responses published within 3 business days
- Questions after cut-off not accepted

### BR-05: Amendment Rules
- Amendments extend closing by minimum 7 days
- Material amendments require re-publication
- Maximum 3 amendments per tender

### BR-06: Minimum Bids Required
- Competitive tenders require minimum 3 bids
- If < 3 bids, tender may be cancelled
- Limited tender requires minimum 2 bids

### BR-07: Conflict of Interest
- Employees cannot bid on own department tenders
- Related parties disclosed and managed
- Bidders must declare conflicts

---

**End of Document**
