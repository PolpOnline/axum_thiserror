# axum_thiserror

`axum_thiserror` is a library that offers a procedural macro to allow `thiserror` error types to be used as `axum` responses.

## Usage

Add the library to your current project using Cargo:
```bash
cargo add axum_thiserror
```

Then you can create a basic `thiserror` error:
```rust
#[derive(Error, Debug)]
pub enum UserCreateError {
  #[error("User {0} already exists")]
  UserAlreadyExists(String),
}
```

Now you can use `axum_thiserror` to implement `IntoResponse` on your error:
```rust
#[derive(Error, Debug, ErrorStatus)]
pub enum UserCreateError {
  #[error("User {0} already exists")]
  #[status(StatusCode::CONFLICT)]
  UserAlreadyExists(String),
}
```

## License

This project is licensed under the [MIT License](LICENSE).