use diesel::prelude::MysqlConnection;
use ops::zone::{ analyse_zone_data, display_zone_mappings, generate_zone_mappings };
use std::collections::HashMap;
use models::zones::ZoneIdMapping;

mod models;
mod schema;
mod ops;

pub fn analyse(peq_db: &mut MysqlConnection, takp_db: &mut MysqlConnection) {
    let zone_list: HashMap<String, ZoneIdMapping> = generate_zone_mappings(peq_db, takp_db);
    display_zone_mappings(&zone_list);

    println!("Zone data SQL:\n{}", analyse_zone_data(&zone_list, peq_db, takp_db));
}
