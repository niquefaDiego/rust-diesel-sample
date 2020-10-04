# Rust diesel sample

This repo aims to show via example how to use rust's (diesel)[http://diesel.rs/] ORM.

## Instructions

### Install depencies

Instal diesel_cli

```
cargo install diesel_cli
```

### Setup database connection string

Create a file `.env` in the project's root folder and specify the database connection string as follows:

```
DATABASE_URL=postgres://username:password@localhost/diesel_demo
```

### Some useful diesel scripts

Create a new migration:
```
diesel migration generate create_posts
```

Run migrations (only runs migrations not yet ran, and also generate `src/schema.rs`):
```
diesel migration run
```

Redo last run migration:
```
diesel migration redo
```
