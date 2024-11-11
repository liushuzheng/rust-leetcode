mod satisfies_conditions;

mod max_consecutive_answers;

mod contains_nearby_duplicate;

mod max_strength;

mod clear_digits;

mod remove_stars;

mod max_score_sightseeing_pair;


struct EquityOrder {
    pub time: u64,
    pub id: u64,
}

struct Order {
    pub time: u64,
    pub id: u64,
}

trait WriteOff{
    fn t(self: Self)->u64;
   fn id(self: Self)->u64;
    
}

impl WriteOff for EquityOrder {
    fn t(self: Self) -> u64 {
        self.time
    }

    fn id(self: Self) -> u64 {
        self.id
    }
}


impl WriteOff for Order {
    fn t(self: Self) -> u64 {
        self.time
    }

    fn id(self: Self) -> u64 {
        self.id
    }
}

