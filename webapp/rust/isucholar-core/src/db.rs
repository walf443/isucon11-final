use sqlx::{Executor, MySql, MySqlPool, Transaction};

pub type DBPool = MySqlPool;
pub type TxConn<'c> = Transaction<'c, MySql>;
use futures::StreamExt as _;

/*
 * sqlx の MySQL ドライバには
 *
 * - commit()/rollback() していないトランザクション (sqlx::Transaction) が drop される
 *   - このとき drop 後に自動的に ROLLBACK が実行される
 * - fetch_one()/fetch_optional() のように MySQL からのレスポンスを最後まで読まない関数を最後に使っ
 *   ている
 *
 * の両方を満たす場合に、sqlx::Transaction が drop された後に panic する不具合がある。
 * panic しても正常にレスポンスは返されておりアプリケーションとしての動作には影響無い。
 *
 * この不具合を回避するため、fetch() したストリームを最後まで詠み込むような
 * fetch_one()/fetch_optional() をここで定義し、アプリケーションコードではトランザクションに関して
 * これらの関数を使うことにする。
 *
 * 上記のワークアラウンド以外にも、sqlx::Transaction が drop される前に必ず commit()/rollback() を
 * 呼ぶように気をつけて実装することでも不具合を回避できる。
 *
 * - https://github.com/launchbadge/sqlx/issues/1078
 * - https://github.com/launchbadge/sqlx/issues/1358
 */

pub async fn get_db_conn() -> Result<DBPool, sqlx::Error> {
    let database = &std::env::var("MYSQL_DATABASE")
        .ok()
        .unwrap_or_else(|| "isucholar".to_owned());

    _get_db_conn(database).await
}

#[cfg(test)]
pub async fn get_test_db_conn() -> Result<DBPool, sqlx::Error> {
    let database = &std::env::var("MYSQL_TEST_DATABASE")
        .ok();

    match database {
        None => {
            panic!("please set MYSQL_TEST_DATABASE");
        }
        Some(database) => {
            return _get_db_conn(&database).await
        }
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
    let pool = sqlx::mysql::MySqlPoolOptions::new()
        .max_connections(10)
        .after_connect(|conn| {
            Box::pin(async move {
                conn.execute("set time_zone = '+00:00'").await?;
                Ok(())
            })
        })
        .connect_with(mysql_config)
        .await;

    pool
}

pub async fn fetch_one_as<'q, 'c, O>(
    query: sqlx::query::QueryAs<'q, sqlx::MySql, O, sqlx::mysql::MySqlArguments>,
    tx: &mut sqlx::Transaction<'c, sqlx::MySql>,
) -> sqlx::Result<O>
where
    O: 'q + Send + Unpin + for<'r> sqlx::FromRow<'r, sqlx::mysql::MySqlRow>,
{
    match fetch_optional_as(query, tx).await? {
        Some(row) => Ok(row),
        None => Err(sqlx::Error::RowNotFound),
    }
}

pub async fn fetch_one_scalar<'q, 'c, O>(
    query: sqlx::query::QueryScalar<'q, sqlx::MySql, O, sqlx::mysql::MySqlArguments>,
    tx: &mut sqlx::Transaction<'c, sqlx::MySql>,
) -> sqlx::Result<O>
where
    O: 'q + Send + Unpin,
    (O,): for<'r> sqlx::FromRow<'r, sqlx::mysql::MySqlRow>,
{
    match fetch_optional_scalar(query, tx).await? {
        Some(row) => Ok(row),
        None => Err(sqlx::Error::RowNotFound),
    }
}

pub async fn fetch_optional_as<'q, 'c, O>(
    query: sqlx::query::QueryAs<'q, sqlx::MySql, O, sqlx::mysql::MySqlArguments>,
    tx: &mut sqlx::Transaction<'c, sqlx::MySql>,
) -> sqlx::Result<Option<O>>
where
    O: Send + Unpin + for<'r> sqlx::FromRow<'r, sqlx::mysql::MySqlRow>,
{
    let mut rows = query.fetch(tx);
    let mut resp = None;
    while let Some(row) = rows.next().await {
        let row = row?;
        if resp.is_none() {
            resp = Some(row);
        }
    }
    Ok(resp)
}

pub async fn fetch_optional_scalar<'q, 'c, O>(
    query: sqlx::query::QueryScalar<'q, sqlx::MySql, O, sqlx::mysql::MySqlArguments>,
    tx: &mut sqlx::Transaction<'c, sqlx::MySql>,
) -> sqlx::Result<Option<O>>
where
    O: 'q + Send + Unpin,
    (O,): for<'r> sqlx::FromRow<'r, sqlx::mysql::MySqlRow>,
{
    let mut rows = query.fetch(tx);
    let mut resp = None;
    while let Some(row) = rows.next().await {
        let row = row?;
        if resp.is_none() {
            resp = Some(row);
        }
    }
    Ok(resp)
}
