pub enum EDiceSide {
    Side1,
    Side2,
    Side3,
    Side4,
    Side5,
    Side6,
}

pub struct RollResult {
    pub side: EDiceSide,
    pub side_as_number: u16,
}
