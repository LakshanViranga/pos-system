<template>
  <v-app>
    <v-main>
      <Login v-if="!isLoggedIn" @login="isLoggedIn = true"/>
      <v-container v-else fluid>
        <v-row>
          <!-- LEFT MENU -->
<!--          <v-col cols="2">-->
<!--            <v-list>-->
<!--              <v-list-item to="/" title="Dashboard" />-->
<!--              <v-list-item to="/orders" title="Orders" />-->
<!--              <v-list-item to="/products" title="Products" />-->
<!--              <v-list-item to="/settings" title="Settings" />-->
<!--            </v-list>-->
<!--          </v-col>-->
          <!-- RIGHT CONTENT -->
          <v-col cols="12">
            <router-view />
          </v-col>
        </v-row>
      </v-container>
    </v-main>

  </v-app>
</template>

<script>
import Login from './component/Login.vue';
import { useProductStore } from './stores/product.ts';

export default {
  components: {
    Login
  },
  data() {
    return {
      isLoggedIn: false
    };
  },
  async mounted() {
    const productStore = useProductStore();
    await productStore.fetchProducts();
  }
};
</script>
