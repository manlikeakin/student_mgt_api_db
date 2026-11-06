use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

// The structure of a student record.
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Student {
    pub id: i64,
    pub name: String,
    pub age: i64,
    pub course: String,
}

// The information we expect when creating or updating a student.
#[derive(Debug, Deserialize)]
pub struct StudentInput {
    pub name: String,
    pub age: i64,
    pub course: String,
}

// CREATE: Register a new student.
pub async fn create_student(
    pool: &SqlitePool,
    input: StudentInput,
) -> Result<Student, sqlx::Error> {
    let student = sqlx::query_as::<_, Student>(
        "INSERT INTO students (name, age, course)
         VALUES (?, ?, ?)
         RETURNING id, name, age, course",
    )
    .bind(input.name)
    .bind(input.age)
    .bind(input.course)
    .fetch_one(pool)
    .await?;

    Ok(student)
}

// READ: Retrieve all students.
pub async fn get_all_students(pool: &SqlitePool) -> Result<Vec<Student>, sqlx::Error> {
    let students = sqlx::query_as::<_, Student>("SELECT id, name, age, course FROM students")
        .fetch_all(pool)
        .await?;

    Ok(students)
}

// READ: Retrieve one student by ID.
pub async fn get_student_by_id(
    pool: &SqlitePool,
    student_id: i64,
) -> Result<Option<Student>, sqlx::Error> {
    let student = sqlx::query_as::<_, Student>(
        "SELECT id, name, age, course
         FROM students
         WHERE id = ?",
    )
    .bind(student_id)
    .fetch_optional(pool)
    .await?;

    Ok(student)
}

// UPDATE: Modify an existing student's details.
pub async fn update_student(
    pool: &SqlitePool,
    student_id: i64,
    input: StudentInput,
) -> Result<Option<Student>, sqlx::Error> {
    let student = sqlx::query_as::<_, Student>(
        "UPDATE students
         SET name = ?, age = ?, course = ?
         WHERE id = ?
         RETURNING id, name, age, course",
    )
    .bind(input.name)
    .bind(input.age)
    .bind(input.course)
    .bind(student_id)
    .fetch_optional(pool)
    .await?;

    Ok(student)
}

// DELETE: Remove a student from the database.
pub async fn delete_student(pool: &SqlitePool, student_id: i64) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM students WHERE id = ?")
        .bind(student_id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}

