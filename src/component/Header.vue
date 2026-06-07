<template>
  <header class="header">
    <div class="header-container">
      <!-- LEFT SECTION: Logo & User Info -->
      <div class="header-left">
        <div class="logo-section">
          <div class="logo-icon">
            <img src="../../images/first-log.png" alt="logo"/>
          </div>
          <div class="restaurant-info">
            <h3 class="restaurant-name">Sri Thali Restaurant</h3>
            <p class="user-info">{{ userName }}</p>
          </div>
        </div>
      </div>

      <!-- RIGHT SECTION: Action Buttons -->
      <div class="header-right">
        <button class="header-button back-button" @click="goBack" :disabled="!showBackButton">
          <i class="ti ti-arrow-left"></i>
          <span>Back</span>
        </button>
        <button class="header-button logout-button" @click="handleLogout">
          <i class="ti ti-logout"></i>
          <span>Logout</span>
        </button>
      </div>
    </div>
  </header>
</template>

<script>
export default {
  name: 'Header',
  props: {
    userName: {
      type: String,
      default: 'User'
    },
    showBackButton: {
      type: Boolean,
      default: true
    }
  },
  methods: {
    goBack() {
      this.$router.go(-1);
    },
    handleLogout() {
      this.$emit('logout');
    }
  }
};
</script>

<style scoped>
.header {
  background: var(--color-background-primary);
  border-bottom: 1px solid var(--color-border-tertiary);
  padding: 0;
  position: sticky;
  top: 0;
  z-index: 100;
}

.header-container {
  max-width: 100%;
  margin: 0 auto;
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem 2rem;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.logo-section {
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

.logo-icon {
  width: 48px;
  height: 48px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border-radius: var(--border-radius-md);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 24px;
  color: white;
  flex-shrink: 0;
}

.logo-icon img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.restaurant-info {
  display: flex;
  flex-direction: column;
  gap: 0.2rem;
}

.restaurant-name {
  margin: 0;
  font-size: 16px;
  font-weight: 500;
  color: var(--color-text-primary);
}

.user-info {
  margin: 0;
  font-size: 13px;
  color: var(--color-text-secondary);
}

.header-right {
  display: flex;
  gap: 0.5rem;
  align-items: center;
}

.header-button {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.75rem 1rem;
  border: 1px solid var(--color-border-tertiary);
  background: var(--color-background-secondary);
  color: var(--color-text-primary);
  border-radius: var(--border-radius-md);
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: all 0.2s ease;
  white-space: nowrap;
}

.header-button:hover:not(:disabled) {
  background: var(--color-background-tertiary);
  border-color: var(--color-border-secondary);
}

.header-button:active:not(:disabled) {
  transform: scale(0.98);
}

.header-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.logout-button {
  border-color: var(--color-border-warning);
  color: var(--color-text-warning);
}

.logout-button:hover:not(:disabled) {
  background: rgba(251, 146, 60, 0.1);
}

@media (max-width: 768px) {
  .header-container {
    padding: 0.75rem 1rem;
  }

  .header-button span {
    display: none;
  }

  .header-button {
    padding: 0.75rem;
  }

  .restaurant-info {
    display: none;
  }
}
</style>
