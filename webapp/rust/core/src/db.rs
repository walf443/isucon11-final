use sqlx::{Executor, MySql, MySqlConnection, MySqlPool, Transaction};

pub type DBPool = MySqlPool;
pub type DBConn = MySqlConnection;
pub type TxConn<'c> = Transaction<'c, MySql>;

pub async fn get_db_conn() -> Result<DBPool, sqlx::Error> {
    let database = &std::env::var("MYSQL_DATABASE")
        .ok()
        .unwrap_or_else(|| "isucholar".to_owned());

    _get_db_conn(database).await
}

#[cfg(any(test, feature = "test"))]
pub async fn get_test_db_conn() -> Result<DBPool, sqlx::Error> {
    let database = &std::env::var("MYSQL_TEST_DATABASE").ok();

    match database {
        None => {
            panic!("please set MYSQL_TEST_DATABASE");
        }
        Some(database) => _get_db_conn(database).await,
    }
}

async fn _get_db_conn(database: &str) -> Result<DBPool, sqlx::Error> {
    let mysql_config = sqlx::mysql::MySqlConnectOptions::new()
        .host(
            &std::env::var("MYSQL_HOSTNAME")
                .ok()
                .unwrap_or_else(|| "127.0.0.1".to_owned()),
        )
        .port(
            std::env::var("MYSQL_PORT")
                .ok()
                .and_then(|port_str| port_str.parse().ok())
                .unwrap_or(3306),
        )
        .username(
            &std::env::var("MYSQL_USER")
                .ok()
                .unwrap_or_else(|| "isucon".to_owned()),
        )
        .password(
            &std::env::var("MYSQL_PASS")
                .ok()
                .unwrap_or_else(|| "isucon".to_owned()),
        )
        .database(database)
        .ssl_mode(sqlx::mysql::MySqlSslMode::Disabled);

    sqlx::mysql::MySqlPoolOptions::new()
        .max_connections(10)
        .after_connect(|conn, _| {
            Box::pin(async move {
                conn.execute("set time_zone = '+00:00'").await?;
                Ok(())
            })
        })
        .connect_with(mysql_config)
        .await
}
