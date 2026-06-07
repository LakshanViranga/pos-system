import { createRouter, createWebHashHistory } from 'vue-router'
import Dashboard from "../pages/Dashboard.vue";
import MakeOrder from "../pages/MakeOrder.vue";
import ViewOrder from "../pages/ViewOrder.vue";
import Products from "../pages/ManageProducts.vue";

const routes = [
    { path: '/', component: Dashboard },
    { path: '/orders', component: MakeOrder },
    { path: '/view-order', component: ViewOrder },
    { path: '/products', component: Products },
]

const index = createRouter({
    history: createWebHashHistory(), // IMPORTANT for Electron
    routes
})

export default index
