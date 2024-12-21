use mysql::*;
use mysql::prelude::*;
use std::env;
use std::sync::{Arc, Once};
use std::sync::Mutex;

pub struct MysqlPool {
    pub pool: Arc<Mutex<Pool>>,
}

impl MysqlPool {
    pub fn new() -> Self {
        let url: String = env::var("MYSQL_DB_URI").expect("MYSQL_DB_URI must be set");
        let opts: Opts = Opts::from_url(&url).expect("MYSQL_DB_URI is invalid");
        let pool: Pool = Pool::new(opts).expect("Failed to create MySQL connection pool");
        MysqlPool {
            pool: Arc::new(Mutex::new(pool)),
        }
    }

    pub fn get_conn(&self) -> Result<PooledConn> {
        let pool: std::sync::MutexGuard<'_, Pool> = self.pool.lock().unwrap();
        pool.get_conn()
    }
}
