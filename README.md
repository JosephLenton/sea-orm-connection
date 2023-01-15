<div align="center">
  <h1>
    Tea Orm ☕<br>
    helper utilities for Sea Orm
  </h1>

  [![crate](https://img.shields.io/crates/v/tea-orm.svg)](https://crates.io/crates/tea-orm)
  [![docs](https://docs.rs/tea-orm/badge.svg)](https://docs.rs/tea-orm)
</div>

This is a crate for containing lots of helper functions when working with Sea Orm.
Primarily for when writing tests.

For example utilities for quickly making a DB connection, and creating a new blank database for use within a test.

# Features

These are Sea Orm specific options for it's runtime.

 * `runtime-actix-native-tls` - Sets Sea-Orm Migrations to use this runtime.
 * `runtime-actix-rustls` - Sets Sea-Orm Migrations to use this runtime.
 * `runtime-async-std-native-tls` - Sets Sea-Orm Migrations to use this runtime.
 * `runtime-async-std-rustls` - Sets Sea-Orm Migrations to use this runtime.
 * `runtime-tokio-native-tls` **Default** - Sets Sea-Orm Migrations to use this runtime.
 * `runtime-tokio-rustls` - Sets Sea-Orm Migrations to use this runtime.

# Limitations

This currently **only supports PostgresSQL.**

Note that this is a work in progress.
