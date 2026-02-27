# Software Requirements Specification (SRS)
# Purchase Requisitions Module

**Document Version:** 1.0
**Date:** 27 February 2026
**Project:** GovProcure eProcurement Platform
**Module:** Purchase Requisitions Management

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

This SRS document describes the requirements for the Purchase Requisitions module of the GovProcure eProcurement Platform. This module enables authorized users to create, submit, and track purchase requisitions through an automated approval workflow.

### 1.2 Scope

The Purchase Requisitions module encompasses:

- Requisition creation with item details
- Multi-step form wizard interface
- Budget validation and cost center allocation
- Approval workflow routing
- Status tracking and notifications
- Requisition history and reporting

### 1.3 Definitions

| Term | Definition |
|------|------------|
| Requisition | A formal request to purchase goods or services |
| Cost Center | An organizational unit that incurs costs |
| GL Account | General Ledger account for financial tracking |
| PO | Purchase Order generated from approved requisition |

---

## 2. Overall Description

### 2.1 Product Perspective

```
┌─────────────────────────────────────────────────────────────────┐
│                 PURCHASE REQUISITION WORKFLOW                    │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐     │
│  │  Create  │ → │  Submit  │ → │ Approve  │ → │ Generate │     │
│  │   Req    │   │   Req    │   │   Req    │   │    PO    │     │
│  └──────────┘   └──────────┘   └──────────┘   └──────────┘     │
│       │              │              │              │            │
│       ▼              ▼              ▼              ▼            │
│  ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐     │
│  │  Draft   │   │ Pending  │   │ Approved │   │   PO     │     │
│  │  Saved   │   │ Approval │   │  Status  │   │ Created  │     │
│  └──────────┘   └──────────┘   └──────────┘   └──────────┘     │
└─────────────────────────────────────────────────────────────────┘
```

### 2.2 User Classes

| User Class | Description | Access Level |
|------------|-------------|--------------|
| Requester | Creates and submits requisitions | Create, View own |
| Line Manager | Approves team requisitions | Approve team |
| Budget Holder | Approves within budget authority | Approve by budget |
| CFO | Approves high-value requisitions | Approve all |
| Procurement Officer | Processes approved requisitions | View all, Process |

---

## 3. Functional Requirements

### 3.1 Requisition Creation

#### FR-3.1.1 Create New Requisition
| ID | FR-3.1.1 |
|----|----------|
| **Description** | System shall allow users to create new purchase requisitions |
| **Priority** | High |

**Form Sections:**

**Step 1: Requester Information (Auto-populated)**
- Employee name
- Employee ID
- Department
- Contact details

**Step 2: Item Details**
| Field | Type | Required | Validation |
|-------|------|----------|------------|
| Item Description | Text | Yes | Min 10 chars |
| Procurement Category | Select | Yes | From master list |
| Specification/Part No. | Text | No | - |
| Quantity | Number | Yes | > 0 |
| Unit of Measure | Select | Yes | EA, Box, Litre, Hour |
| Estimated Unit Cost | Currency | Yes | > 0 |
| Estimated Total | Currency | Auto | Qty × Unit Cost |

**Step 3: Delivery & Scheduling**
| Field | Type | Required |
|-------|------|----------|
| Required Delivery Date | Date | Yes |
| Delivery Address | Select | Yes |
| Preferred Supplier | Select | No |
| Priority Level | Select | Yes |

**Step 4: Budget & Authorization**
| Field | Type | Required |
|-------|------|----------|
| Cost Center | Select | Yes |
| GL Account Code | Text | No |
| Business Justification | Textarea | Yes |

#### FR-3.1.2 Save Draft
| ID | FR-3.1.2 |
|----|----------|
| **Description** | System shall allow saving requisition as draft |
| **Priority** | High |

- Draft auto-saved every 60 seconds
- User can manually save at any step
- Drafts retained for 30 days

#### FR-3.1.3 Auto-Generate Reference
| ID | FR-3.1.3 |
|----|----------|
| **Description** | System shall generate unique requisition reference |
| **Format** | REQ-YYYY-NNNN (e.g., REQ-2026-0442) |
| **Priority** | High |

### 3.2 Budget Validation

#### FR-3.2.1 Real-time Budget Check
| ID | FR-3.2.1 |
|----|----------|
| **Description** | System shall validate requisition against available budget |
| **Priority** | High |

**Validation Rules:**
- Check cost center has sufficient budget
- Check GL account is valid and active
- Check total does not exceed single-transaction limit
- Display warning if > 80% of category budget

#### FR-3.2.2 Budget Commitment
| ID | FR-3.2.2 |
|----|----------|
| **Description** | System shall commit budget upon approval |
| **Priority** | High |

- Reduce available budget by requisition amount
- Create budget commitment record
- Release commitment if requisition cancelled

### 3.3 Approval Workflow

#### FR-3.3.1 Workflow Routing
| ID | FR-3.3.1 |
|----|----------|
| **Description** | System shall route requisition based on value and rules |
| **Priority** | High |

**Approval Matrix:**

| Requisition Value | Required Approvers |
|-------------------|-------------------|
| < R 10,000 | Line Manager |
| R 10,000 - R 50,000 | Line Manager → Budget Holder |
| R 50,000 - R 500,000 | Line Manager → Budget Holder → SCM Manager |
| > R 500,000 | Line Manager → Budget Holder → SCM Manager → CFO |

#### FR-3.3.2 Approval Actions
| ID | FR-3.3.2 |
|----|----------|
| **Description** | System shall support approval actions |
| **Priority** | High |

**Actions:**
- **Approve**: Move to next approver or complete
- **Reject**: Return to requester with reason
- **Return for Amendment**: Request changes
- **Delegate**: Assign to alternate approver

#### FR-3.3.3 Approval Timeline
| ID | FR-3.3.3 |
|----|----------|
| **Description** | System shall display approval timeline |
| **Priority** | Medium |

**Timeline Events:**
- Submitted by [User] at [Timestamp]
- Budget Check - Passed/Failed at [Timestamp]
- Approved/Rejected by [User] at [Timestamp]
- PO Generated at [Timestamp]

### 3.4 Notifications

#### FR-3.4.1 Email Notifications
| ID | FR-3.4.1 |
|----|----------|
| **Description** | System shall send email notifications |
| **Priority** | High |

**Notification Triggers:**
| Event | Recipients |
|-------|------------|
| Requisition submitted | Requester, First Approver |
| Pending approval | Current Approver |
| Approved | Requester, Next Approver |
| Rejected | Requester |
| PO Generated | Requester, Procurement |

#### FR-3.4.2 Escalation
| ID | FR-3.4.2 |
|----|----------|
| **Description** | System shall escalate overdue approvals |
| **Priority** | Medium |

- Reminder after 24 hours
- Escalate to supervisor after 48 hours
- Auto-reject after 7 days (configurable)

### 3.5 Requisition Management

#### FR-3.5.1 View Requisitions
| ID | FR-3.5.1 |
|----|----------|
| **Description** | System shall display requisition list |
| **Priority** | High |

**List Columns:**
- Reference number
- Description
- Category
- Estimated total
- Status
- Submitted date
- Current approver

**Filters:**
- Status (Draft, Pending, Approved, Rejected)
- Date range
- Category
- Amount range

#### FR-3.5.2 Search Requisitions
| ID | FR-3.5.2 |
|----|----------|
| **Description** | System shall support requisition search |
| **Priority** | Medium |

**Search Fields:**
- Reference number
- Item description
- Requester name

#### FR-3.5.3 Export Requisitions
| ID | FR-3.5.3 |
|----|----------|
| **Description** | System shall export requisition data |
| **Priority** | Medium |

**Export Formats:**
- CSV
- Excel
- PDF

---

## 4. Non-Functional Requirements

### 4.1 Performance

| ID | Requirement | Target |
|----|-------------|--------|
| NFR-4.1.1 | Form load time | < 2 seconds |
| NFR-4.1.2 | Save draft response | < 500ms |
| NFR-4.1.3 | Budget check response | < 1 second |
| NFR-4.1.4 | Submit response | < 2 seconds |

### 4.2 Usability

| ID | Requirement |
|----|-------------|
| NFR-4.2.1 | Step-by-step wizard interface |
| NFR-4.2.2 | Progress indicator for form completion |
| NFR-4.2.3 | Inline validation with error messages |
| NFR-4.2.4 | Auto-save functionality |
| NFR-4.2.5 | Mobile-responsive design |

---

## 5. User Interface Requirements

### 5.1 Requisition Form Wizard

```
┌─────────────────────────────────────────────────────────────────┐
│ New Purchase Requisition — REQ-2026-442 (Draft)                 │
│ All fields marked * are mandatory                     [Pending] │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  [✓]──────[2]──────[ ]──────[ ]                                 │
│  Requester  Item    Delivery  Review                            │
│   Info    Details     &       & Submit                          │
│                    Schedule                                      │
├─────────────────────────────────────────────────────────────────┤
│ ITEM / SERVICE INFORMATION                                       │
│                                                                  │
│ Item or Service Description *                                    │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Dell Latitude 5540 Laptop Computers                         │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                  │
│ Procurement Category *          Specification / Part No.        │
│ ┌────────────────────────┐     ┌────────────────────────────┐  │
│ │ IT & Technology      ▼ │     │ LAT-5540-I5-16GB           │  │
│ └────────────────────────┘     └────────────────────────────┘  │
│                                                                  │
│ Quantity Required *             Unit of Measure                  │
│ ┌────────────────────────┐     ┌────────────────────────────┐  │
│ │ 10                     │     │ Each (EA)                ▼ │  │
│ └────────────────────────┘     └────────────────────────────┘  │
│                                                                  │
│ Estimated Unit Cost (ZAR) *     Estimated Total (ZAR)           │
│ ┌────────────────────────┐     ┌────────────────────────────┐  │
│ │ 18,500.00              │     │ 185,000.00         (Auto)  │  │
│ └────────────────────────┘     └────────────────────────────┘  │
├─────────────────────────────────────────────────────────────────┤
│ [← Back]                      [Save as Draft] [Next: Delivery →]│
└─────────────────────────────────────────────────────────────────┘
```

### 5.2 Approval Timeline

```
┌─────────────────────────────────────────────────────────────────┐
│ APPROVAL WORKFLOW                                                │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ● Submitted — T. Molefe                                        │
│  │ 26 Feb 2026, 09:14                                           │
│  │                                                               │
│  ● Budget Check — Passed                                        │
│  │ 26 Feb 2026, 09:15 · Automated                               │
│  │                                                               │
│  ◉ Line Manager Approval                                        │
│  │ Awaiting · S. Dlamini                                        │
│  │                                                               │
│  ○ CFO Sign-off (if > R50,000)                                  │
│  │ Not yet reached                                              │
│  │                                                               │
│  ○ Purchase Order Generation                                    │
│    Pending approval                                              │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘

Legend: ● Complete  ◉ In Progress  ○ Pending
```

---

## 6. Data Requirements

### 6.1 Data Model

#### 6.1.1 Requisition Entity

| Field | Type | Description |
|-------|------|-------------|
| requisition_id | VARCHAR(20) | Primary key (REQ-YYYY-NNNN) |
| requester_id | VARCHAR(50) | Employee ID |
| department_id | VARCHAR(20) | Department reference |
| status | ENUM | Draft, Submitted, Pending, Approved, Rejected, Cancelled |
| total_amount | DECIMAL(15,2) | Total estimated value |
| priority | ENUM | Normal, High, Urgent |
| justification | TEXT | Business justification |
| cost_center | VARCHAR(20) | Cost center code |
| gl_account | VARCHAR(20) | GL account code |
| delivery_date | DATE | Required delivery date |
| delivery_address | VARCHAR(255) | Delivery location |
| created_at | TIMESTAMP | Creation timestamp |
| submitted_at | TIMESTAMP | Submission timestamp |
| completed_at | TIMESTAMP | Completion timestamp |

#### 6.1.2 Requisition Line Item Entity

| Field | Type | Description |
|-------|------|-------------|
| line_id | INT | Primary key |
| requisition_id | VARCHAR(20) | Foreign key |
| description | VARCHAR(500) | Item description |
| category | VARCHAR(100) | Procurement category |
| specification | VARCHAR(200) | Part number/spec |
| quantity | DECIMAL(10,2) | Required quantity |
| unit | VARCHAR(20) | Unit of measure |
| unit_price | DECIMAL(15,2) | Estimated unit price |
| line_total | DECIMAL(15,2) | Line total amount |
| preferred_supplier | VARCHAR(50) | Preferred supplier ID |

#### 6.1.3 Requisition Approval Entity

| Field | Type | Description |
|-------|------|-------------|
| approval_id | INT | Primary key |
| requisition_id | VARCHAR(20) | Foreign key |
| approver_id | VARCHAR(50) | Approver employee ID |
| approver_role | VARCHAR(50) | Role in workflow |
| sequence | INT | Approval sequence |
| status | ENUM | Pending, Approved, Rejected, Delegated |
| action_date | TIMESTAMP | Action timestamp |
| comments | TEXT | Approval comments |

---

## 7. Use Cases

### UC-01: Create Purchase Requisition

**Actor:** Requester

**Preconditions:**
- User is logged in
- User has requisition creation permission

**Main Flow:**
1. User clicks "+ New Requisition"
2. System displays requisition wizard (Step 1)
3. System auto-populates requester info
4. User clicks "Next"
5. System displays item details form (Step 2)
6. User enters item information
7. User clicks "Next"
8. System displays delivery form (Step 3)
9. User enters delivery details
10. User clicks "Next"
11. System displays budget form (Step 4)
12. User enters budget information
13. System validates budget availability
14. User enters justification
15. User clicks "Submit"
16. System validates all fields
17. System generates requisition reference
18. System routes to first approver
19. System displays confirmation

**Postconditions:**
- Requisition created with status "Pending"
- First approver notified
- Budget committed

---

### UC-02: Approve Requisition

**Actor:** Approver

**Preconditions:**
- Requisition pending approval
- User is designated approver

**Main Flow:**
1. User receives approval notification
2. User clicks notification link
3. System displays requisition details
4. User reviews item details
5. User reviews budget allocation
6. User reviews justification
7. User clicks "Approve"
8. System prompts for comments (optional)
9. User enters comments
10. User confirms approval
11. System updates approval record
12. System routes to next approver or completes

**Alternative Flows:**
- 7a. User clicks "Reject": Enter reason, notify requester
- 7b. User clicks "Return": Request amendments

---

## 8. Business Rules

### BR-01: Requisition Reference Format
- Format: REQ-YYYY-NNNN
- Year resets counter on 1 January
- Sequential numbering within year

### BR-02: Budget Validation
- Requisition cannot exceed cost center available budget
- Warning displayed if requisition > 80% of remaining budget
- Budget committed upon submission, released upon rejection

### BR-03: Approval Escalation
- Reminder sent after 24 hours
- Escalation after 48 hours
- Auto-cancellation after 7 days (configurable)

### BR-04: Mandatory Fields by Value
- Justification required for all requisitions
- Three quotes required if > R 30,000
- CFO approval required if > R 50,000

### BR-05: Preferred Supplier Selection
- If preferred supplier selected, must be from approved vendor list
- Justification required if selecting non-preferred supplier

---

**End of Document**
