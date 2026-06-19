import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'

interface Product {
    id: number;
    name: string;
    dietary_type: string;
    price: number;
    description: string;
    category: string;
    created_at: string;
}

export const useProductStore = defineStore('products', () => {
    const products = ref<Product[]>([])
    const loading = ref(false)
    const error = ref<string | null>(null)

    const fetchProducts = async () => {
        loading.value = true
        error.value = null
        try {
            // Call your Rust backend
            const result = await invoke<Product[]>('get_products');
            products.value = result
        } catch (err) {
            error.value = err instanceof Error ? err.message : String(err)
        } finally {
            loading.value = false
        }
    }

    return {
        products,
        loading,
        error,
        fetchProducts
    }
},{
    persist:{
        storage: sessionStorage
    }
})
