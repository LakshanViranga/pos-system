<template>
  <div>
    <h2>Menu</h2>

    <v-row>
      <v-col
          v-for="item in products"
          :key="item.id"
          cols="4"
      >
        <v-card @click="$emit('add', item)" class="pa-3" hover>
          <v-card-title>{{ item.name }}</v-card-title>
          <v-card-subtitle>Rs. {{ item.price }}</v-card-subtitle>
        </v-card>
      </v-col>
    </v-row>
  </div>
</template>

<script setup>
const products = [
  { id: 1, name: 'Burger', price: 1200 },
  { id: 2, name: 'Pizza', price: 2500 },
  { id: 3, name: 'Fries', price: 800 },
  { id: 4, name: 'Coke', price: 400 },
  { id: 5, name: 'Coffee', price: 600 },
]
</script>
