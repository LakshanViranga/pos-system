import { invoke } from '@tauri-apps/api/core';
import type {CreateOrderRequest, Order} from '../types/order/models';
import type {CreateProductRequest, Product} from "../types/product";

export interface OrderItem {
    id?: number;
    order_id?: number;
    product_id: number;
    quantity: number;
    unit_price: number;
    item_total: number;
    notes?: string;
}

export const dbService = {
    // Orders
    async createOrder(request: CreateOrderRequest) {
        return invoke<Order>('create_order', { request });
    },

    async getAllOrders() {
        return invoke<Order[]>('get_orders');
    },

    async getOrdersByDate(date: string) {
        return invoke<Order[]>('get_orders_by_date', { date });
    },

    async createProduct(request: CreateProductRequest) {
        return invoke('create_product', { request });
    },
    async getProducts() {
        return invoke<Product[]>('get_products');
    },
    async getOrderId() {
        return invoke<String>('get_order_sequence');
    }
};
