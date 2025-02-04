use sqlx::PgPool;

use crate::task::Task;

pub async fn create_new_task(pool: &PgPool, title: String) -> Result<(), sqlx::Error> {
    sqlx::query(
        "
INSERT INTO tasks (title)
VALUES ($1);
    ",
    )
    .bind(title)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn toggle_task(pool: &PgPool, id: i32) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE tasks SET completed = NOT completed where id = $1;")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn delete_task(pool: &PgPool, id: i32) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM tasks where ID = $1;")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn get_all_tasks(pool: &PgPool) -> Result<Vec<Task>, sqlx::Error> {
    let tasks = sqlx::query_as!(Task, "SELECT * FROM tasks ORDER BY id;")
        .fetch_all(pool)
        .await?;

    Ok(tasks)
}
