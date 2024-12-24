# Rust Web Application

This is a Rust web application using Rocket, Diesel, PostgreSQL, and Redis.


## Prerequisites

- Rust
- Docker
- Docker Compose

## Setting Up

1. Clone the repository:
    ```sh
    git clone <repository-url>
    cd <repository-directory>
    ```

2. Build and run the Docker containers:
    ```sh
    docker-compose up --build
    ```

## Environment Variables

The following environment variables are used in the [docker-compose.yml](http://_vscodecontentref_/4) file:

- `DATABASE_URL`: URL for the PostgreSQL database.
- `ROCKET_DATABASES`: Configuration for Rocket databases.
- `RUST_BACKTRACE`: Enables backtrace for debugging.
- `SMTP_HOST`: SMTP server host.
- `SMTP_USERNAME`: SMTP server username.
- `SMTP_PASSWORD`: SMTP server password.

## Docker Setup

The [docker-compose.yml](http://_vscodecontentref_/5) file sets up the following services:

- **Redis**: Uses the `redis:latest` image.
- **App**: Builds the application from the current directory and sets up the environment variables, ports, and volumes.


Database Migrations
Database migrations are managed using Diesel. Migrations are located in the migrations directory.

To run migrations, use the following command:
diesel migration run

Running Tests
To run tests, use the following command:
cargo test

