use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};

use sqlx::SqlitePool;

// Connect to our SQLite database.
pub async fn connect_db() -> Result<SqlitePool, sqlx::Error> {
    // Specify the database file and create it if it doesn't exist.
    let options = SqliteConnectOptions::new()
        .filename("students.db")
        .create_if_missing(true);

    // Create a connection pool.
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    println!("Successfully connected to SQLite database!");

    Ok(pool)
}

// Create the students table if it doesn't already exist.
pub async fn create_table(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS students (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            age INTEGER NOT NULL,
            course TEXT NOT NULL
        )",
    )
    .execute(pool)
    .await?;

    println!("Students table created successfully!");

    Ok(())
}
