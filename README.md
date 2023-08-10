
# MySQL bool type issue

 mysql treats `bool` as an alias for `tinyint(1)`

So the `view_type` table look like this :

```sql
CREATE TABLE `view_type` (
  `channel_id` int NOT NULL,
  `view_type` varchar(255) NOT NULL,
  `enabled` tinyint(1) NOT NULL,
  PRIMARY KEY (`channel_id`,`view_type`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci
```

To reproduce

```shell

// start mysql and readyset in docker
docker compose up -d

// wait for services to start

// crate schema
cargo run --bin migrate_fresh

// insert a record directly in mysql
cargo run --bin view_type_insert

// select the record from mysql
cargo run --bin view_type_mysql_select

// select the records from readyset
cargo run --bin view_type_readyset_select

```

observe the error

```
Error: Query Error: error communicating with database: expected to read 4 bytes, got 0 bytes at EOF

Caused by:
    error communicating with database: expected to read 4 bytes, got 0 bytes at EOF

Location:
    app/src/bin/view_type_readyset_select.rs:12:17
```

and the readyset logs in docker

```
readyset_seaorm_issues-readyset-1  | 2023-08-10T02:10:09.134045Z ERROR connection{addr=172.25.0.1:56330}:connection{addr=172.25.0.1:56330}: readyset_mysql::backend: encountered error while attempting to write column packet err=tried to use 1 as MYSQL_TYPE_BIT
```

&nbsp;

&nbsp;

&nbsp;

&nbsp;

# MySQL DECIMAL type issue

https://github.com/readysettech/readyset/issues/143

**NOTE: fixed using https://github.com/cameronbraid/sqlx/tree/decimal_compatibility_lenient and this repo is configured to use the fix**

To reproduce the error

```shell
// edit /Cargo.toml and comment out the sqlx patch

// start mysql and readyset in docker
docker compose up -d

// wait for services to start

// crate schema
cargo run --bin migrate_fresh

// insert a record directly in mysql
cargo run --bin has_decimal_seaorm_mysql_insert

// select the record from mysql
cargo run --bin has_decimal_seaorm_mysql_select

// select the record from mysql
cargo run --bin has_decimal_seaorm_readyset_select

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

---
