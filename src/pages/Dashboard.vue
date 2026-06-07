<template>
  <div class="dashboard">
    <!-- Header Component -->
    <header-component
        :userName="`Logged in as ${loggedUser}`"
        :showBackButton="false"
        @logout="handleLogout"
    />

    <!-- Dashboard Content -->
    <div class="dashboard-content">
      <div class="dashboard-container">

        <!-- First Row: Make Order Button -->
        <div class="action-row">
          <button class="make-order-button" @click="navigateToOrders">
            <i class="fa fa-plus" aria-hidden="true"></i>
            <span>Make Order</span>
          </button>
        </div>

        <!-- Dashboard Stats Grid -->
        <div class="stats-grid">
          <div class="stat-card">
            <div class="stat-icon today">
              <i class="fa fa-calendar"></i>
            </div>
            <div class="stat-content">
              <p class="stat-label">Today's Orders</p>
              <h3 class="stat-value">{{ todayOrders }}</h3>
            </div>
          </div>

          <div class="stat-card">
            <div class="stat-icon revenue">
              <i class="fa fa-dollar"></i>
            </div>
            <div class="stat-content">
              <p class="stat-label">Today's Revenue</p>
              <h3 class="stat-value">${{ todayRevenue }}</h3>
            </div>
          </div>

          <div class="stat-card">
            <div class="stat-icon products">
              <i class="fa fa-list"></i>
            </div>
            <div class="stat-content">
              <p class="stat-label">Total Products</p>
              <h3 class="stat-value">{{ totalProducts }}</h3>
            </div>
          </div>

          <div class="stat-card">
            <div class="stat-icon pending">
              <i class="fa fa-clock-o"></i>
            </div>
            <div class="stat-content">
              <p class="stat-label">Pending Orders</p>
              <h3 class="stat-value">{{ pendingOrders }}</h3>
            </div>
          </div>
        </div>

        <!-- Quick Links -->
        <div class="quick-links">
          <h2 class="section-title">Quick access</h2>
          <div class="links-grid">
            <router-link to="/view-order" class="quick-link">
              <i class="fa fa-check"></i>
              <span>View Orders</span>
            </router-link>
            <router-link to="/products" class="quick-link">
              <i class="fa fa-list"></i>
              <span>Manage Products</span>
            </router-link>
            <router-link to="/settings" class="quick-link">
              <i class="fa fa-gear"></i>
              <span>Settings</span>
            </router-link>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import HeaderComponent from '../component/Header.vue';

export default {
  name: 'Dashboard',
  components: {
    HeaderComponent
  },
  data() {
    return {
      loggedUser: 'Admin User',
      todayOrders: 24,
      todayRevenue: 1280.50,
      totalProducts: 156,
      pendingOrders: 5
    };
  },
  methods: {
    navigateToOrders() {
      this.$router.push('/orders');
    },
    handleLogout() {
      this.$emit('logout');
    }
  }
};
</script>

<style scoped>
.dashboard {
  min-height: 100vh;
  background: var(--color-background-tertiary);
}

.dashboard-content {
  padding: 2rem 0;
}

.dashboard-container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 2rem;
  display: flex;
  flex-direction: column;
  gap: 2.5rem;
}

.action-row {
  display: flex;
  gap: 1rem;
  margin-bottom: 1rem;
}

.make-order-button {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.75rem;
  padding: 1rem 2rem;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  border-radius: var(--border-radius-md);
  font-size: 16px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.3s ease;
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.3);
  min-width: 200px;
}

.make-order-button:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 20px rgba(102, 126, 234, 0.4);
}

.make-order-button:active {
  transform: translateY(0);
}

.make-order-button i {
  font-size: 20px;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 1.5rem;
}

.stat-card {
  background: var(--color-background-primary);
  border: 1px solid var(--color-border-tertiary);
  border-radius: var(--border-radius-lg);
  padding: 1.5rem;
  display: flex;
  align-items: flex-start;
  gap: 1.5rem;
  transition: all 0.2s ease;
}

.stat-card:hover {
  border-color: var(--color-border-secondary);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
}

.stat-icon {
  width: 56px;
  height: 56px;
  border-radius: var(--border-radius-md);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 28px;
  flex-shrink: 0;
  color: white;
}

.stat-icon.today {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}

.stat-icon.revenue {
  background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
}

.stat-icon.products {
  background: linear-gradient(135deg, #4facfe 0%, #00f2fe 100%);
}

.stat-icon.pending {
  background: linear-gradient(135deg, #fa709a 0%, #fee140 100%);
}

.stat-content {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  flex: 1;
}

.stat-label {
  margin: 0;
  font-size: 13px;
  color: var(--color-text-secondary);
  font-weight: 400;
}

.stat-value {
  margin: 0;
  font-size: 24px;
  font-weight: 500;
  color: var(--color-text-primary);
}

.quick-links {
  padding: 2rem 0;
}

.section-title {
  margin: 0 0 1.5rem 0;
  font-size: 18px;
  font-weight: 500;
  color: var(--color-text-primary);
}

.links-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1rem;
}

.quick-link {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 1rem;
  padding: 2rem;
  background: var(--color-background-primary);
  border: 2px solid var(--color-border-tertiary);
  border-radius: var(--border-radius-lg);
  text-decoration: none;
  color: var(--color-text-primary);
  transition: all 0.2s ease;
  cursor: pointer;
}

.quick-link:hover {
  border-color: var(--color-border-primary);
  background: var(--color-background-secondary);
  transform: translateY(-2px);
}

.quick-link i {
  font-size: 32px;
  color: #667eea;
}

.quick-link span {
  font-size: 15px;
  font-weight: 500;
  text-align: center;
}

@media (max-width: 768px) {
  .dashboard-container {
    padding: 0 1rem;
    gap: 1.5rem;
  }

  .make-order-button {
    width: 100%;
    min-width: unset;
  }

  .stats-grid {
    grid-template-columns: 1fr;
  }

  .links-grid {
    grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
  }
}
</style>
