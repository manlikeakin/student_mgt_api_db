mod database;
mod routes;
mod student;

use database::{connect_db, create_table};
use routes::create_routes;

#[tokio::main]
async fn main() {
    // 1. Connect to SQLite.
    let pool = connect_db().await.expect("Database connection failed");

    // 2. Create the students table if it doesn't exist.
    create_table(&pool)
        .await
        .expect("Could not create students table");

   

    // 3. Build our API routes.
    let app = create_routes(pool);

    // 4. Start the HTTP server.
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Could not start server");

    println!("Student API is running!");
    println!("Visit http://127.0.0.1:3000/students");

    axum::serve(listener, app).await.expect("Server error");
}
