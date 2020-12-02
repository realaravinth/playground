use deadpool_postgres::{Client, Pool};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;

fn main() {
    let statement = include_str!("./add_user.sql");
    let statement = statement.replace("$table_fields", &User::sql_table_fields());

    let client: Client = db_pool.get().await?;
    let command = client.prepare(&statement).await?;

    let new_user = client
        .query(&command, &[&self.username, &self.email_id, &self.password])
        .await?
        .iter()
        .map(|row| User::from_row_ref(row).unwrap())
        .collect::<Vec<User>>()
        .pop()
        .unwrap();
    Ok(new_user)
}
