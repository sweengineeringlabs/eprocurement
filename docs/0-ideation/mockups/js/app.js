/**
 * SARS eProcurement Mockup - Interactive Functionality
 * BRS RFP 33/2025 - UI/UX Demonstration
 */

// Toast Notification System
const Toast = {
  show(message, type = 'info', duration = 3000) {
    const container = document.getElementById('toast-container') || this.createContainer();
    const toast = document.createElement('div');
    toast.className = `toast toast-${type}`;
    toast.innerHTML = `
      <span class="toast-icon">${this.getIcon(type)}</span>
      <span class="toast-message">${message}</span>
      <button class="toast-close" onclick="this.parentElement.remove()">&times;</button>
    `;
    container.appendChild(toast);
    setTimeout(() => toast.classList.add('show'), 10);
    setTimeout(() => {
      toast.classList.remove('show');
      setTimeout(() => toast.remove(), 300);
    }, duration);
  },

  createContainer() {
    const container = document.createElement('div');
    container.id = 'toast-container';
    document.body.appendChild(container);
    return container;
  },

  getIcon(type) {
    const icons = {
      success: '✓',
      error: '✕',
      warning: '⚠',
      info: 'ℹ'
    };
    return icons[type] || icons.info;
  }
};

// Modal System
const Modal = {
  show(options) {
    const { title, content, actions, size = 'medium' } = options;
    const overlay = document.createElement('div');
    overlay.className = 'modal-overlay';
    overlay.innerHTML = `
      <div class="modal modal-${size}">
        <div class="modal-header">
          <h3 class="modal-title">${title}</h3>
          <button class="modal-close" onclick="Modal.close()">&times;</button>
        </div>
        <div class="modal-body">${content}</div>
        ${actions ? `<div class="modal-footer">${actions}</div>` : ''}
      </div>
    `;
    overlay.addEventListener('click', (e) => {
      if (e.target === overlay) Modal.close();
    });
    document.body.appendChild(overlay);
    setTimeout(() => overlay.classList.add('show'), 10);
    document.body.style.overflow = 'hidden';
  },

  close() {
    const overlay = document.querySelector('.modal-overlay');
    if (overlay) {
      overlay.classList.remove('show');
      setTimeout(() => {
        overlay.remove();
        document.body.style.overflow = '';
      }, 300);
    }
  },

  confirm(message, onConfirm, onCancel) {
    this.show({
      title: 'Confirm Action',
      content: `<p>${message}</p>`,
      actions: `
        <button class="btn btn-secondary" onclick="Modal.close(); ${onCancel || ''}">Cancel</button>
        <button class="btn btn-primary" onclick="Modal.close(); ${onConfirm}">Confirm</button>
      `
    });
  }
};

// Loading Indicator
const Loading = {
  show(message = 'Processing...') {
    const overlay = document.createElement('div');
    overlay.id = 'loading-overlay';
    overlay.innerHTML = `
      <div class="loading-spinner"></div>
      <div class="loading-message">${message}</div>
    `;
    document.body.appendChild(overlay);
  },

  hide() {
    const overlay = document.getElementById('loading-overlay');
    if (overlay) overlay.remove();
  }
};

// Tab Functionality
function initTabs() {
  document.querySelectorAll('.tabs').forEach(tabContainer => {
    tabContainer.querySelectorAll('.tab').forEach(tab => {
      tab.addEventListener('click', function() {
        tabContainer.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
        this.classList.add('active');
        Toast.show(`Showing: ${this.textContent.trim()}`, 'info', 1500);
      });
    });
  });
}

// Form Actions
const FormActions = {
  saveDraft() {
    Loading.show('Saving draft...');
    setTimeout(() => {
      Loading.hide();
      Toast.show('Draft saved successfully!', 'success');
    }, 1000);
  },

  submit(formName) {
    Modal.confirm(
      `Are you sure you want to submit this ${formName}?`,
      `FormActions.doSubmit('${formName}')`
    );
  },

  doSubmit(formName) {
    Loading.show('Submitting...');
    setTimeout(() => {
      Loading.hide();
      Toast.show(`${formName} submitted successfully!`, 'success');
    }, 1500);
  },

  approve(itemType, itemId) {
    Modal.show({
      title: `Approve ${itemType}`,
      content: `
        <p>You are about to approve <strong>${itemId}</strong>.</p>
        <div class="form-group" style="margin-top: 16px;">
          <label class="form-label">Comments (Optional)</label>
          <textarea class="form-input" id="approve-comments" rows="3" placeholder="Add approval comments..."></textarea>
        </div>
      `,
      actions: `
        <button class="btn btn-secondary" onclick="Modal.close()">Cancel</button>
        <button class="btn btn-success" onclick="FormActions.doApprove('${itemType}', '${itemId}')">Approve</button>
      `
    });
  },

  doApprove(itemType, itemId) {
    Modal.close();
    Loading.show('Processing approval...');
    setTimeout(() => {
      Loading.hide();
      Toast.show(`${itemType} ${itemId} approved successfully!`, 'success');
    }, 1000);
  },

  reject(itemType, itemId) {
    Modal.show({
      title: `Reject ${itemType}`,
      content: `
        <p>You are about to reject <strong>${itemId}</strong>.</p>
        <div class="form-group" style="margin-top: 16px;">
          <label class="form-label required">Reason for Rejection</label>
          <textarea class="form-input" id="reject-reason" rows="3" placeholder="Please provide a reason..." required></textarea>
        </div>
      `,
      actions: `
        <button class="btn btn-secondary" onclick="Modal.close()">Cancel</button>
        <button class="btn btn-accent" style="background: #dc2626;" onclick="FormActions.doReject('${itemType}', '${itemId}')">Reject</button>
      `
    });
  },

  doReject(itemType, itemId) {
    const reason = document.getElementById('reject-reason')?.value;
    if (!reason) {
      Toast.show('Please provide a reason for rejection', 'error');
      return;
    }
    Modal.close();
    Loading.show('Processing...');
    setTimeout(() => {
      Loading.hide();
      Toast.show(`${itemType} ${itemId} has been rejected.`, 'warning');
    }, 1000);
  },

  delete(itemType, itemId) {
    Modal.confirm(
      `Are you sure you want to delete ${itemType} <strong>${itemId}</strong>? This action cannot be undone.`,
      `FormActions.doDelete('${itemType}', '${itemId}')`
    );
  },

  doDelete(itemType, itemId) {
    Loading.show('Deleting...');
    setTimeout(() => {
      Loading.hide();
      Toast.show(`${itemType} ${itemId} has been deleted.`, 'success');
    }, 1000);
  }
};

// Procurement Actions
const ProcurementActions = {
  createRequisition() {
    window.location.href = window.location.href.includes('modules/')
      ? 'requisitions/create.html'
      : 'modules/requisitions/create.html';
  },

  createTender() {
    window.location.href = window.location.href.includes('modules/')
      ? 'tenders/create.html'
      : 'modules/tenders/create.html';
  },

  viewDetails(type, id) {
    Modal.show({
      title: `${type} Details: ${id}`,
      content: `
        <div style="padding: 16px;">
          <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 16px;">
            <div>
              <div style="font-size: 12px; color: var(--text-muted);">Reference</div>
              <div style="font-weight: 500;">${id}</div>
            </div>
            <div>
              <div style="font-size: 12px; color: var(--text-muted);">Status</div>
              <span class="status status-pending">Pending</span>
            </div>
            <div>
              <div style="font-size: 12px; color: var(--text-muted);">Created</div>
              <div style="font-weight: 500;">25 Mar 2025</div>
            </div>
            <div>
              <div style="font-size: 12px; color: var(--text-muted);">Created By</div>
              <div style="font-weight: 500;">Thabo Molefe</div>
            </div>
          </div>
          <div style="margin-top: 16px; padding-top: 16px; border-top: 1px solid var(--border);">
            <div style="font-size: 12px; color: var(--text-muted);">Description</div>
            <p style="margin-top: 4px;">This is a detailed view of the ${type.toLowerCase()} record. In the full implementation, this would show all relevant information.</p>
          </div>
        </div>
      `,
      actions: `
        <button class="btn btn-secondary" onclick="Modal.close()">Close</button>
        <button class="btn btn-primary" onclick="Toast.show('Opening full details...', 'info'); Modal.close();">View Full Details</button>
      `,
      size: 'medium'
    });
  },

  submitBid(tenderId) {
    Modal.show({
      title: `Submit Bid - ${tenderId}`,
      content: `
        <div style="padding: 16px;">
          <div class="notice-bar warning" style="margin-bottom: 16px;">
            <span>⚠</span>
            <span>Ensure all required documents are uploaded before submission.</span>
          </div>
          <div class="form-group">
            <label class="form-label required">Bid Amount (ZAR)</label>
            <input type="text" class="form-input" placeholder="Enter your bid amount" style="font-family: 'IBM Plex Mono';">
          </div>
          <div class="form-group">
            <label class="form-label required">Validity Period</label>
            <select class="form-input">
              <option>90 Days</option>
              <option>120 Days</option>
              <option>180 Days</option>
            </select>
          </div>
          <div class="form-group">
            <label class="form-label">Upload Bid Documents</label>
            <div style="border: 2px dashed var(--border); border-radius: var(--radius); padding: 24px; text-align: center; cursor: pointer;">
              <div style="color: var(--text-muted);">Click or drag files to upload</div>
            </div>
          </div>
        </div>
      `,
      actions: `
        <button class="btn btn-secondary" onclick="Modal.close()">Cancel</button>
        <button class="btn btn-primary" onclick="ProcurementActions.doSubmitBid('${tenderId}')">Submit Bid</button>
      `,
      size: 'large'
    });
  },

  doSubmitBid(tenderId) {
    Modal.close();
    Loading.show('Submitting bid...');
    setTimeout(() => {
      Loading.hide();
      Toast.show(`Bid for ${tenderId} submitted successfully!`, 'success');
    }, 2000);
  },

  startEvaluation(tenderId) {
    Modal.show({
      title: 'Conflict of Interest Declaration',
      content: `
        <div style="padding: 16px;">
          <p style="margin-bottom: 16px;">Before proceeding with the evaluation of <strong>${tenderId}</strong>, you must declare any potential conflicts of interest.</p>
          <div class="form-group">
            <label style="display: flex; align-items: flex-start; gap: 12px; cursor: pointer;">
              <input type="checkbox" id="coi-none" style="margin-top: 4px;">
              <span>I declare that I have <strong>NO</strong> conflict of interest with any of the bidders for this tender.</span>
            </label>
          </div>
          <div class="form-group">
            <label style="display: flex; align-items: flex-start; gap: 12px; cursor: pointer;">
              <input type="checkbox" id="coi-declare" style="margin-top: 4px;">
              <span>I wish to <strong>DECLARE</strong> a potential conflict of interest.</span>
            </label>
          </div>
          <div class="form-group" id="coi-details" style="display: none;">
            <label class="form-label">Please describe the conflict</label>
            <textarea class="form-input" rows="3" placeholder="Describe the nature of the conflict..."></textarea>
          </div>
        </div>
      `,
      actions: `
        <button class="btn btn-secondary" onclick="Modal.close()">Cancel</button>
        <button class="btn btn-primary" onclick="ProcurementActions.confirmCOI('${tenderId}')">Proceed to Evaluation</button>
      `
    });

    // Add toggle functionality after modal is shown
    setTimeout(() => {
      document.getElementById('coi-declare')?.addEventListener('change', function() {
        document.getElementById('coi-details').style.display = this.checked ? 'block' : 'none';
      });
    }, 100);
  },

  confirmCOI(tenderId) {
    const noCOI = document.getElementById('coi-none')?.checked;
    const declareCOI = document.getElementById('coi-declare')?.checked;

    if (!noCOI && !declareCOI) {
      Toast.show('Please complete the COI declaration', 'error');
      return;
    }

    Modal.close();
    Toast.show('COI declaration recorded. Proceeding to evaluation...', 'success');
    setTimeout(() => {
      window.location.href = 'scoring.html';
    }, 1500);
  },

  publishTender(tenderId) {
    Modal.confirm(
      `Publish tender <strong>${tenderId}</strong> to eTender Portal and SARS website?`,
      `ProcurementActions.doPublish('${tenderId}')`
    );
  },

  doPublish(tenderId) {
    Loading.show('Publishing to portals...');
    setTimeout(() => {
      Loading.hide();
      Toast.show(`${tenderId} published successfully to all portals!`, 'success');
    }, 2000);
  },

  generateContract(tenderId) {
    Loading.show('Generating contract from award...');
    setTimeout(() => {
      Loading.hide();
      Toast.show('Contract generated. Redirecting to contract editor...', 'success');
      setTimeout(() => {
        window.location.href = window.location.href.includes('modules/')
          ? '../contracts/create.html'
          : 'modules/contracts/create.html';
      }, 1000);
    }, 1500);
  },

  sendPO(poId) {
    Modal.confirm(
      `Send Purchase Order <strong>${poId}</strong> to supplier via email?`,
      `ProcurementActions.doSendPO('${poId}')`
    );
  },

  doSendPO(poId) {
    Loading.show('Sending to supplier...');
    setTimeout(() => {
      Loading.hide();
      Toast.show(`${poId} sent to supplier successfully!`, 'success');
    }, 1500);
  },

  receiveGoods(poId) {
    Modal.show({
      title: `Receive Goods - ${poId}`,
      content: `
        <div style="padding: 16px;">
          <table class="data-table" style="font-size: 13px;">
            <thead>
              <tr>
                <th>Item</th>
                <th style="width: 80px;">Ordered</th>
                <th style="width: 80px;">Received</th>
                <th style="width: 100px;">Status</th>
              </tr>
            </thead>
            <tbody>
              <tr>
                <td>Dell PowerEdge R750 Server</td>
                <td>5</td>
                <td><input type="number" class="form-input" value="5" style="width: 60px;"></td>
                <td><span class="status status-approved">OK</span></td>
              </tr>
              <tr>
                <td>NetApp AFF A400 Storage</td>
                <td>2</td>
                <td><input type="number" class="form-input" value="2" style="width: 60px;"></td>
                <td><span class="status status-approved">OK</span></td>
              </tr>
              <tr>
                <td>Cisco Catalyst 9300 Switch</td>
                <td>5</td>
                <td><input type="number" class="form-input" value="3" style="width: 60px;"></td>
                <td><span class="status status-pending">Partial</span></td>
              </tr>
            </tbody>
          </table>
          <div class="form-group" style="margin-top: 16px;">
            <label class="form-label">Delivery Note Number</label>
            <input type="text" class="form-input" placeholder="Enter delivery note number">
          </div>
          <div class="form-group">
            <label class="form-label">Notes</label>
            <textarea class="form-input" rows="2" placeholder="Any delivery notes or issues..."></textarea>
          </div>
        </div>
      `,
      actions: `
        <button class="btn btn-secondary" onclick="Modal.close()">Cancel</button>
        <button class="btn btn-success" onclick="ProcurementActions.doReceiveGoods('${poId}')">Confirm Receipt</button>
      `,
      size: 'large'
    });
  },

  doReceiveGoods(poId) {
    Modal.close();
    Loading.show('Recording goods receipt...');
    setTimeout(() => {
      Loading.hide();
      Toast.show(`Goods receipt for ${poId} recorded. 3-way match initiated.`, 'success');
    }, 1500);
  }
};

// Supplier Actions
const SupplierActions = {
  viewProfile(supplierId) {
    Modal.show({
      title: 'Supplier Profile',
      content: `
        <div style="padding: 16px;">
          <div style="display: flex; gap: 16px; margin-bottom: 20px;">
            <div style="width: 64px; height: 64px; background: var(--blue); border-radius: 50%; display: flex; align-items: center; justify-content: center; color: white; font-weight: 700; font-size: 24px;">TS</div>
            <div>
              <div style="font-size: 18px; font-weight: 600;">TechSolutions SA (Pty) Ltd</div>
              <div style="color: var(--text-muted);">CSD: ${supplierId}</div>
              <div style="margin-top: 4px;">
                <span class="tag" style="background: var(--green); color: white;">Level 2 B-BBEE</span>
                <span class="status status-approved" style="margin-left: 8px;">CSD Verified</span>
              </div>
            </div>
          </div>
          <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 16px;">
            <div>
              <div style="font-size: 12px; color: var(--text-muted);">Tax Clearance</div>
              <span class="status status-approved">Valid until Nov 2025</span>
            </div>
            <div>
              <div style="font-size: 12px; color: var(--text-muted);">Performance Rating</div>
              <div style="color: var(--accent);">★★★★★ 4.8/5.0</div>
            </div>
            <div>
              <div style="font-size: 12px; color: var(--text-muted);">Active Contracts</div>
              <div style="font-weight: 500;">5 contracts (R 145M)</div>
            </div>
            <div>
              <div style="font-size: 12px; color: var(--text-muted);">Total Spend (YTD)</div>
              <div style="font-weight: 500; font-family: 'IBM Plex Mono';">R 45,200,000</div>
            </div>
          </div>
        </div>
      `,
      actions: `
        <button class="btn btn-secondary" onclick="Modal.close()">Close</button>
        <button class="btn btn-primary" onclick="Toast.show('Opening full profile...', 'info'); Modal.close();">View Full Profile</button>
      `,
      size: 'medium'
    });
  },

  inviteToTender(supplierId) {
    Modal.show({
      title: 'Invite Supplier to Tender',
      content: `
        <div style="padding: 16px;">
          <p style="margin-bottom: 16px;">Select a tender to invite <strong>${supplierId}</strong>:</p>
          <div class="form-group">
            <label class="form-label required">Select Tender</label>
            <select class="form-input">
              <option value="">-- Select Tender --</option>
              <option>RFP-2025-0089 - IT Infrastructure Modernization</option>
              <option>RFQ-2025-0198 - Network Equipment Supply</option>
              <option>TND-2025-0045 - Data Center Cooling</option>
            </select>
          </div>
          <div class="form-group">
            <label class="form-label">Personal Message</label>
            <textarea class="form-input" rows="3" placeholder="Add a personal message to the invitation..."></textarea>
          </div>
        </div>
      `,
      actions: `
        <button class="btn btn-secondary" onclick="Modal.close()">Cancel</button>
        <button class="btn btn-primary" onclick="SupplierActions.doInvite('${supplierId}')">Send Invitation</button>
      `
    });
  },

  doInvite(supplierId) {
    Modal.close();
    Loading.show('Sending invitation...');
    setTimeout(() => {
      Loading.hide();
      Toast.show(`Invitation sent to ${supplierId}`, 'success');
    }, 1000);
  },

  verifyCSD(supplierId) {
    Loading.show('Verifying with CSD...');
    setTimeout(() => {
      Loading.hide();
      Toast.show(`CSD verification complete for ${supplierId}. Status: Valid`, 'success');
    }, 2000);
  }
};

// Report Actions
const ReportActions = {
  export(reportName, format = 'PDF') {
    Loading.show(`Generating ${format} report...`);
    setTimeout(() => {
      Loading.hide();
      Toast.show(`${reportName} exported as ${format}`, 'success');
    }, 2000);
  },

  generate(reportType) {
    Modal.show({
      title: `Generate ${reportType} Report`,
      content: `
        <div style="padding: 16px;">
          <div class="form-group">
            <label class="form-label required">Date Range</label>
            <div style="display: flex; gap: 8px;">
              <input type="date" class="form-input" value="2025-01-01">
              <span style="align-self: center;">to</span>
              <input type="date" class="form-input" value="2025-03-25">
            </div>
          </div>
          <div class="form-group">
            <label class="form-label">Format</label>
            <select class="form-input">
              <option>PDF</option>
              <option>Excel</option>
              <option>CSV</option>
            </select>
          </div>
          <div class="form-group">
            <label class="form-label">Include Sections</label>
            <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 8px;">
              <label style="display: flex; align-items: center; gap: 8px;"><input type="checkbox" checked> Summary</label>
              <label style="display: flex; align-items: center; gap: 8px;"><input type="checkbox" checked> Charts</label>
              <label style="display: flex; align-items: center; gap: 8px;"><input type="checkbox" checked> Details</label>
              <label style="display: flex; align-items: center; gap: 8px;"><input type="checkbox"> Appendix</label>
            </div>
          </div>
        </div>
      `,
      actions: `
        <button class="btn btn-secondary" onclick="Modal.close()">Cancel</button>
        <button class="btn btn-primary" onclick="ReportActions.doGenerate('${reportType}')">Generate</button>
      `
    });
  },

  doGenerate(reportType) {
    Modal.close();
    Loading.show('Generating report...');
    setTimeout(() => {
      Loading.hide();
      Toast.show(`${reportType} report generated successfully!`, 'success');
    }, 3000);
  }
};

// Stepper Navigation
const Stepper = {
  current: 1,
  total: 6,

  init(totalSteps) {
    this.total = totalSteps;
    this.updateUI();
  },

  next() {
    if (this.current < this.total) {
      this.current++;
      this.updateUI();
      Toast.show(`Step ${this.current} of ${this.total}`, 'info', 1500);
    }
  },

  prev() {
    if (this.current > 1) {
      this.current--;
      this.updateUI();
      Toast.show(`Step ${this.current} of ${this.total}`, 'info', 1500);
    }
  },

  goTo(step) {
    if (step >= 1 && step <= this.total) {
      this.current = step;
      this.updateUI();
    }
  },

  updateUI() {
    document.querySelectorAll('.step').forEach((step, index) => {
      step.classList.remove('active', 'completed');
      if (index + 1 < this.current) {
        step.classList.add('completed');
      } else if (index + 1 === this.current) {
        step.classList.add('active');
      }
    });
  }
};

// Cart/Basket for Catalogue
const Cart = {
  items: [],

  add(item) {
    this.items.push(item);
    Toast.show(`${item} added to cart`, 'success');
    this.updateBadge();
  },

  updateBadge() {
    const badge = document.getElementById('cart-badge');
    if (badge) {
      badge.textContent = this.items.length;
      badge.style.display = this.items.length > 0 ? 'flex' : 'none';
    }
  },

  view() {
    if (this.items.length === 0) {
      Toast.show('Cart is empty', 'info');
      return;
    }
    Modal.show({
      title: `Shopping Cart (${this.items.length} items)`,
      content: `
        <div style="padding: 16px;">
          <ul style="list-style: none; padding: 0; margin: 0;">
            ${this.items.map(item => `<li style="padding: 8px 0; border-bottom: 1px solid var(--border);">${item}</li>`).join('')}
          </ul>
        </div>
      `,
      actions: `
        <button class="btn btn-secondary" onclick="Cart.clear(); Modal.close();">Clear Cart</button>
        <button class="btn btn-primary" onclick="Cart.checkout()">Create Requisition</button>
      `
    });
  },

  clear() {
    this.items = [];
    this.updateBadge();
    Toast.show('Cart cleared', 'info');
  },

  checkout() {
    Modal.close();
    Loading.show('Creating requisition from cart...');
    setTimeout(() => {
      Loading.hide();
      this.clear();
      Toast.show('Requisition created from cart items!', 'success');
    }, 1500);
  }
};

// Initialize on page load
document.addEventListener('DOMContentLoaded', function() {
  initTabs();

  // Make buttons clickable
  document.querySelectorAll('.btn').forEach(btn => {
    if (!btn.onclick && !btn.hasAttribute('href')) {
      btn.addEventListener('click', function(e) {
        const text = this.textContent.trim();

        // Handle specific button actions
        if (text.includes('Save Draft') || text.includes('Save as Draft')) {
          FormActions.saveDraft();
        } else if (text.includes('Export')) {
          ReportActions.export('Report');
        } else if (text.includes('Add to Cart')) {
          const card = this.closest('.card');
          const itemName = card?.querySelector('.card-title, [style*="font-weight: 600"]')?.textContent || 'Item';
          Cart.add(itemName);
        } else if (text.includes('Submit')) {
          FormActions.submit('Form');
        } else if (text.includes('Approve')) {
          FormActions.approve('Item', 'REF-001');
        } else if (text.includes('Reject')) {
          FormActions.reject('Item', 'REF-001');
        } else if (text.includes('View') && !text.includes('View All')) {
          ProcurementActions.viewDetails('Item', 'REF-001');
        } else if (text.includes('Generate')) {
          ReportActions.generate('Custom');
        } else if (!this.disabled) {
          Toast.show(`Action: ${text}`, 'info', 1500);
        }
      });
    }
  });

  // Handle stepper navigation
  const prevBtn = document.querySelector('[onclick*="prev"], .btn:contains("Previous")');
  const nextBtn = document.querySelector('[onclick*="next"], .btn:contains("Continue"), .btn:contains("Next")');

  document.querySelectorAll('.stepper').forEach(stepper => {
    const steps = stepper.querySelectorAll('.step');
    Stepper.init(steps.length);

    steps.forEach((step, index) => {
      step.style.cursor = 'pointer';
      step.addEventListener('click', () => Stepper.goTo(index + 1));
    });
  });
});

// Helper function for search
function handleSearch(query) {
  if (query.length > 0) {
    Toast.show(`Searching for: ${query}`, 'info', 1500);
  }
}

// Expose functions globally
window.Toast = Toast;
window.Modal = Modal;
window.Loading = Loading;
window.FormActions = FormActions;
window.ProcurementActions = ProcurementActions;
window.SupplierActions = SupplierActions;
window.ReportActions = ReportActions;
window.Stepper = Stepper;
window.Cart = Cart;
