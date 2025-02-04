use database::{create_new_task, delete_task, get_all_tasks, toggle_task};
use sqlx::PgPool;
use ui::{display_sqlx_error_message, display_todos};
use utils::convert_string_to_int_or_display_error_message;

mod database;
mod task;
pub mod ui;
pub mod utils;

pub async fn view(pool: &PgPool) {
    match get_all_tasks(pool).await {
        Ok(tasks) => display_todos(&tasks),
        Err(e) => display_sqlx_error_message(e),
    }
}
pub async fn remove(pool: &PgPool, id_string: String) {
    let id = match convert_string_to_int_or_display_error_message(id_string) {
        Ok(value) => value,
        Err(_) => return,
    };

    match delete_task(pool, id).await {
        Ok(_) => view(&pool).await,
        Err(e) => display_sqlx_error_message(e),
    }
}

pub async fn toggle(pool: &PgPool, id_string: String) {
    let id = match convert_string_to_int_or_display_error_message(id_string) {
        Ok(value) => value,
        Err(_) => return,
    };
    match toggle_task(pool, id).await {
        Ok(_) => view(&pool).await,
        Err(e) => display_sqlx_error_message(e),
    }
}

pub async fn add(pool: &PgPool, value: String) {
    match create_new_task(pool, value).await {
        Ok(_) => view(&pool).await,
        Err(e) => display_sqlx_error_message(e),
    }
}
