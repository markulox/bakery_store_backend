
# Migration file management

## Initialization
1. To initialize new migration file *(for the Postgres database)* in the project, use command
    ```bash
    sea-orm-cli migrate init -u "postgres://$DB_USERNAME:$DB_PASSWORD@$DB_HOST/$DB_NAME" 
    ```
2. Don't forget to add the generated migration files (usually stays in `migration` directory) into the workspace members in the `Cargo.toml` file to let the **clippy** ease us while we writting migration files.
    ```toml
    ...
    [workspace]
    members = [".", "migration"]
    ...
    ```

## Edit migration files and run migrate
1. In `MigrationTrait`, there are 2 functions: `up()` and `down()`. Code a table creation in `up()` and undo all stuff in `down()`

2. Don't forget to add features in `migration/Cargo.toml`. The example below use the **`Postgres`** as a database server.
    ```toml
    ...
    [dependencies.sea-orm-migration]
    version = "1.1.0"
    features = [
        "sqlx-postgres",
        "runtime-tokio-native-tls",
    ]
    ...
    ```

3. Then execute the command
    ```bash
    sea-orm-cli migrate refresh
    ```
    There also other actions available for the command `sea-orm-cli migrate` such as
    - `up` to apply all pending migrations
    - `down` to rollback the applied migrations 
    
    ** For more information you can check at `sea-orm-cli migrate --help`


## Entities Generation
