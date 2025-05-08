use diesel::prelude::MysqlConnection;
use crate::models::{ peq, takp, zones::ZoneIdMapping };
use std::collections::HashMap;

pub fn generate_zone_mappings(
    peq_db: &mut MysqlConnection,
    takp_db: &mut MysqlConnection
) -> HashMap<String, ZoneIdMapping> {
    let peq_zones = peq::Zone::get_list(peq_db);
    let takp_zones = takp::Zone::get_list(takp_db);

    let mut zone_list: HashMap<String, ZoneIdMapping> = HashMap::new();

    for takp_zone in &takp_zones {
        let mut peq_found = false;
        for peq_zone in &peq_zones {
            if peq_zone.short_name == takp_zone.short_name && peq_zone.min_expansion < 5 {
                if zone_list.contains_key(&takp_zone.short_name) {
                    panic!("Zone {} already in zone list", takp_zone.short_name);
                }
                zone_list.insert(
                    takp_zone.short_name.clone(),
                    ZoneIdMapping::new(peq_zone.id, takp_zone.id)
                );
                peq_found = true;
            }
        }
        if !peq_found {
            panic!("Zone {} not found in peq data", takp_zone.short_name);
        }
    }

    zone_list
}

pub fn display_zone_mappings(zone_list: &HashMap<String, ZoneIdMapping>) {
    for short_name in zone_list.keys() {
        match zone_list.get(short_name) {
            Some(mapping) =>
                println!("{}\tpeq_id:{}\ttakp_id:{}", short_name, mapping.peq_id, mapping.takp_id),
            None => panic!("Unable to retrieve data from zone_list with key: {}", short_name),
        }
    }
}

pub fn analyse_zone_data(
    zone_mapping: &HashMap<String, ZoneIdMapping>,
    peq_db: &mut MysqlConnection,
    takp_db: &mut MysqlConnection
) -> String {
    let peq_zones = peq::Zone::get_list(peq_db);
    let takp_zones = takp::Zone::get_list(takp_db);

    let mut query = String::new();

    for short_name in zone_mapping.keys() {
        let (peq_id, takp_id) = match zone_mapping.get(short_name) {
            Some(zone_ids) => (zone_ids.peq_id, zone_ids.takp_id),
            None => panic!("No mapping with short_name {} in zone mappings", short_name),
        };
        let peq_zone = peq_zones
            .iter()
            .find(|zone| zone.id == peq_id)
            .unwrap(); // TODO
        let takp_zone = takp_zones
            .iter()
            .find(|zone| zone.id == takp_id)
            .unwrap(); // TODO

        let mut updates: Vec<(&str, String)> = Vec::new();

        if peq_zone.safe_x != takp_zone.safe_x {
            updates.push(("safe_x", takp_zone.safe_x.to_string()));
        }

        if peq_zone.safe_y != takp_zone.safe_y {
            updates.push(("safe_y", takp_zone.safe_y.to_string()));
        }

        if peq_zone.safe_z != takp_zone.safe_z {
            updates.push(("safe_z", takp_zone.safe_z.to_string()));
        }

        if peq_zone.safe_heading != takp_zone.safe_heading {
            updates.push(("safe_heading", takp_zone.safe_heading.to_string()));
        }

        if peq_zone.sky != takp_zone.sky {
            updates.push(("sky", takp_zone.sky.to_string()));
        }

        if peq_zone.minclip != takp_zone.minclip {
            updates.push(("minclip", takp_zone.minclip.to_string()));
        }

        if peq_zone.maxclip != takp_zone.maxclip {
            updates.push(("maxclip", takp_zone.maxclip.to_string()));
        }

        if peq_zone.fog_minclip != takp_zone.fog_minclip {
            updates.push(("fog_minclip", takp_zone.fog_minclip.to_string()));
        }

        if peq_zone.fog_maxclip != takp_zone.fog_maxclip {
            updates.push(("fog_maxclip", takp_zone.fog_maxclip.to_string()));
        }

        if peq_zone.fog_red != takp_zone.fog_red {
            updates.push(("fog_red", takp_zone.fog_red.to_string()));
        }

        if peq_zone.fog_green != takp_zone.fog_green {
            updates.push(("fog_green", takp_zone.fog_green.to_string()));
        }

        if peq_zone.fog_blue != takp_zone.fog_blue {
            updates.push(("fog_blue", takp_zone.fog_blue.to_string()));
        }

        if peq_zone.fog_red1 != takp_zone.fog_red1 {
            updates.push(("fog_red1", takp_zone.fog_red1.to_string()));
        }

        if peq_zone.fog_green1 != takp_zone.fog_green1 {
            updates.push(("fog_green1", takp_zone.fog_green1.to_string()));
        }

        if peq_zone.fog_blue1 != takp_zone.fog_blue1 {
            updates.push(("fog_blue1", takp_zone.fog_blue1.to_string()));
        }

        if peq_zone.fog_minclip1 != takp_zone.fog_minclip1 {
            updates.push(("fog_minclip1", takp_zone.fog_minclip1.to_string()));
        }

        if peq_zone.fog_maxclip1 != takp_zone.fog_maxclip1 {
            updates.push(("fog_maxclip1", takp_zone.fog_maxclip1.to_string()));
        }

        if peq_zone.fog_red2 != takp_zone.fog_red2 {
            updates.push(("fog_red2", takp_zone.fog_red2.to_string()));
        }

        if peq_zone.fog_green2 != takp_zone.fog_green2 {
            updates.push(("fog_green2", takp_zone.fog_green2.to_string()));
        }

        if peq_zone.fog_blue2 != takp_zone.fog_blue2 {
            updates.push(("fog_blue2", takp_zone.fog_blue2.to_string()));
        }

        if peq_zone.fog_minclip2 != takp_zone.fog_minclip2 {
            updates.push(("fog_minclip2", takp_zone.fog_minclip2.to_string()));
        }

        if peq_zone.fog_maxclip2 != takp_zone.fog_maxclip2 {
            updates.push(("fog_maxclip2", takp_zone.fog_maxclip2.to_string()));
        }

        if peq_zone.fog_red3 != takp_zone.fog_red3 {
            updates.push(("fog_red3", takp_zone.fog_red3.to_string()));
        }

        if peq_zone.fog_green3 != takp_zone.fog_green3 {
            updates.push(("fog_green3", takp_zone.fog_green3.to_string()));
        }

        if peq_zone.fog_blue3 != takp_zone.fog_blue3 {
            updates.push(("fog_blue3", takp_zone.fog_blue3.to_string()));
        }

        if peq_zone.fog_minclip3 != takp_zone.fog_minclip3 {
            updates.push(("fog_minclip3", takp_zone.fog_minclip3.to_string()));
        }

        if peq_zone.fog_maxclip3 != takp_zone.fog_maxclip3 {
            updates.push(("fog_maxclip3", takp_zone.fog_maxclip3.to_string()));
        }

        if peq_zone.fog_red4 != takp_zone.fog_red4 {
            updates.push(("fog_red4", takp_zone.fog_red4.to_string()));
        }

        if peq_zone.fog_green4 != takp_zone.fog_green4 {
            updates.push(("fog_green4", takp_zone.fog_green4.to_string()));
        }

        if peq_zone.fog_blue4 != takp_zone.fog_blue4 {
            updates.push(("fog_blue4", takp_zone.fog_blue4.to_string()));
        }

        if peq_zone.fog_minclip4 != takp_zone.fog_minclip4 {
            updates.push(("fog_minclip4", takp_zone.fog_minclip4.to_string()));
        }

        if peq_zone.fog_maxclip4 != takp_zone.fog_maxclip4 {
            updates.push(("fog_maxclip4", takp_zone.fog_maxclip4.to_string()));
        }

        if peq_zone.fog_density != takp_zone.fog_density {
            updates.push(("fog_density", takp_zone.fog_density.to_string()));
        }

        if peq_zone.rain_chance1 != takp_zone.rain_chance1 {
            updates.push(("rain_chance1", takp_zone.rain_chance1.to_string()));
        }

        if peq_zone.rain_chance2 != takp_zone.rain_chance2 {
            updates.push(("rain_chance2", takp_zone.rain_chance2.to_string()));
        }

        if peq_zone.rain_chance3 != takp_zone.rain_chance3 {
            updates.push(("rain_chance3", takp_zone.rain_chance3.to_string()));
        }

        if peq_zone.rain_chance4 != takp_zone.rain_chance4 {
            updates.push(("rain_chance4", takp_zone.rain_chance4.to_string()));
        }

        if peq_zone.rain_duration1 != takp_zone.rain_duration1 {
            updates.push(("rain_duration1", takp_zone.rain_duration1.to_string()));
        }

        if peq_zone.rain_duration2 != takp_zone.rain_duration2 {
            updates.push(("rain_duration2", takp_zone.rain_duration2.to_string()));
        }

        if peq_zone.rain_duration3 != takp_zone.rain_duration3 {
            updates.push(("rain_duration3", takp_zone.rain_duration3.to_string()));
        }

        if peq_zone.rain_duration4 != takp_zone.rain_duration4 {
            updates.push(("rain_duration4", takp_zone.rain_duration4.to_string()));
        }

        if peq_zone.snow_chance1 != takp_zone.snow_chance1 {
            updates.push(("snow_chance1", takp_zone.snow_chance1.to_string()));
        }

        if peq_zone.snow_chance2 != takp_zone.snow_chance2 {
            updates.push(("snow_chance2", takp_zone.snow_chance2.to_string()));
        }

        if peq_zone.snow_chance3 != takp_zone.snow_chance3 {
            updates.push(("snow_chance3", takp_zone.snow_chance3.to_string()));
        }

        if peq_zone.snow_chance4 != takp_zone.snow_chance4 {
            updates.push(("snow_chance4", takp_zone.snow_chance4.to_string()));
        }

        if peq_zone.snow_duration1 != takp_zone.snow_duration1 {
            updates.push(("snow_duration1", takp_zone.snow_duration1.to_string()));
        }

        if peq_zone.snow_duration2 != takp_zone.snow_duration2 {
            updates.push(("snow_duration2", takp_zone.snow_duration2.to_string()));
        }

        if peq_zone.snow_duration3 != takp_zone.snow_duration3 {
            updates.push(("snow_duration3", takp_zone.snow_duration3.to_string()));
        }

        if peq_zone.snow_duration4 != takp_zone.snow_duration4 {
            updates.push(("snow_duration4", takp_zone.snow_duration4.to_string()));
        }

        if updates.len() > 0 {
            query.push_str("UPDATE zone SET ");
            if updates.len() > 1 {
                for counter in 0..updates.len() - 1 {
                    query.push_str(&format!("{} = {}, ", updates[counter].0, updates[counter].1));
                }
            }

            query.push_str(
                &format!("{} = {}", updates[updates.len() - 1].0, updates[updates.len() - 1].1)
            );

            query.push_str(&format!(" WHERE id={};\n", peq_id));
        }
    }

    query
}
