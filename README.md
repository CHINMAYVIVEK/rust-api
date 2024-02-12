
# Rust API with PostgreSQL Database

This project implements a CRUD (Create, Read, Update, Delete) API in Rust using Actix-web framework and PostgreSQL as the database. The API allows users to perform CRUD operations on a resource (e.g., users, products, etc.).

## Features

- Create, Read, Update, and Delete operations on a resource.
- Secure endpoints with JWT (JSON Web Tokens) authentication.
- Utilizes asynchronous programming with Tokio runtime.
- Database integration with PostgreSQL using the `sqlx` crate.
- Dockerized deployment for easy scaling and management.

## Requirements

- Rust programming language (https://www.rust-lang.org/)
- Docker (https://www.docker.com/)
- Docker Compose (https://docs.docker.com/compose/)

## Setup

1. Clone the repository:

```bash
git clone https://github.com/your_username/rust-api.git
cd rust-api
```

2. Install dependencies:

```bash
cargo build
```

3. Set up the PostgreSQL database. You can use Docker Compose to run a PostgreSQL container:

```bash
docker-compose up -d postgres
```

4. Run database migrations to create tables:

```bash
cargo run --bin migrations
```

5. Start the API server:

```bash
cargo run --bin api
```

The API server will start on http://localhost:8080.

## API Endpoints

- `GET /resource`: Retrieve all resources.
- `GET /resource/{id}`: Retrieve a specific resource by ID.
- `POST /resource`: Create a new resource.
- `PUT /resource/{id}`: Update a resource by ID.
- `DELETE /resource/{id}`: Delete a resource by ID.

## Docker Deployment

To deploy the application using Docker, follow these steps:

1. Build the Docker image:

```bash
docker build -t rust-api .
```

2. Run the Docker container:

```bash
docker run -d --name rust-api -p 8080:8080 rust-api
```

The API will be accessible at http://localhost:8080.

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request for any improvements or additional features.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---