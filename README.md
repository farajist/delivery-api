# delivery-api: rust in the backend

The project is proof-of-concept on how to implement web services using rust and its web development's toolchain (actix-web, tokio ...etc.)

## Running

```shell
cargo build --release

./target/release/delivery_api 
```
To test the app endpoint:

```shell
curl -v http://127.0.0.1:8080
```
