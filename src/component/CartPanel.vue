<template>
  <div>
    <h2>Current Order</h2>

    <v-list>
      <v-list-item
          v-for="item in cart"
          :key="item.id"
      >
        <v-list-item-title>
          {{ item.name }} x{{ item.qty }}
        </v-list-item-title>

        <template #append>
          Rs. {{ item.price * item.qty }}
        </template>
      </v-list-item>
    </v-list>

    <v-divider class="my-3" />

    <h3>Total: Rs. {{ total }}</h3>

    <v-btn color="success" block class="mt-3">
      Checkout
    </v-btn>
  </div>
</template>

<script setup>
import { computed } from 'vue'

const props = defineProps({
  cart: Array
})

const total = computed(() =>
    props.cart.reduce((sum, i) => sum + i.price * i.qty, 0)
)
</script>
