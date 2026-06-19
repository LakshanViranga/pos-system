<template>
  <div class="orders-page">
    <!-- Header Component -->
    <header-component
        :userName="`Logged in as ${loggedUser}`"
        :showBackButton="true"
        @logout="handleLogout"
    />

    <!-- Orders Content -->
    <div class="orders-content">
      <div class="orders-container">

        <!-- Page Header -->
        <div class="page-header">
          <h1>Orders</h1>
          <div class="header-actions">
            <select v-model="filterStatus" class="filter-select">
              <option value="">All Orders</option>
              <option value="pending">Pending</option>
              <option value="completed">Completed</option>
              <option value="cancelled">Cancelled</option>
            </select>
          </div>
        </div>

        <!-- Orders Table -->
        <div class="table-section">
          <div class="table-wrapper">
            <table class="orders-table">
              <thead>
              <tr>
                <th>Order ID</th>
                <th>Date & Time</th>
                <th>Items</th>
                <th>Subtotal</th>
                <th>Service Charge</th>
                <th>Total</th>
                <th>Status</th>
                <th>Actions</th>
              </tr>
              </thead>
              <tbody>
              <tr v-for="order in filteredOrders" :key="order.id" class="order-row">
                <td class="order-id">
                  <strong>#{{ order.id }}</strong>
                </td>
                <td class="date-time">
                  <div>{{ formatDate(order.timestamp) }}</div>
                  <div class="time">{{ formatTime(order.timestamp) }}</div>
                </td>
                <td class="items-count">
                  {{ order.itemCount }} item(s)
                </td>
                <td class="price">
                  {{ formatPrice(order.subtotal) }}
                </td>
                <td class="price">
                  {{ formatPrice(order.serviceCharge) }}
                </td>
                <td class="total-price">
                  <strong>{{ formatPrice(order.total) }}</strong>
                </td>
                <td class="status">
                    <span :class="['status-badge', `status-${order.status}`]">
                      {{ formatStatus(order.status) }}
                    </span>
                </td>
                <td class="actions">
                  <button class="btn-action view" @click="viewOrder(order)" title="View details">
                    <i class="fa fa-eye"></i>
                  </button>
                  <button class="btn-action print" @click="printOrder(order)" title="Print">
                    <i class="fa fa-print"></i>
                  </button>
                  <button v-if="order.status !== 'completed'" class="btn-action complete" @click="completeOrder(order.id)" title="Complete">
                    <i class="fa fa-check"></i>
                  </button>
                  <button class="btn-action delete" @click="deleteOrder(order.id)" title="Delete">
                    <i class="fa fa-trash"></i>
                  </button>
                </td>
              </tr>
              </tbody>
            </table>

            <!-- Empty State -->
            <div v-if="filteredOrders.length === 0" class="empty-state">
              <i class="ti ti-inbox"></i>
              <p>No orders found</p>
            </div>
          </div>
        </div>

        <!-- Summary Stats -->
        <div class="summary-stats">
          <div class="stat-card">
            <label>Total Orders</label>
            <span class="stat-value">{{ orders.length }}</span>
          </div>
          <div class="stat-card">
            <label>Pending</label>
            <span class="stat-value pending">{{ pendingCount }}</span>
          </div>
          <div class="stat-card">
            <label>Completed</label>
            <span class="stat-value success">{{ completedCount }}</span>
          </div>
          <div class="stat-card">
            <label>Total Revenue</label>
            <span class="stat-value">{{ formatPrice(totalRevenue) }}</span>
          </div>
        </div>
      </div>
    </div>

    <!-- Order Details Modal -->
    <div v-if="showModal" class="modal-overlay" @click="closeModal">
      <div class="modal-content" @click.stop>
        <div class="modal-header">
          <h2>Order #{{ selectedOrder.id }}</h2>
          <button class="btn-close" @click="closeModal">
            <i class="ti ti-x"></i>
          </button>
        </div>

        <div class="modal-body">
          <div class="order-info">
            <p><strong>Date:</strong> {{ formatDate(selectedOrder.timestamp) }} {{ formatTime(selectedOrder.timestamp) }}</p>
            <p><strong>Status:</strong> <span :class="['status-badge', `status-${selectedOrder.status}`]">{{ formatStatus(selectedOrder.status) }}</span></p>
          </div>

          <h3>Items</h3>
          <table class="items-detail-table">
            <thead>
            <tr>
              <th>Food</th>
              <th>Qty</th>
              <th>Price</th>
              <th>Total</th>
            </tr>
            </thead>
            <tbody>
            <tr v-for="(item, index) in selectedOrder.items" :key="index">
              <td>{{ item.foodName }}</td>
              <td class="text-center">{{ item.count }}</td>
              <td class="text-right">{{ formatPrice(item.unitPrice) }}</td>
              <td class="text-right"><strong>{{ formatPrice(item.itemTotal) }}</strong></td>
            </tr>
            </tbody>
          </table>

          <div class="modal-summary">
            <div class="summary-row">
              <label>Subtotal</label>
              <span>{{ formatPrice(selectedOrder.subtotal) }}</span>
            </div>
            <div class="summary-row">
              <label>Service Charge ({{ selectedOrder.serviceChargePercentage }}%)</label>
              <span>{{ formatPrice(selectedOrder.serviceCharge) }}</span>
            </div>
            <div class="summary-row final">
              <label>Total</label>
              <span>{{ formatPrice(selectedOrder.total) }}</span>
            </div>
          </div>
        </div>

        <div class="modal-footer">
          <button class="btn-secondary" @click="closeModal">Close</button>
          <button class="btn-primary" @click="printOrder(selectedOrder)">
            <i class="fa fa-print"></i>
            Print
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import HeaderComponent from '../component/Header.vue';
import { dbService } from '../services/db.ts';
import { commonUtils } from "../utils/common.ts";

export default {
  name: 'Orders',
  components: {
    HeaderComponent
  },
  data() {
    return {
      loggedUser: 'Admin User',
      filterStatus: '',
      showModal: false,
      selectedOrder: null,
      orders: [
        {
          id: 1001,
          timestamp: new Date(Date.now() - 3600000),
          itemCount: 3,
          items: [
            { foodName: 'Biryani', count: 2, unitPrice: 250, itemTotal: 500 },
            { foodName: 'Naan', count: 1, unitPrice: 50, itemTotal: 50 }
          ],
          subtotal: 550,
          serviceChargePercentage: 10,
          serviceCharge: 55,
          total: 605,
          status: 'completed'
        },
        {
          id: 1002,
          timestamp: new Date(Date.now() - 1800000),
          itemCount: 2,
          items: [
            { foodName: 'Pizza', count: 1, unitPrice: 400, itemTotal: 400 },
            { foodName: 'Garlic Bread', count: 1, unitPrice: 100, itemTotal: 100 }
          ],
          subtotal: 500,
          serviceChargePercentage: 10,
          serviceCharge: 50,
          total: 550,
          status: 'pending'
        },
        {
          id: 1003,
          timestamp: new Date(Date.now() - 900000),
          itemCount: 4,
          items: [
            { foodName: 'Sushi Roll', count: 2, unitPrice: 300, itemTotal: 600 },
            { foodName: 'Miso Soup', count: 2, unitPrice: 150, itemTotal: 300 }
          ],
          subtotal: 900,
          serviceChargePercentage: 10,
          serviceCharge: 90,
          total: 990,
          status: 'pending'
        }
      ]
    };
  },
  computed: {
    filteredOrders() {
      if (!this.filterStatus) return this.orders;
      return this.orders.filter(order => order.status === this.filterStatus);
    },
    pendingCount() {
      return this.orders.filter(o => o.status === 'pending').length;
    },
    completedCount() {
      return this.orders.filter(o => o.status === 'completed').length;
    },
    totalRevenue() {
      return this.orders.reduce((sum, order) => sum + order.total, 0);
    }
  },
  methods: {
    formatPrice(price) {
      return commonUtils.formatPrice(price);
    },
    formatDate(date) {
      return new Intl.DateTimeFormat('en-US', {
        year: 'numeric',
        month: 'short',
        day: 'numeric'
      }).format(new Date(date));
    },
    formatTime(date) {
      return new Intl.DateTimeFormat('en-US', {
        hour: '2-digit',
        minute: '2-digit'
      }).format(new Date(date));
    },
    formatStatus(status) {
      return status.charAt(0).toUpperCase() + status.slice(1);
    },
    viewOrder(order) {
      this.selectedOrder = order;
      this.showModal = true;
    },
    closeModal() {
      this.showModal = false;
      this.selectedOrder = null;
    },
    printOrder(order) {
      alert(`Printing order #${order.id}...\n\nOrder Details:\n${order.itemCount} items\nTotal: ${this.formatPrice(order.total)}`);
      // In production, use actual print functionality
    },
    completeOrder(orderId) {
      const order = this.orders.find(o => o.id === orderId);
      if (order) {
        order.status = 'completed';
      }
    },
    deleteOrder(orderId) {
      if (confirm('Are you sure you want to delete this order?')) {
        const index = this.orders.findIndex(o => o.id === orderId);
        if (index > -1) {
          this.orders.splice(index, 1);
        }
      }
    },
    handleLogout() {
      this.$emit('logout');
    },
    async getAllOrders() {
      return await dbService.getAllOrders();
    }
  },
  async mounted() {
    const orders = await this.getAllOrders();
    console.log('Order', orders);
  }
};
</script>

<style scoped>
.orders-page {
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden;
  background: var(--color-background-tertiary);
}

.orders-content {
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.orders-container {
  display: flex;
  flex-direction: column;
  flex: 1;
  overflow: hidden;
  padding: 1.5rem;
  max-width: 1400px;
  margin: 0 auto;
  width: 100%;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1.5rem;
}

.page-header h1 {
  margin: 0;
  font-size: 28px;
  font-weight: 500;
  color: var(--color-text-primary);
}

.header-actions {
  display: flex;
  gap: 1rem;
}

.filter-select {
  padding: 0.6rem 1rem;
  border: 1px solid var(--color-border-tertiary);
  border-radius: var(--border-radius-md);
  background: var(--color-background-primary);
  color: var(--color-text-primary);
  font-size: 14px;
  cursor: pointer;
  transition: border-color var(--transition-fast);
}

.filter-select:focus {
  outline: none;
  border-color: var(--color-info);
}

/* === TABLE SECTION === */
.table-section {
  flex: 1;
  background: var(--color-background-primary);
  border-radius: var(--border-radius-lg);
  border: 1px solid var(--color-border-tertiary);
  padding: 1rem;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  margin-bottom: 1.5rem;
}

.table-wrapper {
  flex: 1;
  overflow-y: auto;
  overflow-x: hidden;
  border-radius: 4px;
}

.orders-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 13px;
}

.orders-table thead {
  position: sticky;
  top: 0;
  background: var(--color-background-secondary);
  border-bottom: 1px solid var(--color-border-tertiary);
  z-index: 10;
}

.orders-table th {
  padding: 0.75rem;
  text-align: left;
  font-weight: 500;
  color: var(--color-text-secondary);
}

.orders-table td {
  padding: 0.75rem;
  border-bottom: 1px solid var(--color-border-tertiary);
  vertical-align: middle;
}

.orders-table tbody tr:hover {
  background: var(--color-background-secondary);
}

.order-id {
  font-family: monospace;
  color: var(--color-info);
}

.date-time {
  font-size: 12px;
}

.time {
  color: var(--color-text-secondary);
  font-size: 11px;
}

.items-count {
  color: var(--color-text-secondary);
}

.price {
  text-align: right;
  font-family: monospace;
  color: var(--color-text-primary);
}

.total-price {
  text-align: right;
  font-family: monospace;
  color: var(--color-success);
}

.status {
  text-align: center;
}

.status-badge {
  display: inline-block;
  padding: 0.4rem 0.8rem;
  border-radius: 20px;
  font-size: 11px;
  font-weight: 500;
}

.status-pending {
  background: #fef3c7;
  color: #d97706;
}

.status-completed {
  background: #dcfce7;
  color: #16a34a;
}

.status-cancelled {
  background: #fee2e2;
  color: #dc2626;
}

.actions {
  display: flex;
  gap: 0.4rem;
  justify-content: center;
}

.btn-action {
  padding: 0.4rem 0.5rem;
  border: 1px solid var(--color-border-tertiary);
  background: transparent;
  color: var(--color-text-secondary);
  border-radius: 4px;
  cursor: pointer;
  transition: all var(--transition-fast);
  display: inline-flex;
  align-items: center;
  justify-content: center;
  font-size: 14px;
}

.btn-action:hover {
  border-color: var(--color-info);
  color: var(--color-info);
  background: rgba(59, 130, 246, 0.05);
}

.btn-action.complete:hover {
  border-color: var(--color-success);
  color: var(--color-success);
  background: rgba(34, 197, 94, 0.05);
}

.btn-action.delete:hover {
  border-color: var(--color-danger);
  color: var(--color-danger);
  background: rgba(239, 68, 68, 0.05);
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 2rem;
  color: var(--color-text-secondary);
}

.empty-state i {
  font-size: 48px;
  margin-bottom: 1rem;
  opacity: 0.5;
}

/* === SUMMARY STATS === */
.summary-stats {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1rem;
}

.stat-card {
  background: var(--color-background-primary);
  border: 1px solid var(--color-border-tertiary);
  border-radius: var(--border-radius-lg);
  padding: 1.25rem;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.stat-card label {
  font-size: 13px;
  color: var(--color-text-secondary);
  font-weight: 400;
}

.stat-value {
  font-size: 24px;
  font-weight: 500;
  color: var(--color-text-primary);
}

.stat-value.pending {
  color: #d97706;
}

.stat-value.success {
  color: var(--color-success);
}

/* === MODAL === */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-content {
  background: var(--color-background-primary);
  border-radius: var(--border-radius-lg);
  width: 90%;
  max-width: 600px;
  max-height: 90vh;
  overflow-y: auto;
  box-shadow: 0 10px 40px rgba(0, 0, 0, 0.2);
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1.5rem;
  border-bottom: 1px solid var(--color-border-tertiary);
}

.modal-header h2 {
  margin: 0;
  font-size: 20px;
  color: var(--color-text-primary);
}

.btn-close {
  padding: 0.4rem;
  border: none;
  background: transparent;
  color: var(--color-text-secondary);
  cursor: pointer;
  font-size: 20px;
  transition: color var(--transition-fast);
}

.btn-close:hover {
  color: var(--color-text-primary);
}

.modal-body {
  padding: 1.5rem;
}

.order-info {
  margin-bottom: 1.5rem;
  padding-bottom: 1.5rem;
  border-bottom: 1px solid var(--color-border-tertiary);
}

.order-info p {
  margin: 0.5rem 0;
  font-size: 14px;
}

.modal-body h3 {
  margin: 1rem 0 0.75rem 0;
  font-size: 16px;
  color: var(--color-text-primary);
}

.items-detail-table {
  width: 100%;
  border-collapse: collapse;
  margin-bottom: 1.5rem;
  font-size: 13px;
}

.items-detail-table th {
  padding: 0.6rem;
  background: var(--color-background-secondary);
  border-bottom: 1px solid var(--color-border-tertiary);
  font-weight: 500;
  text-align: left;
  color: var(--color-text-secondary);
}

.items-detail-table td {
  padding: 0.6rem;
  border-bottom: 1px solid var(--color-border-tertiary);
}

.text-center {
  text-align: center;
}

.text-right {
  text-align: right;
}

.modal-summary {
  background: var(--color-background-secondary);
  border-radius: 4px;
  padding: 1rem;
  margin-bottom: 1.5rem;
}

.summary-row {
  display: flex;
  justify-content: space-between;
  padding: 0.5rem 0;
  font-size: 13px;
}

.summary-row.final {
  border-top: 1px solid var(--color-border-tertiary);
  padding-top: 0.75rem;
  margin-top: 0.75rem;
  font-weight: 500;
  font-size: 14px;
}

.modal-footer {
  display: flex;
  gap: 1rem;
  padding: 1.5rem;
  border-top: 1px solid var(--color-border-tertiary);
  justify-content: flex-end;
}

.btn-secondary {
  padding: 0.6rem 1.5rem;
  border: 1px solid var(--color-border-tertiary);
  background: transparent;
  color: var(--color-text-primary);
  border-radius: var(--border-radius-md);
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: all var(--transition-fast);
}

.btn-secondary:hover {
  background: var(--color-background-secondary);
}

.btn-primary {
  padding: 0.6rem 1.5rem;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  border-radius: var(--border-radius-md);
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: all var(--transition-fast);
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.btn-primary:hover {
  transform: translateY(-1px);
}

/* === RESPONSIVE === */
@media (max-width: 1024px) {
  .orders-container {
    padding: 1rem;
  }

  .orders-table {
    font-size: 12px;
  }

  .orders-table th,
  .orders-table td {
    padding: 0.6rem;
  }

  .btn-action {
    padding: 0.3rem 0.4rem;
    font-size: 12px;
  }
}

@media (max-width: 768px) {
  .page-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 1rem;
  }

  .orders-table {
    font-size: 11px;
  }

  .orders-table th,
  .orders-table td {
    padding: 0.5rem;
  }

  .modal-content {
    width: 95%;
  }

  .summary-stats {
    grid-template-columns: repeat(2, 1fr);
  }
}
</style>
