# gRPC Demo in Rust

demo application to use gRPC in Rust. [reference](https://betterprogramming.pub/building-a-grpc-server-with-rust-be2c52f0860e)

## Notes
- requirements: `protobuf`, `grpc-cli`
   ```shell
   $ sudo pacman -S protobuf grpc-cli
   ```
- test using `grpc-cli`:
   ```shell
   $ grpc_cli call localhost:50051 bookstore.BookStore.GetBook "id:'test-book-id'"
   connecting to localhost:50051
   Received initial metadata from server:
   date : Tue, 07 Jun 2022 07:19:08 GMT
   id: "test-book-id"
   name: "Zero to One"
   author: "Peter"
   year: 2014
   Rpc succeeded with OK status
   ```