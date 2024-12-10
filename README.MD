# Rust URL Shortener

A simple, high-performance URL shortener built using the Rust programming language. This application leverages the **Actix-web** framework for building the web server and **MongoDB** for database management.

## Features

- **Fast and lightweight**: Built with Rust for high performance.
- **RESTful API**: Easily integrate the shortener with other applications.
- **MongoDB integration**: Store and retrieve URL mappings efficiently.
- **Customizable short URLs**: Support for user-defined aliases.
- **Error handling**: Detailed responses for invalid or missing URLs.

---

## Getting Started

### Prerequisites

To build and run the project, ensure you have the following installed:

1. **Rust** (Latest stable version): [Install Rust](https://www.rust-lang.org/tools/install)
2. **MongoDB**: [Install MongoDB](https://www.mongodb.com/docs/manual/installation/)

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/rust-url-shortener.git
   cd rust-url-shortener
   ```

2. Install dependencies:

   ```bash
   cargo build
   ```

3. Configure the MongoDB connection in `.env`:

   ```bash
   MONGO_URI=mongodb://localhost:27017
   MONGO_DB=shortener
   ```

4. Run the application:
   ```bash
   cargo run
   ```

---

## Usage

### Endpoints

| Method | Endpoint       | Description                   |
| ------ | -------------- | ----------------------------- |
| `POST` | `/shorten`     | Shorten a given URL.          |
| `GET`  | `/{alias}`     | Redirect to the original URL. |
| `GET`  | `/healthcheck` | Check server health.          |

#### Example: Shortening a URL

**Request**  
`POST /shorten`

```json
{
	"url": "https://example.com",
	"alias": "example" // Optional
}
```

**Response**

```json
{
	"short_url": "http://localhost:8080/example"
}
```

#### Example: Redirecting

Visit `http://localhost:8080/example` in your browser, and it redirects to `https://example.com`.

---

## Project Structure

```
rust-url-shortener/
├── src/
│   ├── main.rs          # Entry point
│   ├── handlers.rs      # API route handlers
│   ├── models.rs        # Data models for URL and database
│   ├── db.rs            # MongoDB connection and queries
│   └── utils.rs         # Utility functions
├── .env                 # Environment variables
├── Cargo.toml           # Project dependencies and metadata
└── README.md            # Documentation
```

---

## Dependencies

- [Actix-web](https://actix.rs/): Web framework
- [MongoDB](https://docs.rs/mongodb): MongoDB driver
- [Serde](https://serde.rs/): JSON serialization/deserialization
- [dotenv](https://docs.rs/dotenv): Environment variable management

---

## Development

1. Run in development mode:

   ```bash
   cargo watch -x run
   ```

2. Run tests:
   ```bash
   cargo test
   ```

---

## Contribution

Contributions are welcome! Please fork the repository and submit a pull request.

1. Fork the repository.
2. Create a new branch: `git checkout -b feature-name`.
3. Commit your changes: `git commit -m "Add feature name"`.
4. Push to the branch: `git push origin feature-name`.
5. Open a pull request.

---

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

## Contact

For questions or suggestions, reach out to:

- **Author**: Your Name
- **Email**: your.email@example.com
- **GitHub**: [yourusername](https://github.com/yourusername)

---

Happy coding! 🚀
