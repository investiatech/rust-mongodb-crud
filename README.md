# Rust Axum MongoDB CRUD API

A REST API built with **Rust**, **Axum**, and **MongoDB**, demonstrating CRUD operations using a layered architecture.

## Features

- RESTful API for managing items
- Create, read, update, and delete operations
- MongoDB integration using the official Rust driver
- Asynchronous request handling with Tokio
- JSON serialization and deserialization with Serde
- MongoDB ObjectId validation
- HTTP status codes for success and error responses
- Input validation to reject empty or whitespace-only item names
- Separation of HTTP handlers, business logic, and database access

## Tech Stack

- **Rust** — programming language
- **Axum** — HTTP routing and request handling
- **Tokio** — asynchronous runtime
- **MongoDB** — document database
- **Serde** — serialization and deserialization
- **dotenvy** — environment variable configuration

## Requirements

- Rust
- Cargo
- A running MongoDB instance

### Responsibilities

| Layer      | Responsibility                                                   |
|------------|------------------------------------------------------------------|
| Routes     | Define API endpoints and HTTP methods                            |
| Handler    | Process HTTP requests, validate input, and return HTTP responses |
| Service    | Implement application logic                                      |
| Repository | Execute database operations                                      |
| Model      | Represent MongoDB documents                                      |
| DTO        | Define request and response data structures                      |
| State      | Share application resources, including the database connection   |

## API Endpoints

The API exposes the following endpoints for managing items.

| Method   | Endpoint      | Description            |
|----------|---------------|------------------------|
| `POST`   | `/items`      | Create an item         |
| `GET`    | `/items`      | Retrieve all items     |
| `GET`    | `/items/{id}` | Retrieve an item by ID |
| `PUT`    | `/items/{id}` | Update an item         |
| `DELETE` | `/items/{id}` | Delete an item         |

## Configuration

The application uses environment variables to configure the MongoDB connection, database name, and HTTP server address.

Create a `.env` file in the project root:

```dotenv
MONGODB_URI=mongodb://localhost:27017
MONGODB_DATABASE=exampledb
SERVER_ADDRESS=0.0.0.0:8000
```

- `MONGODB_URI` — MongoDB connection string.
- `MONGODB_DATABASE` — name of the database used by the application.
- `SERVER_ADDRESS` — IP address and port on which the HTTP server listens. `0.0.0.0:8000` makes it listen on all network interfaces on port `8000`.

Ensure that the application's database configuration reads the corresponding environment variable.

## Running the Application

Clone the repository and navigate to the project directory.

```bash
git clone <repository-url>
cd crud
```

Check the project for compilation errors:

```bash
cargo check
```

Run the application:

```bash
cargo run
```

The application listens on the address configured in the project.

## Testing the API

The following examples demonstrate the CRUD endpoints using `curl`.

The examples assume the application is running at `http://localhost:8000` and MongoDB is available.

### 1. Create an item

```bash
curl -i -X POST http://localhost:8000/items \
  -H "Content-Type: application/json" \
  -d '{"name":"Laptop","description":"Development laptop"}'
```

Expected response: `201 Created`.

Save the returned `id` for the following requests.

### 2. Retrieve all items

```bash
curl -i http://localhost:8000/items
```

Expected response: `200 OK`.

### 3. Retrieve an item by ID

Replace `ITEM_ID` with the ID returned when creating the item.

```bash
curl -i http://localhost:8000/items/ITEM_ID
```

Expected response: `200 OK`.

### 4. Update an item

```bash
curl -i -X PUT http://localhost:8000/items/ITEM_ID \
  -H "Content-Type: application/json" \
  -d '{"name":"Updated Laptop","description":"Updated description"}'
```

Expected response: `200 OK`, with the updated item in the response body.

### 5. Delete an item

```bash
curl -i -X DELETE http://localhost:8000/items/ITEM_ID
```

Expected response: `204 No Content`.

## Learning Objectives

This project demonstrates:

- Building REST APIs in Rust
- Working with Axum handlers, routes, and shared state
- Using asynchronous Rust with Tokio
- Integrating MongoDB into a Rust application
- Designing a layered application architecture
- Separating DTOs, models, services, and repositories
- Handling errors and mapping them to HTTP status codes
- Validating incoming request data

## License

This project is licensed under the MIT License.

