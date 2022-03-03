# clickhouse-migrator

### Note: Still under development

Rust library for running and versioning clickhouse MigrationFiles.

This is currently compatible with the latest version (21.9) of clickhouse.

## Installation

N/A

## Usage

```sh-session
clickhouse-migrate [command]

clickhouse-migrate [command] help
```

## Commands

- [`setup`](#setup)
- [`migrate`](#export)

### Setup

#### Create a new config

```sh-session
clickhouse-migrate setup init
```

#### View the current config setup

```sh-session
clickhouse-migrate setup view
```

#### Set up fdb cluster file

```sh-session
clickhouse-migrate setup set --uri http://localhost:8083 --migrations ./migrations
```

### Migrate

#### Creating a migration

Creating a migration with a name

```sh-session
RUST_LOG=info clickhouse-migrate migrate make --name "name of my migration"
```

#### Running the latest set of migrations

```sh-session
RUST_LOG=info clickhouse-migrate migrate latest
```

## Currently known to be unsupported

- Will always create a new configuration and wont check if there is one present