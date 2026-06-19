use crate::model::{Order, CreateOrderRequest, CreateProductRequest, Product};
use crate::AppState;
use rusqlite::params;
use chrono::Local;

#[tauri::command]
pub fn create_order(state: tauri::State<AppState>, request: CreateOrderRequest,) -> Result<String, String> {
    let mut db = state.db.lock().unwrap();
    let tx = db.transaction().map_err(|e| e.to_string())?;
    tx.execute(
        "INSERT INTO orders (order_number, customer_id, subtotal, service_charge, discount_amount, total_amount,payment_method, order_type, notes) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![
            request.order_number,
            request.customer_id,
            request.subtotal,
            request.service_charge,
            request.discount_amount,
            request.total_amount,
            request.payment_method,
            request.order_type,
            request.notes
        ],
    ).map_err(|e| {
        println!("{}", e);
        e.to_string()
    })?;
    tx.commit().map_err(|e| e.to_string())?;

    Ok("order created".to_string())
}

#[tauri::command]
pub fn create_product(state: tauri::State<AppState>, request: CreateProductRequest) -> Result<String, String> {
    let mut db = state.db.lock().unwrap();
    let tx = db.transaction().map_err(|e| e.to_string())?;
    // Insert order
    tx.execute(
        "INSERT INTO products (name, dietary_type, price, description, category)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            request.name ,
            request.dietary_type,
            request.price,
            request.description,
            request.category
        ],
    )
    .map_err(|e| e.to_string())?;

    tx.commit().map_err(|e| e.to_string())?;

    Ok("product is added".to_string())
}

#[tauri::command]
pub fn get_orders(state: tauri::State<AppState>) -> Result<Vec<Order>, String> {
let db = state.db.lock().unwrap();

    let mut stmt = db
        .prepare(
            "SELECT id, order_number, customer_id, subtotal, service_charge, discount_amount,
                    total_amount,order_type, payment_method, payment_status, order_status, notes, created_at, updated_at
             FROM orders ORDER BY created_at DESC",
        )
        .map_err(|e| e.to_string())?;

    let orders = stmt
        .query_map([], |row| {
            Ok(Order {
                    id: row.get(0)?,
                    order_number: row.get(1)?,
                    customer_id: row.get(2)?,
                    subtotal: row.get(3)?,
                    service_charge: row.get(4)?,
                    discount_amount: row.get(5)?,
                    total_amount: row.get(6)?,
                    order_type: row.get(7)?,
                    payment_method: row.get(8)?,
                    payment_status: row.get(9)?,
                    order_status: row.get(10)?,
                    items: Vec::new(),
                    notes: row.get(11)?,
                    created_at: row.get(12)?,
                    updated_at: row.get(13)?,
                }
             )
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<Order>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(orders)
}

#[tauri::command]
pub fn get_products(state: tauri::State<AppState>) -> Result<Vec<Product>, String> {
    let db = state.db.lock().unwrap();

    let mut stmt = db
        .prepare(
            "SELECT id, name, dietary_type, price, description,category,created_at
             FROM products ORDER BY created_at DESC",
        )
        .map_err(|e| e.to_string())?;

    let products = stmt
        .query_map([], |row| {
            Ok(Product {
                id: row.get(0)?,
                name: row.get(1)?,
                dietary_type: row.get(2)?,
                price: row.get(3)?,
                description: row.get(4)?,
                category: row.get(5)?,
                created_at: row.get(6)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<Product>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(products)
}

#[tauri::command]
pub fn get_order_sequence(state: tauri::State<AppState>) -> Result<String, String> {
     let mut db = state.db.lock().unwrap();
     let tx = db.transaction().map_err(|e| e.to_string())?;
     tx.execute(
             "INSERT INTO order_sequence (seq_date, counter) VALUES (date('now'), 1) ON CONFLICT(seq_date)
             DO UPDATE SET counter = counter + 1",
             [],
         )
         .map_err(|e| e.to_string())?;

     // read counter
    let counter = tx.query_row(
                 "SELECT counter FROM order_sequence WHERE seq_date = date('now')",
                 [],
                 |row| row.get::<_, i64>(0),
             )
             .map_err(|e| e.to_string())?;

    tx.commit().map_err(|e| e.to_string())?;

    let date = Local::now().format("%Y%m%d")
             .to_string();
     Ok(format!("{}-{:05}", date, counter))
}
