# Software Requirements Specification (SRS)
# Invoice Management Module

**Document Version:** 1.0
**Date:** 27 February 2026
**Project:** GovProcure eProcurement Platform
**Module:** Invoice Processing & Payment

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

This SRS document describes the requirements for the Invoice Management module of the GovProcure eProcurement Platform. This module manages the complete invoice lifecycle from receipt to payment.

### 1.2 Scope

The Invoice Management module encompasses:

- Invoice registration and capture
- Three-way matching (Invoice, PO, GRN)
- Invoice approval workflow
- Payment processing and tracking
- Aging analysis and reporting
- Integration with financial systems

### 1.3 Definitions

| Term | Definition |
|------|------------|
| GRN | Goods Received Note |
| Three-way Match | Verification of Invoice against PO and GRN |
| Aging | Classification of invoices by days outstanding |
| EFT | Electronic Funds Transfer |

---

## 2. Overall Description

### 2.1 Product Perspective

```
┌─────────────────────────────────────────────────────────────────┐
│                    INVOICE PROCESSING FLOW                       │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐     │
│  │ Receive  │ → │  Match   │ → │ Approve  │ → │  Pay     │     │
│  │ Invoice  │   │ PO/GRN   │   │ Invoice  │   │ Invoice  │     │
│  └──────────┘   └──────────┘   └──────────┘   └──────────┘     │
│       │              │              │              │            │
│       ▼              ▼              ▼              ▼            │
│  ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐     │
│  │Registered│   │ Matched/ │   │ Approved │   │   Paid   │     │
│  │          │   │Exception │   │          │   │          │     │
│  └──────────┘   └──────────┘   └──────────┘   └──────────┘     │
└─────────────────────────────────────────────────────────────────┘
```

### 2.2 User Classes

| User Class | Description | Access Level |
|------------|-------------|--------------|
| Accounts Clerk | Captures and matches invoices | Create, Match |
| Accounts Payable | Processes payments | Full access |
| Finance Manager | Approves payments | Approve |
| CFO | Approves large payments | Final approval |

---

## 3. Functional Requirements

### 3.1 Invoice Registration

#### FR-3.1.1 Register Invoice
| ID | FR-3.1.1 |
|----|----------|
| **Description** | System shall register supplier invoices |
| **Priority** | High |

**Invoice Fields:**
| Field | Type | Required |
|-------|------|----------|
| Invoice Number | Text | Yes |
| Invoice Date | Date | Yes |
| Due Date | Date | Yes |
| Supplier | Select | Yes |
| PO Reference | Select | Yes |
| Currency | Select | Yes |
| Subtotal | Currency | Yes |
| VAT Amount | Currency | Yes |
| Total Amount | Currency | Yes |
| Description | Text | No |

#### FR-3.1.2 OCR Integration
| ID | FR-3.1.2 |
|----|----------|
| **Description** | System shall integrate with OCR module |
| **Priority** | High |

- Auto-populate fields from OCR extraction
- Link to scanned document
- Retain confidence scores

#### FR-3.1.3 Duplicate Detection
| ID | FR-3.1.3 |
|----|----------|
| **Description** | System shall detect duplicate invoices |
| **Priority** | High |

**Detection Criteria:**
- Same invoice number + supplier
- Same amount + date + supplier
- Similar invoice number patterns

### 3.2 Three-Way Matching

#### FR-3.2.1 PO Matching
| ID | FR-3.2.1 |
|----|----------|
| **Description** | System shall match invoice to purchase order |
| **Priority** | High |

**Match Criteria:**
| Element | Tolerance |
|---------|-----------|
| Supplier | Exact match |
| PO Number | Exact match |
| Line items | Description match |
| Quantities | ±5% |
| Prices | ±2% |
| Total | ±2% |

#### FR-3.2.2 GRN Matching
| ID | FR-3.2.2 |
|----|----------|
| **Description** | System shall match invoice to goods received |
| **Priority** | High |

**Match Criteria:**
- Goods received against PO line items
- Quantities received vs invoiced
- Quality acceptance status

#### FR-3.2.3 Match Results
| ID | FR-3.2.3 |
|----|----------|
| **Description** | System shall display match results |
| **Priority** | High |

**Match Statuses:**
| Status | Description |
|--------|-------------|
| Full Match | All criteria met |
| Partial Match | Some variances within tolerance |
| Exception | Variances exceed tolerance |
| No Match | PO/GRN not found |

### 3.3 Invoice Approval

#### FR-3.3.1 Approval Workflow
| ID | FR-3.3.1 |
|----|----------|
| **Description** | System shall route invoices for approval |
| **Priority** | High |

**Approval Matrix:**
| Invoice Value | Approvers |
|---------------|-----------|
| < R 50,000 | Finance Manager |
| R 50,000 - R 500,000 | Finance Manager + CFO |
| > R 500,000 | Finance Manager + CFO + Board |

#### FR-3.3.2 Exception Handling
| ID | FR-3.3.2 |
|----|----------|
| **Description** | System shall handle matching exceptions |
| **Priority** | High |

**Exception Actions:**
- Accept variance with justification
- Reject invoice
- Request credit note
- Hold for investigation

### 3.4 Payment Processing

#### FR-3.4.1 Payment Scheduling
| ID | FR-3.4.1 |
|----|----------|
| **Description** | System shall schedule payments |
| **Priority** | High |

**Scheduling Options:**
- Pay on due date
- Pay early (for discount)
- Hold payment
- Batch payments

#### FR-3.4.2 Payment Methods
| ID | FR-3.4.2 |
|----|----------|
| **Description** | System shall support payment methods |
| **Priority** | High |

| Method | Description |
|--------|-------------|
| EFT | Electronic transfer |
| Cheque | Manual cheque |
| Card | Corporate card |

#### FR-3.4.3 Payment File Generation
| ID | FR-3.4.3 |
|----|----------|
| **Description** | System shall generate bank payment files |
| **Priority** | High |

**File Formats:**
- ABSA PayBatch
- Standard Bank Host-to-Host
- FNB Online

### 3.5 Invoice Tracking

#### FR-3.5.1 Invoice Status
| ID | FR-3.5.1 |
|----|----------|
| **Description** | System shall track invoice status |
| **Priority** | High |

**Statuses:**
- Registered
- Matched
- Exception
- Pending Approval
- Approved
- Scheduled
- Paid
- Cancelled

#### FR-3.5.2 Aging Analysis
| ID | FR-3.5.2 |
|----|----------|
| **Description** | System shall calculate invoice aging |
| **Priority** | Medium |

**Aging Buckets:**
- Current (0-30 days)
- 30-60 days
- 60-90 days
- 90+ days (Overdue)

### 3.6 Reporting

#### FR-3.6.1 Invoice Register
| ID | FR-3.6.1 |
|----|----------|
| **Description** | System shall provide invoice listing |
| **Priority** | High |

**Report Columns:**
- Invoice number
- Supplier
- PO Reference
- Amount
- Due Date
- Status
- Age

#### FR-3.6.2 Aging Report
| ID | FR-3.6.2 |
|----|----------|
| **Description** | System shall generate aging report |
| **Priority** | Medium |

#### FR-3.6.3 Payment Report
| ID | FR-3.6.3 |
|----|----------|
| **Description** | System shall generate payment reports |
| **Priority** | Medium |

---

## 4. Non-Functional Requirements

### 4.1 Performance

| ID | Requirement | Target |
|----|-------------|--------|
| NFR-4.1.1 | Invoice registration | < 2 seconds |
| NFR-4.1.2 | Three-way matching | < 5 seconds |
| NFR-4.1.3 | Payment file generation | < 30 seconds |

### 4.2 Security

| ID | Requirement |
|----|-------------|
| NFR-4.2.1 | Segregation of duties (capture vs approve) |
| NFR-4.2.2 | Payment approval audit trail |
| NFR-4.2.3 | Encrypt bank details |

---

## 5. User Interface Requirements

### 5.1 Invoice Register

```
┌─────────────────────────────────────────────────────────────────┐
│ Invoices                                     [+ Upload Invoice] │
├─────────────────────────────────────────────────────────────────┤
│ ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────────┐│
│ │   R 2.1M    │ │     47      │ │     12      │ │      3      ││
│ │ Total MTD   │ │    Paid     │ │   Pending   │ │   Overdue   ││
│ │   ↑ 8%      │ │  On time    │ │  Awaiting   │ │   Action    ││
│ └─────────────┘ └─────────────┘ └─────────────┘ └─────────────┘│
├─────────────────────────────────────────────────────────────────┤
│ Invoice Register                                       [Export] │
├─────────────────────────────────────────────────────────────────┤
│ Invoice #    │ Supplier         │ PO Ref     │ Amount    │Status│
│──────────────┼──────────────────┼────────────┼───────────┼──────│
│ INV-0847     │ Datacom          │ PO-0412    │ R 124,500 │ Paid │
│ INV-0846     │ TechServ         │ PO-0409    │ R 290,000 │Pendng│
│ INV-0845     │ BuildRight       │ PO-0410    │ R  56,000 │Procss│
│ INV-0844     │ Apex Stationery  │ PO-0411    │ R   8,200 │Overdu│
└─────────────────────────────────────────────────────────────────┘
```

---

## 6. Data Requirements

### 6.1 Invoice Entity

| Field | Type | Description |
|-------|------|-------------|
| invoice_id | VARCHAR(20) | Primary key |
| invoice_number | VARCHAR(50) | Supplier invoice number |
| supplier_id | VARCHAR(20) | Foreign key |
| po_id | VARCHAR(20) | Purchase order reference |
| invoice_date | DATE | Invoice date |
| due_date | DATE | Payment due date |
| subtotal | DECIMAL(15,2) | Subtotal amount |
| vat_amount | DECIMAL(15,2) | VAT amount |
| total_amount | DECIMAL(15,2) | Total amount |
| status | ENUM | Invoice status |
| match_status | ENUM | Three-way match status |
| paid_date | DATE | Actual payment date |
| paid_amount | DECIMAL(15,2) | Amount paid |

---

## 7. Use Cases

### UC-01: Register and Match Invoice

**Actor:** Accounts Clerk

**Main Flow:**
1. Clerk uploads or OCR scans invoice
2. System extracts invoice data
3. Clerk verifies and corrects data
4. Clerk submits invoice
5. System performs three-way matching
6. System displays match results
7. If matched, system routes for approval
8. If exception, clerk handles variance

---

### UC-02: Process Payment

**Actor:** Accounts Payable

**Main Flow:**
1. User views approved invoices
2. User selects invoices for payment
3. User selects payment date
4. System validates bank details
5. User generates payment file
6. System creates payment batch
7. User downloads bank file
8. User uploads to banking system
9. User confirms payment processed
10. System updates invoice status

---

## 8. Business Rules

### BR-01: Payment Terms
- Standard payment terms: 30 days
- Early payment discount: 2% if paid within 10 days
- Late payment interest: Prime + 2%

### BR-02: Matching Tolerances
- Quantity variance: ±5%
- Price variance: ±2%
- Total variance: ±2%
- Variances outside tolerance require approval

### BR-03: Duplicate Prevention
- Same invoice number from same supplier rejected
- Warning if similar amount within 7 days

### BR-04: Approval Limits
- Single approver: < R 50,000
- Dual approval: R 50,000 - R 500,000
- Board approval: > R 500,000

### BR-05: Payment Prioritization
1. Overdue invoices
2. Early payment discount eligible
3. By due date

---

**End of Document**
