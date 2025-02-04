use crate::task::Task;

pub fn display_welcome_message() {
    println!("🎯 Welcome to Rust Todo List ©Marco Musiol - Use \"help\" for more information");
}

pub fn display_todos(tasks: &Vec<Task>) {
    println!("\n🎯 Your Tasks:");

    for task in tasks {
        let status = if task.completed { "[✅]" } else { "[  ]" };

        println!("{} {} - {}", task.id, status, task.title);
    }
    println!();
}

pub fn display_help_content() {
    println!(
        "
👉 \"add [TODO]\" - Add a new task
👀 \"view\" - Display all tasks
🎛️  \"toggle [ID]\" - Toggle a task (completed/uncompleted)
🗑️  \"remove [ID]\" - Removes a task completly from the list
"
    );
}

pub fn display_sqlx_error_message(error: sqlx::Error) {
    println!("❌ An error occured: {}", error);
}
