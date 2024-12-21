use std::sync::Arc;
use std::sync::Mutex;
use mysql::*;

pub struct MysqlRepository {
    pub pool: Arc<Mutex<Pool>>,
}


impl MysqlRepository {
    pub fn new(mysql_pool: Arc<Mutex<Pool>>) -> Self {
        MysqlRepository { pool: mysql_pool }
    }
}