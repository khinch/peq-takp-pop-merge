use diesel::{ prelude::*, MysqlConnection, Queryable };

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::takp::zone)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Zone {
    pub id: i32,
    pub short_name: String,
    pub safe_x: f32,
    pub safe_y: f32,
    pub safe_z: f32,
    pub safe_heading: f32,
    pub sky: i8,
    pub minclip: f32,
    pub maxclip: f32,
    pub fog_minclip: f32,
    pub fog_maxclip: f32,
    pub fog_red: i8,
    pub fog_green: i8,
    pub fog_blue: i8,
    pub fog_red1: i8,
    pub fog_green1: i8,
    pub fog_blue1: i8,
    pub fog_minclip1: f32,
    pub fog_maxclip1: f32,
    pub fog_red2: i8,
    pub fog_green2: i8,
    pub fog_blue2: i8,
    pub fog_minclip2: f32,
    pub fog_maxclip2: f32,
    pub fog_red3: i8,
    pub fog_green3: i8,
    pub fog_blue3: i8,
    pub fog_minclip3: f32,
    pub fog_maxclip3: f32,
    pub fog_red4: i8,
    pub fog_green4: i8,
    pub fog_blue4: i8,
    pub fog_minclip4: f32,
    pub fog_maxclip4: f32,
    pub fog_density: f32,
    pub rain_chance1: i32,
    pub rain_chance2: i32,
    pub rain_chance3: i32,
    pub rain_chance4: i32,
    pub rain_duration1: i32,
    pub rain_duration2: i32,
    pub rain_duration3: i32,
    pub rain_duration4: i32,
    pub snow_chance1: i32,
    pub snow_chance2: i32,
    pub snow_chance3: i32,
    pub snow_chance4: i32,
    pub snow_duration1: i32,
    pub snow_duration2: i32,
    pub snow_duration3: i32,
    pub snow_duration4: i32,
}

impl Zone {
    pub fn get_list(connection: &mut MysqlConnection) -> Vec<Self> {
        match crate::schema::takp::zone::table.load::<Zone>(connection) {
            Ok(zones) => zones,
            Err(err) => panic!("Unable to retrieve zone list: {}", err),
        }
    }
}
