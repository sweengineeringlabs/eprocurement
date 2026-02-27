# Software Requirements Specification (SRS)
# OCR Document Processing Module

**Document Version:** 1.0
**Date:** 27 February 2026
**Project:** GovProcure eProcurement Platform
**Module:** Intelligent Document Scan (OCR)

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

This SRS document describes the requirements for the OCR Document Processing module of the GovProcure eProcurement Platform. This module uses AI-powered optical character recognition to extract structured data from procurement documents.

### 1.2 Scope

The OCR Document Processing module encompasses:

- Document upload and image preprocessing
- AI-powered text extraction (Claude Vision API)
- Document type classification
- Field extraction and validation
- Confidence scoring
- Data verification and correction
- Integration with procurement workflows
- Processing history and audit trail

### 1.3 Definitions

| Term | Definition |
|------|------------|
| OCR | Optical Character Recognition |
| Confidence Score | AI's certainty level for extracted data (0-100%) |
| Field Extraction | Identifying and extracting specific data points |
| Document Classification | Automatically identifying document type |

---

## 2. Overall Description

### 2.1 Product Perspective

```
┌─────────────────────────────────────────────────────────────────┐
│                    OCR PROCESSING PIPELINE                       │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐     │
│  │  Upload  │ → │ Preproc  │ → │  Vision  │ → │  Parse   │     │
│  │ Document │   │  Image   │   │   API    │   │  Data    │     │
│  └──────────┘   └──────────┘   └──────────┘   └──────────┘     │
│       │              │              │              │            │
│       ▼              ▼              ▼              ▼            │
│  ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐     │
│  │  File    │   │  Image   │   │  Text    │   │ Struct.  │     │
│  │ Received │   │ Enhanced │   │ Extract  │   │  Data    │     │
│  └──────────┘   └──────────┘   └──────────┘   └──────────┘     │
│                                                     │           │
│                                                     ▼           │
│                                              ┌──────────┐       │
│                                              │ Validate │       │
│                                              │  & Save  │       │
│                                              └──────────┘       │
└─────────────────────────────────────────────────────────────────┘
```

### 2.2 Supported Document Types

| Document Type | Code | Description |
|---------------|------|-------------|
| Invoice | INV | Supplier invoices |
| Purchase Order | PO | Purchase orders |
| Delivery Note | DN | Goods received notes |
| Quotation | QT | Price quotations |
| Contract | CON | Service contracts |

### 2.3 User Classes

| User Class | Description | Access Level |
|------------|-------------|--------------|
| Procurement Officer | Scans and processes documents | Full access |
| Finance Officer | Processes invoices | Invoice processing |
| Clerk | Basic document scanning | Upload only |
| Auditor | Reviews processed documents | Read-only |

---

## 3. Functional Requirements

### 3.1 Document Upload

#### FR-3.1.1 File Upload
| ID | FR-3.1.1 |
|----|----------|
| **Description** | System shall accept document uploads |
| **Priority** | High |

**Supported Formats:**
| Format | Max Size | Notes |
|--------|----------|-------|
| PDF | 10 MB | Multi-page supported |
| JPEG/JPG | 10 MB | Single image |
| PNG | 10 MB | Single image |
| TIFF | 10 MB | Multi-page supported |
| WEBP | 10 MB | Single image |

**Upload Methods:**
- Drag and drop
- File browser selection
- Bulk upload (up to 10 files)

#### FR-3.1.2 Document Type Selection
| ID | FR-3.1.2 |
|----|----------|
| **Description** | System shall allow document type selection |
| **Priority** | High |

**Options:**
- Invoice
- Purchase Order
- Delivery Note
- Quotation
- Contract
- Auto-Detect (default)

#### FR-3.1.3 Preview Document
| ID | FR-3.1.3 |
|----|----------|
| **Description** | System shall display document preview |
| **Priority** | Medium |

**Preview Features:**
- Thumbnail display
- Zoom capability
- Page navigation (multi-page)
- Rotate image

### 3.2 Image Preprocessing

#### FR-3.2.1 Image Enhancement
| ID | FR-3.2.1 |
|----|----------|
| **Description** | System shall enhance image quality |
| **Priority** | High |

**Enhancement Steps:**
1. Deskew (straighten tilted images)
2. Noise reduction
3. Contrast optimization
4. Binarization (for poor quality scans)
5. Resolution normalization

#### FR-3.2.2 Page Detection
| ID | FR-3.2.2 |
|----|----------|
| **Description** | System shall detect page boundaries |
| **Priority** | Medium |

- Detect multiple pages in single image
- Auto-crop to document edges
- Remove borders and margins

### 3.3 AI Vision Processing

#### FR-3.3.1 Text Extraction
| ID | FR-3.3.1 |
|----|----------|
| **Description** | System shall extract text using Claude Vision API |
| **Priority** | High |

**Processing Steps:**
1. Send preprocessed image to Claude Vision API
2. Extract all visible text
3. Identify document structure (tables, headers, etc.)
4. Parse into structured format
5. Calculate confidence scores

#### FR-3.3.2 Document Classification
| ID | FR-3.3.2 |
|----|----------|
| **Description** | System shall classify document type |
| **Priority** | High |

**Classification Output:**
- Document type (Invoice, PO, etc.)
- Confidence level
- Key identifying features detected

#### FR-3.3.3 Field Extraction
| ID | FR-3.3.3 |
|----|----------|
| **Description** | System shall extract specific fields |
| **Priority** | High |

**Invoice Fields:**
| Field | Type | Required |
|-------|------|----------|
| Invoice Number | Text | Yes |
| Invoice Date | Date | Yes |
| Due Date | Date | Yes |
| Supplier Name | Text | Yes |
| Supplier VAT | Text | Yes |
| PO Reference | Text | No |
| Subtotal | Currency | Yes |
| VAT Amount | Currency | Yes |
| Total Amount | Currency | Yes |
| Bank Account | Text | No |
| Payment Terms | Text | No |

**Purchase Order Fields:**
| Field | Type | Required |
|-------|------|----------|
| PO Number | Text | Yes |
| PO Date | Date | Yes |
| Supplier Name | Text | Yes |
| Delivery Date | Date | No |
| Total Amount | Currency | Yes |

**Line Item Fields:**
| Field | Type |
|-------|------|
| Description | Text |
| Quantity | Number |
| Unit | Text |
| Unit Price | Currency |
| VAT % | Number |
| Line Total | Currency |

### 3.4 Confidence Scoring

#### FR-3.4.1 Field Confidence
| ID | FR-3.4.1 |
|----|----------|
| **Description** | System shall assign confidence to each field |
| **Priority** | High |

**Confidence Levels:**
| Level | Range | Indicator |
|-------|-------|-----------|
| High | 90-100% | Green |
| Medium | 70-89% | Orange |
| Low | <70% | Red |

#### FR-3.4.2 Overall Confidence
| ID | FR-3.4.2 |
|----|----------|
| **Description** | System shall calculate overall document confidence |
| **Priority** | High |

**Calculation:**
- Weighted average of all field confidences
- Critical fields weighted higher (invoice number, total)
- Display as percentage with visual indicator

### 3.5 Validation

#### FR-3.5.1 Business Rule Validation
| ID | FR-3.5.1 |
|----|----------|
| **Description** | System shall validate extracted data |
| **Priority** | High |

**Validation Rules:**
| Rule | Description | Type |
|------|-------------|------|
| VAT Calculation | VAT = Subtotal × 15% | Auto-check |
| Total Verification | Total = Subtotal + VAT | Auto-check |
| Date Logic | Due Date > Invoice Date | Auto-check |
| Supplier Match | Supplier exists in database | Database lookup |
| PO Match | PO Reference exists | Database lookup |
| Duplicate Check | Invoice number not duplicate | Database lookup |

#### FR-3.5.2 Validation Results
| ID | FR-3.5.2 |
|----|----------|
| **Description** | System shall display validation results |
| **Priority** | High |

**Result Types:**
- Pass (green checkmark)
- Warning (orange alert)
- Fail (red cross)

### 3.6 Data Correction

#### FR-3.6.1 Manual Correction
| ID | FR-3.6.1 |
|----|----------|
| **Description** | System shall allow manual data correction |
| **Priority** | High |

**Features:**
- Editable fields for all extracted data
- Side-by-side document view
- Highlight fields in document
- Track corrections for audit

#### FR-3.6.2 Line Item Editing
| ID | FR-3.6.2 |
|----|----------|
| **Description** | System shall allow line item editing |
| **Priority** | High |

**Actions:**
- Edit existing line items
- Add missing line items
- Delete incorrect line items
- Recalculate totals

### 3.7 Save and Integration

#### FR-3.7.1 Save to System
| ID | FR-3.7.1 |
|----|----------|
| **Description** | System shall save extracted data |
| **Priority** | High |

**Save Actions:**
- Create invoice record
- Link to purchase order
- Attach original document
- Update processing history

#### FR-3.7.2 Workflow Integration
| ID | FR-3.7.2 |
|----|----------|
| **Description** | System shall integrate with workflows |
| **Priority** | High |

**Integrations:**
| Document Type | Integration |
|---------------|-------------|
| Invoice | Create invoice, route to Accounts Payable |
| Delivery Note | Update PO delivery status |
| Quotation | Create quotation record for RFQ |

### 3.8 Processing History

#### FR-3.8.1 View History
| ID | FR-3.8.1 |
|----|----------|
| **Description** | System shall display processing history |
| **Priority** | Medium |

**History Fields:**
- Scan ID
- Document Type
- Supplier
- Amount
- Confidence
- Processed By
- Date/Time
- Status

#### FR-3.8.2 Reprocess Document
| ID | FR-3.8.2 |
|----|----------|
| **Description** | System shall allow reprocessing |
| **Priority** | Low |

- Re-run OCR on existing document
- Compare with previous extraction
- Track version history

---

## 4. Non-Functional Requirements

### 4.1 Performance

| ID | Requirement | Target |
|----|-------------|--------|
| NFR-4.1.1 | Single page processing | < 5 seconds |
| NFR-4.1.2 | Multi-page processing (10 pages) | < 30 seconds |
| NFR-4.1.3 | File upload (5MB) | < 10 seconds |
| NFR-4.1.4 | Preview generation | < 2 seconds |

### 4.2 Accuracy

| ID | Requirement | Target |
|----|-------------|--------|
| NFR-4.2.1 | Text extraction accuracy | > 95% |
| NFR-4.2.2 | Field extraction accuracy | > 90% |
| NFR-4.2.3 | Document classification | > 98% |

### 4.3 Security

| ID | Requirement |
|----|-------------|
| NFR-4.3.1 | Encrypt documents at rest |
| NFR-4.3.2 | Secure API communication (HTTPS) |
| NFR-4.3.3 | Delete temporary files after processing |
| NFR-4.3.4 | Access control for sensitive documents |

---

## 5. User Interface Requirements

### 5.1 Upload Interface

```
┌─────────────────────────────────────────────────────────────────┐
│ Upload Document for OCR Processing           Powered by Claude │
├─────────────────────────────────────────────────────────────────┤
│ Document Type:                                                   │
│ [Invoice] [Purchase Order] [Delivery Note] [Quotation] [Auto]  │
├─────────────────────────────────────────────────────────────────┤
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │                                                             │ │
│ │                    ┌───────┐                                │ │
│ │                    │  ↑    │                                │ │
│ │                    │  │    │                                │ │
│ │                    └───────┘                                │ │
│ │                                                             │ │
│ │           Drag & drop your document here                    │ │
│ │                                                             │ │
│ │    Or click to browse. Scanned PDFs, photos, and images    │ │
│ │                     are all supported.                      │ │
│ │                                                             │ │
│ │         [PDF] [JPG] [PNG] [TIFF] [WEBP]                    │ │
│ │                                                             │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                  │
│ [Process Document with AI OCR]                          [Clear] │
└─────────────────────────────────────────────────────────────────┘
```

### 5.2 Processing Steps Panel

```
┌─────────────────────────────────────────────────────────────────┐
│ Processing Steps                                                 │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ✓  1. Upload                                                   │
│      Document received                                          │
│                                                                  │
│  ✓  2. Pre-process                                              │
│      Image enhancement                                          │
│                                                                  │
│  ◉  3. Vision Analysis                    [████████░░] 80%     │
│      Claude AI extraction                                       │
│                                                                  │
│  ○  4. Parse Data                                               │
│      Structure fields                                           │
│                                                                  │
│  ○  5. Validate                                                 │
│      Business rules check                                       │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘

Legend: ✓ Complete  ◉ In Progress  ○ Pending
```

### 5.3 Results View

```
┌─────────────────────────────────────────────────────────────────┐
│ ✓ Extracted Data — Invoice                              [96%]  │
│   Review and correct extracted fields before saving    ┌─────┐ │
│                                                        │ 96% │ │
│                                                        └─────┘ │
├─────────────────────────────────────────────────────────────────┤
│ DOCUMENT HEADER FIELDS                                          │
│ ───────────────────────────────────────────────────────────────│
│ Invoice Number      │ INV-2026-00412              │ 98% │      │
│ Invoice Date        │ 2026-02-24                  │ 98% │      │
│ Due Date            │ 2026-03-26                  │ 95% │      │
│ PO Reference        │ PO-2026-0412                │ 99% │      │
│ Supplier Name       │ Datacom Solutions (Pty) Ltd │ 98% │      │
│ Supplier VAT        │ 4620183751                  │ 96% │      │
│ Bank Account        │ 62843917024                 │ 82% │      │
├─────────────────────────────────────────────────────────────────┤
│ LINE ITEMS                                                       │
│ ───────────────────────────────────────────────────────────────│
│ # │ Description                    │ Qty │ Price  │ Total     │
│───┼────────────────────────────────┼─────┼────────┼───────────│
│ 1 │ Dell Latitude 5540 Laptop      │ 15  │ 18,500 │ 277,500 ✓│
│ 2 │ Dell USB-C Dock WD19S          │ 15  │  2,400 │  36,000 ✓│
│ 3 │ Logitech MX Master 3 Mouse     │ 20  │    850 │  17,000 ⚠│
│───┴────────────────────────────────┴─────┴────────┼───────────│
│                                         Subtotal  │ 330,500   │
│                                         VAT (15%) │  49,575   │
│                                         TOTAL     │ 380,075   │
├─────────────────────────────────────────────────────────────────┤
│ VALIDATION RESULTS                                               │
│ ✓ Supplier VAT number verified                                  │
│ ✓ Invoice matches PO reference                                  │
│ ⚠ Line item 3 qty exceeds PO                                   │
│ ✓ VAT calculation correct (15%)                                 │
├─────────────────────────────────────────────────────────────────┤
│ [Scan Another]                    [Save to Procurement System]  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 6. Data Requirements

### 6.1 Data Model

#### 6.1.1 OCR Scan Entity

| Field | Type | Description |
|-------|------|-------------|
| scan_id | VARCHAR(20) | Primary key (OCR-YYYY-NNNN) |
| document_type | ENUM | Invoice, PO, DN, QT, CON |
| file_name | VARCHAR(255) | Original file name |
| file_path | VARCHAR(500) | Storage path |
| file_size | INT | Size in bytes |
| mime_type | VARCHAR(50) | File MIME type |
| page_count | INT | Number of pages |
| overall_confidence | DECIMAL(5,2) | Overall confidence % |
| status | ENUM | Processing, Complete, Failed, Saved |
| processed_by | VARCHAR(50) | User ID |
| processed_at | TIMESTAMP | Processing timestamp |
| processing_time_ms | INT | Processing duration |

#### 6.1.2 Extracted Field Entity

| Field | Type | Description |
|-------|------|-------------|
| field_id | INT | Primary key |
| scan_id | VARCHAR(20) | Foreign key |
| field_name | VARCHAR(50) | Field identifier |
| field_value | TEXT | Extracted value |
| confidence | DECIMAL(5,2) | Confidence % |
| corrected | BOOLEAN | Was manually corrected |
| original_value | TEXT | Value before correction |

#### 6.1.3 Extracted Line Item Entity

| Field | Type | Description |
|-------|------|-------------|
| line_id | INT | Primary key |
| scan_id | VARCHAR(20) | Foreign key |
| line_number | INT | Line sequence |
| description | TEXT | Item description |
| quantity | DECIMAL(10,2) | Quantity |
| unit | VARCHAR(20) | Unit of measure |
| unit_price | DECIMAL(15,2) | Unit price |
| vat_percent | DECIMAL(5,2) | VAT percentage |
| line_total | DECIMAL(15,2) | Line total |
| flagged | BOOLEAN | Has validation issue |
| flag_reason | TEXT | Reason for flag |

#### 6.1.4 Validation Result Entity

| Field | Type | Description |
|-------|------|-------------|
| validation_id | INT | Primary key |
| scan_id | VARCHAR(20) | Foreign key |
| rule_code | VARCHAR(20) | Validation rule |
| status | ENUM | Pass, Warn, Fail |
| message | TEXT | Validation message |

---

## 7. Use Cases

### UC-01: Process Invoice

**Actor:** Procurement Officer

**Preconditions:**
- User has OCR access permission
- Invoice document available (PDF/image)

**Main Flow:**
1. User navigates to OCR Document Scan
2. User selects "Invoice" document type
3. User drags document to upload zone
4. System displays document preview
5. User clicks "Process Document"
6. System shows processing steps with progress
7. System extracts text using Claude Vision API
8. System parses invoice fields and line items
9. System validates extracted data
10. System displays results with confidence scores
11. User reviews extracted data
12. User corrects any errors
13. User clicks "Save to Procurement System"
14. System creates invoice record
15. System displays confirmation

**Alternative Flows:**
- 8a. Low confidence: System highlights fields for review
- 9a. Validation fails: System shows warnings
- 12a. User rescans: Return to step 3

**Postconditions:**
- Invoice record created
- Document stored with OCR data
- Processing logged in history

---

### UC-02: Review and Correct OCR Results

**Actor:** Finance Officer

**Main Flow:**
1. User views OCR results
2. User identifies field with low confidence
3. User clicks on field to edit
4. User enters correct value
5. System marks field as corrected
6. User repeats for other fields
7. User saves corrected data

---

## 8. Business Rules

### BR-01: Document Size Limits
- Maximum file size: 10 MB
- Maximum pages: 20 per document
- Minimum resolution: 150 DPI

### BR-02: Confidence Thresholds
- High confidence (≥90%): Auto-accept
- Medium confidence (70-89%): Review recommended
- Low confidence (<70%): Manual verification required

### BR-03: Validation Requirements
- All invoices must pass VAT calculation check
- Supplier must exist in supplier database
- PO reference must be valid if provided
- Duplicate invoice numbers rejected

### BR-04: Data Retention
- Original documents retained for 7 years
- OCR metadata retained for 7 years
- Processing logs retained for 3 years

### BR-05: Processing Limits
- Maximum 100 documents per user per day
- Maximum 10 concurrent processing jobs
- Retry failed processing up to 3 times

---

**End of Document**
