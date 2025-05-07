pub struct ZoneIdMapping {
    pub peq_id: i32,
    pub takp_id: i32,
}

impl ZoneIdMapping {
    pub fn new(peq_id: i32, takp_id: i32) -> Self {
        ZoneIdMapping { peq_id, takp_id }
    }
}
