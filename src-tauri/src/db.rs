use rusqlite::{Connection, Result};
use std::path::PathBuf;

pub fn init_db(db_path: &PathBuf) -> Result<Connection> {
    let conn = Connection::open(db_path)?;

    // Enable foreign keys
    conn.execute("PRAGMA foreign_keys = ON", [])?;

    // Create tables
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS products (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            dietary_type TEXT NOT NULL,
            price REAL NOT NULL,
            description TEXT,
            category TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        -- ===== ORDERS (Main order record) =====
        CREATE TABLE IF NOT EXISTS orders (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            order_number TEXT NOT NULL UNIQUE,      -- INV-001, INV-002, etc.
            customer_id INTEGER,                     -- NULL for walk-in customers
            subtotal REAL NOT NULL DEFAULT 0,        -- Sum of items before service charge
            service_charge REAL NOT NULL DEFAULT 0,  -- Service charge
            discount_amount REAL DEFAULT 0,          -- Discount if any
            total_amount REAL NOT NULL,              -- Final amount
            order_type TEXT NOT NULL,                -- 'dine-in or takeaway'
            payment_method TEXT NOT NULL,            -- 'cash', 'card', 'upi', 'cheque'
            payment_status TEXT DEFAULT 'completed', -- 'pending', 'completed', 'failed'
            order_status TEXT DEFAULT 'completed',   -- 'pending', 'completed', 'cancelled'
            notes TEXT,                              -- Special instructions
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        -- ===== ORDER ITEMS (Products in each order) =====
        CREATE TABLE IF NOT EXISTS order_items (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            order_id INTEGER NOT NULL,
            product_id INTEGER NOT NULL,
            quantity INTEGER NOT NULL,
            unit_price REAL NOT NULL,                -- Price at time of sale
            item_total REAL NOT NULL,                -- quantity * unit_price
            notes TEXT,                              -- Special instructions for item
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY(order_id) REFERENCES orders(id) ON DELETE CASCADE,
            FOREIGN KEY(product_id) REFERENCES products(id)
        );

        -- ==== ORDER SEQUENCE ======
        CREATE TABLE IF NOT EXISTS order_sequence (
            seq_date DATE PRIMARY KEY,
            counter INTEGER NOT NULL
        );
        "
    )?;

    Ok(conn)
}
