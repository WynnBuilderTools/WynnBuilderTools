# USAGE

migrations use [sqlx-cli](https://crates.io/crates/sqlx-cli)

`cargo install sqlx-cli`

`sqlx database create`

With migrations folder and the pre-existing migration, run:
`sqlx database setup`

need:

- OpenSSL：`sudo apt install libssl-dev`
- pkg-config:`sudo apt install pkg-config`

To browse the DB you can use  [DB Browser for SQLite (DB4S)](https://github.com/sqlitebrowser/sqlitebrowser)
or [Sqlite3 Editor](https://marketplace.visualstudio.com/items?itemName=yy0931.vscode-sqlite3-editor) if you're using Visual Studio Code.
