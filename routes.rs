use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post, put},
};

use sqlx::SqlitePool;

use crate::student::{
    Student, StudentInput, create_student, delete_student, get_all_students, get_student_by_id,
    update_student,
};

// CREATE: POST /students
async fn add_student(
    State(pool): State<SqlitePool>,
    Json(input): Json<StudentInput>,
) -> Result<(StatusCode, Json<Student>), (StatusCode, String)> {
    let student = create_student(&pool, input).await.map_err(|error| {
        eprintln!("Database error: {}", error);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Could not register student".to_string(),
        )
    })?;

    Ok((StatusCode::CREATED, Json(student)))
}

// READ: GET /students
async fn get_students(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<Student>>, (StatusCode, String)> {
    let students = get_all_students(&pool).await.map_err(|error| {
        eprintln!("Database error: {}", error);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Could not retrieve students".to_string(),
        )
    })?;

    Ok(Json(students))
}

// READ: GET /students/{id}
async fn get_student(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<Student>, (StatusCode, String)> {
    let student = get_student_by_id(&pool, id).await.map_err(|error| {
        eprintln!("Database error: {}", error);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Could not retrieve student".to_string(),
        )
    })?;

    match student {
        Some(student) => Ok(Json(student)),
        None => Err((
            StatusCode::NOT_FOUND,
            format!("Student with ID {} was not found", id),
        )),
    }
}

// UPDATE: PUT /students/{id}
async fn edit_student(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(input): Json<StudentInput>,
) -> Result<Json<Student>, (StatusCode, String)> {
    let student = update_student(&pool, id, input).await.map_err(|error| {
        eprintln!("Database error: {}", error);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Could not update student".to_string(),
        )
    })?;

    match student {
        Some(student) => Ok(Json(student)),
        None => Err((
            StatusCode::NOT_FOUND,
            format!("Student with ID {} was not found", id),
        )),
    }
}

// DELETE: DELETE /students/{id}
async fn remove_student(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<StatusCode, (StatusCode, String)> {
    let deleted = delete_student(&pool, id).await.map_err(|error| {
        eprintln!("Database error: {}", error);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Could not delete student".to_string(),
        )
    })?;

    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err((
            StatusCode::NOT_FOUND,
            format!("Student with ID {} was not found", id),
        ))
    }
}

// Register all API endpoints.
pub fn create_routes(pool: SqlitePool) -> Router {
    Router::new()
        .route("/students", get(get_students).post(add_student))
        .route(
            "/students/{id}",
            get(get_student).put(edit_student).delete(remove_student),
        )
        .with_state(pool)
}
