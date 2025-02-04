use std::{
    env,
    io::{self, stdin, Write},
};

use dotenv::dotenv;
use rust_todolist_learning::{
    add, remove, toggle,
    ui::{display_help_content, display_welcome_message},
    utils::remove_first_word,
    view,
};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("❌ - Please add a DATABASE_URL value into your .env file");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    display_welcome_message();

    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut input = String::new();

        stdin()
            .read_line(&mut input)
            .expect("An error occured. Please try again");

        input = input.trim().to_string();

        let command = input.split_whitespace().next().unwrap_or("help");

        match command {
            "add" => add(&pool, remove_first_word(&input)).await,
            "toggle" => toggle(&pool, remove_first_word(&input)).await,
            "remove" => remove(&pool, remove_first_word(&input)).await,
            "view" => view(&pool).await,
            _ => display_help_content(),
        }
    }
}
