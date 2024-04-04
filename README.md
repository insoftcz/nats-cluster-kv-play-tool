# Usage
## NATS clustered Key-Value storage

```
NATS cluster POC tool

Usage: kv [OPTIONS]

Options:
  -s, --server-name <SERVER_NAME>
          [default: a]
          [possible values: a, b, c]

      --cmd <CMD>
          [default: read]

          Possible values:
          - read:           reads value of "foo" key in bucket "kv"
          - write:          writes some data to key "foo" in bucket "kv"
          - read-subscribe: listens for changes of "foo" in bucket "kv"
```

### Put data
```bash
cargo run --bin kv -- --server-name a --cmd write
```

### Read message
```bash
cargo run --bin kv -- --server-name a --cmd read
```

### Subscribe to data change
```bash
cargo run --bin kv -- --server-name a --cmd read-subscribe
```
