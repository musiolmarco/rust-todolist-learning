# 🚀 Rust Async Todo List (Learning Project)

This is a **learning project** where I built a simple **Todo List** application using **Rust**, **Tokio (async runtime)**, **SQLx**, and **PostgreSQL**. The goal of this project was to explore **asynchronous programming**, database integration, and best practices in Rust development.

⚠️ **Note:** This is not a production-ready application. It was built purely for learning purposes.

---

## 📌 Features

- ✅ **Create, Read, Update, Delete (CRUD)** operations for tasks
- ⏳ **Asynchronous database queries** with SQLx
- 🗄️ **PostgreSQL** as a database backend
- 🔄 **Task completion toggle** (`completed` field switches between `true` and `false`)
- ⚡ **Efficient database connection pooling** with `PgPool`
- 🛠️ **Rust best practices** for error handling and structured project setup

---

## 🛠 Technologies Used

- **Rust** 🦀
- **Tokio** (async runtime) ⏳
- **SQLx** (PostgreSQL async ORM)
- **PostgreSQL** (relational database)
- **dotenv** (managing database credentials)

---

## 📚 What I Learned

This project helped me deepen my understanding of several Rust concepts and libraries:

### 🦀 Rust Basics (Refresher)

- **Ownership & Borrowing** rules
- **Pattern matching** for error handling (`match`, `if let Err(e)`, `?` operator)
- **Structs & Enums** for data modeling
- **Traits & Generics** for cleaner, reusable code

### ⚡ Async & Tokio

- **How async/await works in Rust** (no built-in runtime, needs external crate)
- **Managing async database queries** efficiently
- **Concurrency with Tokio runtime** (executing multiple DB queries asynchronously)

### 🗄️ SQLx & PostgreSQL

- **How to execute database queries asynchronously**
- **Using `$1, $2, ...` instead of `?` for PostgreSQL**
- **Running database migrations** with `sqlx migrate add` & `sqlx migrate run`
- **Handling SQL errors like `relation does not exist`, `operator does not exist`**

### 🛠️ Rust Best Practices

- **Separation of concerns** (keeping logic and DB interactions separate)
- **Proper error handling** (`Result<>`, `Option<>`, custom error types)
- **Logging & Debugging techniques** with `println!` & `eprintln!`
- **Environment variable management** with `.env` and `dotenv` crate

---

## 📦 Setup Instructions

### 1️⃣ Prerequisites

Make sure you have the following installed:

- **Rust** (latest stable version) → [Install Rust](https://www.rust-lang.org/tools/install)
- **PostgreSQL** → [Download PostgreSQL](https://www.postgresql.org/download/)

### 2️⃣ Clone the Repository

```sh
    git clone https://github.com/yourusername/rust-todo-list.git
    cd rust-todo-list
```

### 3️⃣ Set up Environment Variables

Create a `.env` file and add your **PostgreSQL connection string**:

```sh
DATABASE_URL=postgres://user:password@localhost/todo_db
```

### 4️⃣ Run Database Migrations

```sh
sqlx migrate run
```

### 5️⃣ Build & Run the Project

```sh
cargo run
```

---

## 🔥 Available Functions

| Function          | Description                              |
| ----------------- | ---------------------------------------- |
| `create_new_task` | Inserts a new task into the database     |
| `get_all_tasks`   | Retrieves all tasks                      |
| `toggle_task`     | Toggles the `completed` status of a task |
| `delete_task`     | Deletes a task from the database         |

---

## ⚠️ Notes & Limitations

- **Not production-ready** (built for learning purposes only)
- **No authentication/security measures** implemented
- **Basic error handling**, but could be improved

---

## 🤝 Contributing

This project was built for **learning Rust & async programming**, so contributions are not needed. However, if you have feedback or suggestions, feel free to reach out! 🚀

---

## 📜 License

This project is open-source and free to use for learning purposes.
