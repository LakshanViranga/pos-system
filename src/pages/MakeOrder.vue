<template>
  <div class="make-order-page">
    <!-- Header Component -->
    <header-component
        :userName="`Logged in as ${loggedUser}`"
        :showBackButton="true"
        @logout="handleLogout"
    />

    <!-- Order Content -->
    <div class="order-content">
      <div class="order-layout">

        <!-- LEFT: Items Section -->
        <div class="items-section">
          <div class="section-header">
            <h2 class="section-title">Order items</h2>
            <p class="order-id">Order #{{ orderId }}</p>
          </div>

          <div class="table-wrapper">
            <table class="items-table">
              <thead>
              <tr>
                <th>Food</th>
                <th>Qty</th>
                <th>Price</th>
                <th>Total</th>
                <th></th>
              </tr>
              </thead>
              <tbody>
              <tr v-for="(item, index) in orderItems" :key="index" class="item-row">
                <td>
                  <input
                      v-model="item.foodName"
                      type="text"
                      class="input-field"
                      placeholder="Food"
                  />
                </td>
                <td>
                  <input
                      v-model.number="item.count"
                      type="number"
                      class="input-field small"
                      placeholder="0"
                      @input="calculateItemTotal(index)"
                  />
                </td>
                <td>
                  <input
                      v-model.number="item.unitPrice"
                      type="number"
                      class="input-field small"
                      placeholder="0"
                      @input="calculateItemTotal(index)"
                  />
                </td>
                <td class="price-cell">
                  {{ formatPrice(item.itemTotal) }}
                </td>
                <td class="action-cell">
                  <button class="btn-remove" @click="removeItem(index)" :disabled="orderItems.length === 1">
                    <i class="ti ti-trash"></i>
                  </button>
                </td>
              </tr>
              </tbody>
            </table>
          </div>

          <!-- Add Item Button -->
          <button class="btn-add-item" @click="addItem">
            <i class="ti ti-plus"></i>
            Add
          </button>
        </div>

        <!-- RIGHT: Summary Section -->
        <div class="summary-section">
          <h2 class="section-title">Bill summary</h2>

          <!-- Subtotal -->
          <div class="summary-row">
            <label>Subtotal</label>
            <span class="summary-value">{{ formatPrice(subtotal) }}</span>
          </div>

          <!-- Service Charge -->
          <div class="summary-row service-charge-row">
            <label>Service charge</label>
            <div class="charge-inputs">
              <input
                  v-model.number="serviceChargePercentage"
                  type="number"
                  class="charge-input"
                  @input="calculateServiceCharge"
              />
              <span class="percent">%</span>
              <span class="charge-value">{{ formatPrice(serviceCharge) }}</span>
            </div>
          </div>

          <!-- Divider -->
          <div class="summary-divider"></div>

          <!-- Final Bill -->
          <div class="final-bill-row">
            <label>Final bill</label>
            <span class="final-value">{{ formatPrice(finalBill) }}</span>
          </div>

          <!-- Checkout Button -->
          <button class="btn-checkout" @click="handleCheckout">
            <i class="ti ti-receipt"></i>
            Checkout
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import HeaderComponent from '../component/Header.vue';

export default {
  name: 'MakeOrder',
  components: {
    HeaderComponent
  },
  data() {
    return {
      loggedUser: 'Admin User',
      orderId: Math.floor(Math.random() * 10000),
      orderItems: [
        {
          foodName: '',
          count: 0,
          unitPrice: 0,
          itemTotal: 0
        }
      ],
      serviceChargePercentage: 10,
      serviceCharge: 0,
      subtotal: 0,
      finalBill: 0
    };
  },
  computed: {
    totalItems() {
      return this.orderItems.length;
    }
  },
  methods: {
    addItem() {
      this.orderItems.push({
        foodName: '',
        count: 0,
        unitPrice: 0,
        itemTotal: 0
      });
    },

    removeItem(index) {
      if (this.orderItems.length > 1) {
        this.orderItems.splice(index, 1);
        this.calculateTotals();
      }
    },

    calculateItemTotal(index) {
      const item = this.orderItems[index];
      item.itemTotal = item.count * item.unitPrice;
      this.calculateTotals();
    },

    calculateTotals() {
      this.subtotal = this.orderItems.reduce((sum, item) => {
        return sum + (item.itemTotal || 0);
      }, 0);
      this.calculateServiceCharge();
    },

    calculateServiceCharge() {
      this.serviceCharge = (this.subtotal * this.serviceChargePercentage) / 100;
      this.finalBill = this.subtotal + this.serviceCharge;
    },

    formatPrice(value) {
      return new Intl.NumberFormat('en-US', {
        style: 'currency',
        currency: 'USD',
        minimumFractionDigits: 2,
        maximumFractionDigits: 2
      }).format(value || 0);
    },

    handleCheckout() {
      if (this.subtotal === 0) {
        alert('Please add items to the order');
        return;
      }

      const orderData = {
        orderId: this.orderId,
        items: this.orderItems.filter(item => item.foodName && item.count > 0),
        subtotal: this.subtotal,
        serviceChargePercentage: this.serviceChargePercentage,
        serviceCharge: this.serviceCharge,
        finalBill: this.finalBill,
        timestamp: new Date()
      };

      console.log('Order Data:', orderData);
      alert(`Order #${this.orderId} created successfully!\nTotal: ${this.formatPrice(this.finalBill)}`);

      // Navigate to orders page or reset form
      this.$router.push('/orders');
    },

    handleLogout() {
      this.$emit('logout');
    }
  },
  mounted() {
    this.calculateTotals();
  }
};
</script>

<style scoped>
.make-order-page {
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden;
  background: var(--color-background-tertiary);
}

.order-content {
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.order-layout {
  display: flex;
  gap: 1.5rem;
  flex: 1;
  overflow: hidden;
  padding: 1.5rem;
  max-width: 1400px;
  margin: 0 auto;
  width: 100%;
}

/* === ITEMS SECTION === */
.items-section {
  flex: 1;
  display: flex;
  flex-direction: column;
  background: var(--color-background-primary);
  border-radius: var(--border-radius-lg);
  border: 1px solid var(--color-border-tertiary);
  padding: 1rem;
  overflow: hidden;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
  padding-bottom: 1rem;
  border-bottom: 1px solid var(--color-border-tertiary);
}

.section-title {
  margin: 0;
  font-size: 16px;
  font-weight: 500;
  color: var(--color-text-primary);
}

.order-id {
  margin: 0;
  font-size: 12px;
  color: var(--color-text-secondary);
}

.table-wrapper {
  flex: 1;
  overflow-y: auto;
  overflow-x: hidden;
  margin-bottom: 0.75rem;
  border-radius: 4px;
}

.items-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 13px;
}

.items-table thead {
  position: sticky;
  top: 0;
  background: var(--color-background-secondary);
  border-bottom: 1px solid var(--color-border-tertiary);
  z-index: 10;
}

.items-table th {
  padding: 0.6rem 0.5rem;
  text-align: left;
  font-weight: 500;
  color: var(--color-text-secondary);
}

.items-table td {
  padding: 0.6rem 0.5rem;
  border-bottom: 1px solid var(--color-border-tertiary);
  vertical-align: middle;
}

.items-table tbody tr:hover {
  background: var(--color-background-secondary);
}

.input-field {
  width: 100%;
  padding: 0.4rem;
  border: 1px solid var(--color-border-tertiary);
  border-radius: 3px;
  font-size: 12px;
  color: var(--color-text-primary);
  background: var(--color-background-primary);
  transition: border-color var(--transition-fast);
}

.input-field.small {
  width: 60px;
  text-align: center;
}

.input-field:focus {
  outline: none;
  border-color: var(--color-info);
  box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.1);
}

.price-cell {
  text-align: right;
  font-weight: 500;
  color: var(--color-text-primary);
  font-family: monospace;
  font-size: 12px;
  white-space: nowrap;
}

.action-cell {
  text-align: center;
  width: 40px;
}

.btn-remove {
  padding: 0.3rem 0.4rem;
  border: 1px solid var(--color-border-danger);
  background: transparent;
  color: var(--color-text-danger);
  border-radius: 3px;
  cursor: pointer;
  transition: all var(--transition-fast);
  display: inline-flex;
  align-items: center;
  justify-content: center;
  font-size: 14px;
}

.btn-remove:hover:not(:disabled) {
  background: rgba(239, 68, 68, 0.1);
}

.btn-remove:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

.btn-add-item {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.4rem;
  padding: 0.6rem;
  border: 1px dashed var(--color-border-info);
  background: transparent;
  color: var(--color-info);
  border-radius: 4px;
  cursor: pointer;
  font-size: 13px;
  font-weight: 500;
  transition: all var(--transition-fast);
}

.btn-add-item:hover {
  background: rgba(59, 130, 246, 0.05);
}

/* === SUMMARY SECTION === */
.summary-section {
  width: 350px;
  background: var(--color-background-primary);
  border-radius: var(--border-radius-lg);
  border: 1px solid var(--color-border-tertiary);
  padding: 1.5rem;
  display: flex;
  flex-direction: column;
  gap: 0.8rem;
  overflow-y: auto;
}

.summary-section .section-title {
  margin-bottom: 0.5rem;
}

.summary-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 13px;
  padding: 0.6rem 0;
}

.summary-row label {
  color: var(--color-text-secondary);
  font-weight: 400;
}

.summary-value {
  color: var(--color-text-primary);
  font-weight: 500;
  font-family: monospace;
}

.service-charge-row {
  flex-direction: column;
  align-items: flex-start;
  gap: 0.6rem;
  padding: 0.8rem 0;
  border-top: 1px solid var(--color-border-tertiary);
  border-bottom: 1px solid var(--color-border-tertiary);
}

.charge-inputs {
  width: 100%;
  display: flex;
  align-items: center;
  gap: 0.4rem;
}

.charge-input {
  width: 60px;
  padding: 0.4rem;
  border: 1px solid var(--color-border-tertiary);
  border-radius: 3px;
  font-size: 12px;
  color: var(--color-text-primary);
  text-align: center;
}

.charge-input:focus {
  outline: none;
  border-color: var(--color-info);
  box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.1);
}

.percent {
  color: var(--color-text-secondary);
  font-size: 12px;
  font-weight: 500;
}

.charge-value {
  margin-left: auto;
  color: var(--color-text-primary);
  font-weight: 500;
  font-family: monospace;
  font-size: 12px;
}

.summary-divider {
  height: 1px;
  background: var(--color-border-tertiary);
  margin: 0.4rem 0;
}

.final-bill-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem 0;
  border-top: 1px solid var(--color-border-tertiary);
}

.final-bill-row label {
  font-size: 14px;
  font-weight: 500;
  color: var(--color-text-primary);
}

.final-value {
  font-size: 20px;
  font-weight: 500;
  color: var(--color-success);
  font-family: monospace;
}

.btn-checkout {
  width: 100%;
  padding: 0.8rem;
  margin-top: 0.5rem;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  border-radius: var(--border-radius-md);
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all var(--transition-normal);
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  box-shadow: 0 2px 8px rgba(102, 126, 234, 0.3);
}

.btn-checkout:hover {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.4);
}

.btn-checkout:active {
  transform: translateY(0);
}

.btn-checkout i {
  font-size: 16px;
}

/* === RESPONSIVE === */
@media (max-width: 1024px) {
  .order-layout {
    flex-direction: column;
    gap: 1rem;
    padding: 1rem;
  }

  .summary-section {
    width: 100%;
    max-height: 300px;
  }

  .items-section {
    flex: 1 1 auto;
    min-height: 400px;
  }
}

@media (max-width: 768px) {
  .make-order-page {
    height: auto;
  }

  .order-content {
    min-height: calc(100vh - 80px);
  }

  .order-layout {
    flex-direction: column;
    gap: 0.75rem;
    padding: 0.75rem;
  }

  .items-section {
    max-height: 50vh;
  }

  .summary-section {
    width: 100%;
    max-height: none;
  }

  .section-title {
    font-size: 14px;
  }

  .items-table {
    font-size: 12px;
  }

  .items-table th,
  .items-table td {
    padding: 0.4rem 0.3rem;
  }

  .input-field.small {
    width: 50px;
  }
}

/* Scrollbar styling */
.table-wrapper::-webkit-scrollbar,
.summary-section::-webkit-scrollbar {
  width: 6px;
}

.table-wrapper::-webkit-scrollbar-track,
.summary-section::-webkit-scrollbar-track {
  background: transparent;
}

.table-wrapper::-webkit-scrollbar-thumb,
.summary-section::-webkit-scrollbar-thumb {
  background: var(--color-border-secondary);
  border-radius: 3px;
}

.table-wrapper::-webkit-scrollbar-thumb:hover,
.summary-section::-webkit-scrollbar-thumb:hover {
  background: var(--color-border-primary);
}
</style>
