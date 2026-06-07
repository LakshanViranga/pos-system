<template>
  <div class="login-container">
    <div class="login-card">
      <!-- Row 1: Logo -->
      <div class="logo-section">
        <div class="logo-circle">
          <img src="../../images/first-log.png" alt="logo"/>
        </div>
      </div>

      <!-- Row 2: Restaurant Name -->
      <div class="name-section">
        <h2>Sri Thali Restaurant</h2>
      </div>

      <!-- Row 3: Button -->
      <div class="button-section">
        <button class="enter-button" @click="handleLogin">
          enter!!
        </button>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'Login',
  methods: {
    handleLogin() {
      this.$emit('login');
    }
  }
};
</script>

<style scoped>
.login-container {
  width: 100%;
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 25%, #f093fb 50%, #4facfe 75%, #00f2fe 100%);
  background-size: 400% 400%;
  animation: gradient 15s ease infinite;
  font-family: serif;
}

@keyframes gradient {
  0% { background-position: 0% 50%; }
  50% { background-position: 100% 50%; }
  100% { background-position: 0% 50%; }
}

.login-card {
  background: cornsilk;
  border-radius: 3px;
  padding: 4rem 3rem;
  width: 100%;
  max-width: 400px;
  box-shadow: 0 10px 40px rgba(0, 0, 0, 0.1);
  display: flex;
  flex-direction: column;
  gap: 2.5rem;
}

.logo-section {
  display: flex;
  justify-content: center;
  padding: 1rem 0;
}

.logo-circle {
  width: 100px;
  height: 100px;
  border-radius: 50%;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 40px;
  color: white;
}

.logo-circle img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.name-section {
  display: flex;
  justify-content: center;
  text-align: center;
}

.name-section h2 {
  margin: 0;
  font-size: 28px;
  font-weight: 500;
  color: #71ae19;
}

.button-section {
  display: flex;
  justify-content: center;
  padding: 1rem 0;
}

.enter-button {
  padding: 12px 48px;
  font-size: 16px;
  font-weight: 500;
  border: none;
  border-radius: 3px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  cursor: pointer;
  transition: all 0.3s ease;
  box-shadow: 0 4px 15px rgba(102, 126, 234, 0.4);
}

.enter-button:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 20px rgba(102, 126, 234, 0.6);
}

.enter-button:active {
  transform: translateY(0);
}
</style>

