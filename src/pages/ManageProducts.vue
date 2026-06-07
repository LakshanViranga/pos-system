<template>
  <div class="products-page">
    <!-- Header Component -->
    <header-component
        :userName="`Logged in as ${loggedUser}`"
        :showBackButton="true"
        @logout="handleLogout"
    />

    <!-- Products Content -->
    <div class="products-content">
      <div class="products-container">

        <!-- Page Header with Add Button -->
        <div class="page-header">
          <h1>Manage Products</h1>
          <button class="btn-add-product" @click="openAddModal">
            <i class="ti ti-plus"></i>
            <span>Add Product</span>
          </button>
        </div>

        <!-- Products Table -->
        <div class="table-section">
          <div class="table-wrapper">
            <table class="products-table">
              <thead>
              <tr>
                <th>Product name</th>
                <th>Category</th>
                <th>Unit price</th>
                <th>Type</th>
                <th>Status</th>
                <th>Actions</th>
              </tr>
              </thead>
              <tbody>
              <tr v-for="product in products" :key="product.id" class="product-row">
                <td class="product-name">
                  <strong>{{ product.name }}</strong>
                </td>
                <td class="category">
                  {{ product.category }}
                </td>
                <td class="unit-price">
                  {{ formatPrice(product.unitPrice) }}
                </td>
                <td class="type">
                    <span :class="['type-badge', product.type.toLowerCase()]">
                      {{ product.type }}
                    </span>
                </td>
                <td class="status">
                    <span :class="['status-badge', product.active ? 'active' : 'inactive']">
                      {{ product.active ? 'Active' : 'Inactive' }}
                    </span>
                </td>
                <td class="actions">
                  <button class="btn-action edit" @click="openEditModal(product)" title="Edit">
                    <i class="fa fa-edit"></i>
                  </button>
                  <button class="btn-action delete" @click="deleteProduct(product.id)" title="Delete">
                    <i class="fa fa-trash"></i>
                  </button>
                </td>
              </tr>
              </tbody>
            </table>

            <!-- Empty State -->
            <div v-if="products.length === 0" class="empty-state">
              <i class="ti ti-package-off"></i>
              <p>No products found</p>
              <button class="btn-add-empty" @click="openAddModal">Add your first product</button>
            </div>
          </div>
        </div>

        <!-- Summary Stats -->
        <div class="summary-stats">
          <div class="stat-card">
            <label>Total Products</label>
            <span class="stat-value">{{ products.length }}</span>
          </div>
          <div class="stat-card">
            <label>Vegetarian</label>
            <span class="stat-value veg">{{ vegCount }}</span>
          </div>
          <div class="stat-card">
            <label>Non-Vegetarian</label>
            <span class="stat-value non-veg">{{ nonVegCount }}</span>
          </div>
          <div class="stat-card">
            <label>Active Products</label>
            <span class="stat-value">{{ activeCount }}</span>
          </div>
        </div>
      </div>
    </div>

    <!-- Add/Edit Product Modal -->
    <div v-if="showModal" class="modal-overlay" @click="closeModal">
      <div class="modal-content" @click.stop>
        <div class="modal-header">
          <h2>{{ isEditMode ? 'Edit Product' : 'Add Product' }}</h2>
          <button class="btn-close" @click="closeModal">
            <i class="ti ti-x"></i>
          </button>
        </div>

        <div class="modal-body">
          <form @submit.prevent="saveProduct">
            <!-- Product Name -->
            <div class="form-group">
              <label for="name">Product Name *</label>
              <input
                  id="name"
                  v-model="formData.name"
                  type="text"
                  class="input-field"
                  placeholder="Enter product name"
                  required
              />
            </div>

            <!-- Category -->
            <div class="form-group">
              <label for="category">Category *</label>
              <select
                  id="category"
                  v-model="formData.category"
                  class="input-field"
                  required
              >
                <option value="">Select category</option>
                <option value="Appetizers">Appetizers</option>
                <option value="Main Course">Main Course</option>
                <option value="Biryani">Biryani</option>
                <option value="Curry">Curry</option>
                <option value="Bread">Bread</option>
                <option value="Beverages">Beverages</option>
                <option value="Desserts">Desserts</option>
              </select>
            </div>

            <!-- Unit Price -->
            <div class="form-group">
              <label for="price">Unit Price *</label>
              <input
                  id="price"
                  v-model.number="formData.unitPrice"
                  type="number"
                  class="input-field"
                  placeholder="0.00"
                  step="0.01"
                  required
              />
            </div>

            <!-- Type (Veg/Non-Veg) -->
            <div class="form-group">
              <label for="type">Type *</label>
              <select
                  id="type"
                  v-model="formData.type"
                  class="input-field"
                  required
              >
                <option value="">Select type</option>
                <option value="Veg">Vegetarian</option>
                <option value="Non-Veg">Non-Vegetarian</option>
              </select>
            </div>

            <!-- Description -->
            <div class="form-group">
              <label for="description">Description</label>
              <textarea
                  id="description"
                  v-model="formData.description"
                  class="input-field textarea"
                  placeholder="Enter product description"
                  rows="3"
              ></textarea>
            </div>

            <!-- Active Status -->
            <div class="form-group checkbox">
              <input
                  id="active"
                  v-model="formData.active"
                  type="checkbox"
              />
              <label for="active">Active product</label>
            </div>

            <!-- Form Actions -->
            <div class="form-actions">
              <button type="button" class="btn-secondary" @click="closeModal">Cancel</button>
              <button type="submit" class="btn-primary">
                {{ isEditMode ? 'Update Product' : 'Add Product' }}
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import HeaderComponent from '../component/Header.vue';

export default {
  name: 'Products',
  components: {
    HeaderComponent
  },
  data() {
    return {
      loggedUser: 'Admin User',
      showModal: false,
      isEditMode: false,
      editingProductId: null,
      formData: {
        name: '',
        category: '',
        unitPrice: '',
        type: '',
        description: '',
        active: true
      },
      products: [
        {
          id: 1,
          name: 'Chicken Biryani',
          category: 'Biryani',
          unitPrice: 250,
          type: 'Non-Veg',
          description: 'Fragrant basmati rice with tender chicken pieces',
          active: true
        },
        {
          id: 2,
          name: 'Vegetable Biryani',
          category: 'Biryani',
          unitPrice: 180,
          type: 'Veg',
          description: 'Mixed vegetables with basmati rice',
          active: true
        },
        {
          id: 3,
          name: 'Butter Chicken',
          category: 'Curry',
          unitPrice: 320,
          type: 'Non-Veg',
          description: 'Creamy tomato based curry with chicken',
          active: true
        },
        {
          id: 4,
          name: 'Palak Paneer',
          category: 'Curry',
          unitPrice: 220,
          type: 'Veg',
          description: 'Spinach curry with cottage cheese',
          active: true
        },
        {
          id: 5,
          name: 'Garlic Naan',
          category: 'Bread',
          unitPrice: 80,
          type: 'Veg',
          description: 'Soft bread topped with garlic and butter',
          active: true
        },
        {
          id: 6,
          name: 'Tandoori Chicken',
          category: 'Appetizers',
          unitPrice: 280,
          type: 'Non-Veg',
          description: 'Grilled chicken marinated in yogurt and spices',
          active: true
        }
      ]
    };
  },
  computed: {
    vegCount() {
      return this.products.filter(p => p.type === 'Veg').length;
    },
    nonVegCount() {
      return this.products.filter(p => p.type === 'Non-Veg').length;
    },
    activeCount() {
      return this.products.filter(p => p.active).length;
    }
  },
  methods: {
    formatPrice(value) {
      return new Intl.NumberFormat('en-US', {
        style: 'currency',
        currency: 'USD',
        minimumFractionDigits: 2,
        maximumFractionDigits: 2
      }).format(value || 0);
    },
    openAddModal() {
      this.isEditMode = false;
      this.editingProductId = null;
      this.resetForm();
      this.showModal = true;
    },
    openEditModal(product) {
      this.isEditMode = true;
      this.editingProductId = product.id;
      this.formData = { ...product };
      this.showModal = true;
    },
    closeModal() {
      this.showModal = false;
      this.resetForm();
    },
    resetForm() {
      this.formData = {
        name: '',
        category: '',
        unitPrice: '',
        type: '',
        description: '',
        active: true
      };
    },
    saveProduct() {
      if (!this.formData.name || !this.formData.category || !this.formData.unitPrice || !this.formData.type) {
        alert('Please fill all required fields');
        return;
      }

      if (this.isEditMode) {
        // Update existing product
        const index = this.products.findIndex(p => p.id === this.editingProductId);
        if (index > -1) {
          this.products[index] = {
            ...this.products[index],
            ...this.formData
          };
        }
      } else {
        // Add new product
        const newProduct = {
          id: Math.max(...this.products.map(p => p.id), 0) + 1,
          ...this.formData
        };
        this.products.push(newProduct);
      }

      this.closeModal();
    },
    deleteProduct(productId) {
      if (confirm('Are you sure you want to delete this product?')) {
        const index = this.products.findIndex(p => p.id === productId);
        if (index > -1) {
          this.products.splice(index, 1);
        }
      }
    },
    handleLogout() {
      this.$emit('logout');
    }
  }
};
</script>

<style scoped>
.products-page {
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden;
  background: var(--color-background-tertiary);
}

.products-content {
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.products-container {
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

.btn-add-product {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.75rem 1.5rem;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  border-radius: var(--border-radius-md);
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: all var(--transition-normal);
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.3);
}

.btn-add-product:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 20px rgba(102, 126, 234, 0.4);
}

.btn-add-product i {
  font-size: 18px;
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

.products-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 13px;
}

.products-table thead {
  position: sticky;
  top: 0;
  background: var(--color-background-secondary);
  border-bottom: 1px solid var(--color-border-tertiary);
  z-index: 10;
}

.products-table th {
  padding: 0.75rem;
  text-align: left;
  font-weight: 500;
  color: var(--color-text-secondary);
}

.products-table td {
  padding: 0.75rem;
  border-bottom: 1px solid var(--color-border-tertiary);
  vertical-align: middle;
}

.products-table tbody tr:hover {
  background: var(--color-background-secondary);
}

.product-name {
  font-family: inherit;
  color: var(--color-text-primary);
}

.category {
  color: var(--color-text-secondary);
}

.unit-price {
  text-align: right;
  font-family: monospace;
  color: var(--color-success);
  font-weight: 500;
}

.type {
  text-align: center;
}

.type-badge {
  display: inline-block;
  padding: 0.4rem 0.8rem;
  border-radius: 20px;
  font-size: 11px;
  font-weight: 500;
}

.type-badge.veg {
  background: #dcfce7;
  color: #16a34a;
}

.type-badge.non-veg {
  background: #fee2e2;
  color: #dc2626;
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

.status-badge.active {
  background: #dcfce7;
  color: #16a34a;
}

.status-badge.inactive {
  background: #f3f4f6;
  color: #6b7280;
}

.actions {
  display: flex;
  gap: 0.5rem;
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

.btn-action.edit:hover {
  border-color: #8b5cf6;
  color: #8b5cf6;
  background: rgba(139, 92, 246, 0.05);
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
  padding: 3rem;
  color: var(--color-text-secondary);
}

.empty-state i {
  font-size: 64px;
  margin-bottom: 1rem;
  opacity: 0.3;
}

.empty-state p {
  margin: 0 0 1.5rem 0;
  font-size: 16px;
}

.btn-add-empty {
  padding: 0.75rem 1.5rem;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  border-radius: var(--border-radius-md);
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: all var(--transition-normal);
}

.btn-add-empty:hover {
  transform: translateY(-2px);
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

.stat-value.veg {
  color: #16a34a;
}

.stat-value.non-veg {
  color: #dc2626;
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
  max-width: 500px;
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

.form-group {
  margin-bottom: 1.5rem;
}

.form-group label {
  display: block;
  margin-bottom: 0.5rem;
  font-size: 14px;
  font-weight: 500;
  color: var(--color-text-primary);
}

.form-group.checkbox {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  margin-bottom: 2rem;
}

.form-group.checkbox input {
  width: 18px;
  height: 18px;
  cursor: pointer;
}

.form-group.checkbox label {
  margin: 0;
  cursor: pointer;
  font-weight: 400;
}

.input-field {
  width: 100%;
  padding: 0.75rem;
  border: 1px solid var(--color-border-tertiary);
  border-radius: 4px;
  font-size: 14px;
  color: var(--color-text-primary);
  background: var(--color-background-primary);
  transition: border-color var(--transition-fast);
  font-family: inherit;
}

.input-field:focus {
  outline: none;
  border-color: var(--color-info);
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.input-field.textarea {
  resize: vertical;
  font-family: inherit;
}

.form-actions {
  display: flex;
  gap: 1rem;
  justify-content: flex-end;
  padding-top: 1rem;
  border-top: 1px solid var(--color-border-tertiary);
}

.btn-secondary {
  padding: 0.75rem 1.5rem;
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
  padding: 0.75rem 1.5rem;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  border-radius: var(--border-radius-md);
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: all var(--transition-fast);
}

.btn-primary:hover {
  transform: translateY(-1px);
}

/* === RESPONSIVE === */
@media (max-width: 1024px) {
  .products-container {
    padding: 1rem;
  }

  .products-table {
    font-size: 12px;
  }

  .products-table th,
  .products-table td {
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

  .btn-add-product {
    width: 100%;
  }

  .products-table {
    font-size: 11px;
  }

  .products-table th,
  .products-table td {
    padding: 0.5rem;
  }

  .modal-content {
    width: 95%;
  }

  .summary-stats {
    grid-template-columns: repeat(2, 1fr);
  }
}

/* Scrollbar styling */
.table-wrapper::-webkit-scrollbar,
.modal-content::-webkit-scrollbar {
  width: 6px;
}

.table-wrapper::-webkit-scrollbar-track,
.modal-content::-webkit-scrollbar-track {
  background: transparent;
}

.table-wrapper::-webkit-scrollbar-thumb,
.modal-content::-webkit-scrollbar-thumb {
  background: var(--color-border-secondary);
  border-radius: 3px;
}

.table-wrapper::-webkit-scrollbar-thumb:hover,
.modal-content::-webkit-scrollbar-thumb:hover {
  background: var(--color-border-primary);
}
</style>
