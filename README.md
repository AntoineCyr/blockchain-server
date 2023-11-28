# Local Blockchain
- Multithreading
- Client server rpc connection

## usage

- Start blockchain:
  ```sh
  cargo run start-node
  ```
- Create wallet
  ```
  cargo run create-account  <address> <amount>
  ```
- Transfer funds:
  ```
  cargo run transfer <from> <to> <amount>
  ```
- Get user balance:
  ```
  cargo run balance <address>
  ```

