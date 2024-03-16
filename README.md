# Page View Counter Real Time

This is a real-time page view counter application built with Rust and Actix. It uses WebSockets to provide real-time updates to all connected clients.

## Project Structure

- `src/main.rs`: This is the main entry point of the application. It defines the `AppState` and `UserSocket` structs, and the WebSocket handling logic.
- `src/static/js/index.js`: This is the client-side JavaScript file that handles WebSocket connections.
- `templates/`: This directory contains the Tera templates for the web pages.

## Dependencies

This project uses the following main dependencies:

- `actix`: A powerful, pragmatic, and extremely fast Rust framework for building asynchronous applications.
- `actix-web`: A powerful, pragmatic, and extremely fast web framework for Rust.
- `actix-web-actors`: A library for working with Actix actors in a web context.
- `tera`: A template engine inspired by Jinja2 and the Django template language.

## Running the Project

To run the project, use the following command:

```sh
cargo run
```
This will start the server, and you can access the application at http://localhost:8000.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.