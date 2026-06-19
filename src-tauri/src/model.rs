use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Order {
    pub id: i32,
    pub order_number: String,
    pub customer_id: String,
    pub items: Vec<OrderItem>,
    pub subtotal: f64,
    pub service_charge: f64,
    pub discount_amount: f64,
    pub total_amount: f64,
    pub order_type: String,
    pub payment_method: String,
    pub payment_status: String,
    pub order_status: String,
    pub notes: String,
    pub created_at: String,
    pub updated_at: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderItem {
    pub product_id: i32,
    pub quantity: i32,
    pub unit_price: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateOrderRequest {
    pub order_number: String,
    pub customer_id: String,
    pub items: Vec<OrderItem>,
    pub subtotal: f64,
    pub service_charge: f64,
    pub discount_amount: f64,
    pub payment_method: String,
    pub total_amount: f64,
    pub order_type: String,
    pub notes: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateProductRequest {
    pub name: String,
    pub dietary_type: String,
    pub price: f64,
    pub description: String,
    pub category: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub dietary_type: String,
    pub price: f64,
    pub description: String,
    pub category: String,
    pub created_at: String,
}

