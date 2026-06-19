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
            <v-table class="items-table" density="compact" hover>
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
                  <v-select
                      v-model="item.foodName"
                      :items="productItem"
                      item-title="name"
                      return-object
                      class="food-select"
                      placeholder="Select food"
                      density="compact"
                      variant="outlined"
                      hide-details
                      @update:modelValue="(value) => handleFoodSelect(index, value)"
                  />
                </td>
                <td>
                  <v-text-field
                      v-model.number="item.count"
                      type="number"
                      class="qty-input"
                      placeholder="0"
                      density="compact"
                      variant="outlined"
                      hide-details
                      @input="calculateItemTotal(index)"
                  />
                </td>
                <td>
                  <v-text-field
                      v-model.number="item.unitPrice"
                      type="number"
                      class="price-input"
                      placeholder="0"
                      density="compact"
                      variant="outlined"
                      hide-details
                      readonly
                      @input="calculateItemTotal(index)"
                  />
                </td>
                <td class="price-cell">
                  {{ formatPrice(item.itemTotal) }}
                </td>
                <td class="action-cell">
                  <v-btn
                      icon
                      size="small"
                      variant="text"
                      color="error"
                      :disabled="orderItems.length === 1"
                      @click="removeItem(index)"
                  >
                    <v-icon icon="fa fa-trash" />
                  </v-btn>
                </td>
              </tr>
              </tbody>
            </v-table>
          </div>

          <!-- Add Item Button -->
          <v-btn
              class="btn-add-item"
              prepend-icon="fa fa-plus"
              variant="outlined"
              color="info"
              size="small"
              @click="addItem"
          >
            Add
          </v-btn>
        </div>

        <!-- RIGHT: Summary Section -->
        <div class="summary-section">
          <h2 class="section-title">Bill summary</h2>

          <!-- Subtotal -->
          <div class="summary-row">
            <v-label class="summary-label">Subtotal</v-label>
            <span class="summary-value">{{ formatPrice(subtotal) }}</span>
          </div>

          <!-- Service Charge -->
          <div class="summary-row service-charge-row">
            <v-label class="summary-label">Service charge</v-label>
            <div class="charge-inputs">
              <v-text-field
                  v-model.number="serviceChargePercentage"
                  type="number"
                  class="charge-input"
                  density="compact"
                  variant="outlined"
                  hide-details
                  @input="calculateServiceCharge"
                  readonly
              />
              <span class="percent">%</span>
              <span class="charge-value">{{ formatPrice(serviceCharge) }}</span>
            </div>
          </div>

          <!-- Divider -->
          <v-divider class="summary-divider" />

          <!-- Final Bill -->
          <div class="final-bill-row">
            <v-label class="final-label">Final bill</v-label>
            <span class="final-value">{{ formatPrice(finalBill) }}</span>
          </div>

          <!-- Checkout Button -->
          <v-btn
              class="btn-checkout"
              prepend-icon="fa fa-receipt"
              @click="handleCheckout"
          >
            Checkout
          </v-btn>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import HeaderComponent from '../component/Header.vue';
import { useProductStore } from "../stores/product.ts";
import { dbService } from '../services/db.ts';

export default {
  name: 'MakeOrder',
  components: {
    HeaderComponent
  },
  data() {
    return {
      loggedUser: 'Admin User',
      orderId: null,
      orderItems: [
        {
          foodName: null,
          count: 0,
          unitPrice: 0,
          itemTotal: 0
        }
      ],
      serviceChargePercentage: 10,
      serviceCharge: 0,
      subtotal: 0,
      finalBill: 0,
      productItem: Array.from(useProductStore().products.values())
    };
  },
  computed: {
    totalItems() {
      return this.orderItems.length;
    }
  },
  methods: {
    /**
     * Handle food selection and auto-update unit price
     */
    handleFoodSelect(index, selectedProduct) {
      const item = this.orderItems[index];
      if (selectedProduct && selectedProduct.price) {
        item.unitPrice = selectedProduct.price;
        this.calculateItemTotal(index);
      }
    },

    /**
     * Add new item to order
     */
    addItem() {
      this.orderItems.push({
        foodName: null,
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

    /**
     * Calculate subtotal and final amounts
     */
    calculateTotals() {
      this.subtotal = this.orderItems.reduce((sum, item) => {
        return sum + (item.itemTotal || 0);
      }, 0);
      this.calculateServiceCharge();
    },

    /**
     * Calculate service charge and final bill
     */
    calculateServiceCharge() {
      this.serviceCharge = (this.subtotal * this.serviceChargePercentage) / 100;
      this.finalBill = this.subtotal + this.serviceCharge;
    },

    /**
     * Format price to LKR currency
     */
    formatPrice(value) {
      return new Intl.NumberFormat('en-US', {
        style: 'currency',
        currency: 'LKR',
        minimumFractionDigits: 2,
        maximumFractionDigits: 2
      }).format(value || 0);
    },

    /**
     * Handle checkout
     */
    async handleCheckout() {
      if (this.subtotal === 0) {
        alert('Please add items to the order');
        return;
      }

      const createOrderRequest = {
        order_number: await dbService.getOrderId(),
        items: [],
        customer_id:"fresh",
        subtotal: this.subtotal,
        service_charge: this.serviceCharge,
        discount_amount: 0,
        payment_method: "CASH",
        total_amount: this.finalBill,
        notes: "no",
        order_type: "take away"
      }

      // call backend endpoint
      try {
        await dbService.createOrder(createOrderRequest)
      } catch (error) {
        console.log(error);
      }

      // Navigate to orders page or reset form
      this.$router.push('/orders');
    },

    /**
     * Handle logout
     */
    handleLogout() {
      this.$emit('logout');
    }
  },
  mounted() {
    this.calculateTotals();
    console.log('memory product store', this.productItem);
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

.food-select {
  min-width: 120px;
}

.qty-input,
.price-input {
  max-width: 80px;
}

.food-select :deep(.v-field__input),
.qty-input :deep(.v-field__input),
.price-input :deep(.v-field__input) {
  font-size: 12px;
  padding: 0.3rem !important;
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

.btn-add-item {
  margin-top: 0.5rem;
  text-transform: none;
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

.summary-label {
  color: var(--color-text-secondary);
  font-weight: 400;
  font-size: 13px;
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
  max-width: 80px;
}

.charge-input :deep(.v-field__input) {
  font-size: 12px;
  padding: 0.3rem !important;
  text-align: center;
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
  margin: 0.4rem 0;
}

.final-bill-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem 0;
  border-top: 1px solid var(--color-border-tertiary);
}

.final-label {
  font-size: 14px;
  font-weight: 500;
  color: var(--color-text-primary);
}

.final-value {
  font-size: 20px;
  font-weight: 600;
  color: var(--color-success);
  font-family: monospace;
}

.btn-checkout {
  text-transform: none;
  letter-spacing: 0;
  box-shadow: 0 2px 8px rgba(102, 126, 234, 0.3);
  background-color: var(--color-danger);
}

.btn-checkout:hover {
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.4);
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

  .food-select,
  .qty-input,
  .price-input {
    max-width: 60px;
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
