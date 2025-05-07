use diesel::prelude::*;
use dotenvy::dotenv;
use peq_takp_pop_merge::analyse;
use std::env;

fn read_env_var(env_var: &str) -> String {
    match env::var(env_var) {
        Ok(var) => var,
        Err(err) => panic!("Environment variable \"{}\" must be set:\n {}", env_var, err),
    }
}

fn establish_connection(url: String) -> MysqlConnection {
    match MysqlConnection::establish(&url) {
        Ok(mysqlconnection) => mysqlconnection,
        Err(err) => panic!("Failed to establish connection to database at {}: {}", &url, err),
    }
}

fn main() {
    dotenv().ok();
    let peq_database_url = read_env_var("PEQ_DATABASE_URL");
    let takp_database_url = read_env_var("TAKP_DATABASE_URL");

    let mut peq = establish_connection(peq_database_url);
    let mut takp = establish_connection(takp_database_url);

    analyse(&mut peq, &mut takp);
}
