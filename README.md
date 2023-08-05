To reproduce the error

```shell
docker compose up -d

// wait for docker to start

cargo run --bin migrate_fresh

cargo run --bin has_decimal
```

You should get out like

> Caused by:
> error occurred while decoding column "price": mismatched types; Rust type `core::option::Option<rust_decimal::decimal::Decimal>` (as SQL type `DECIMAL`) is not compatible with SQL type `DECIMAL`
>
> Location:
> app/src/bin/has_decimal.rs:31:17

To start over run the following, then run the steps above

```shell
docker compose kill readyset; sudo rm -rf data/readyset
```
