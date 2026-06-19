export interface CreateProductRequest {
    name: string;
    dietary_type: string;
    price: number;
    description: string;
    category: string;
}

export interface Product {
    id: number;
    name: string;
    dietary_type: string;
    price: number;
    description: string;
    category: string;
    created_at: string;
}
