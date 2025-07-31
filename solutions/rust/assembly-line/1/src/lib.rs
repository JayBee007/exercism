// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

const CARS_PRODUCED: f64 = 221.0;

    pub fn production_rate_per_hour(speed: u8) -> f64 {
        let spd = speed as f64;
        match speed {
            1..=4 => CARS_PRODUCED * spd,
            5..=8 => (CARS_PRODUCED * spd) * 0.90,
            9..=10 => (CARS_PRODUCED * spd) * 0.77,
            _ => 0.0,
        }
    }


    
    pub fn working_items_per_minute(speed: u8) -> u32 {
        let hour_range = production_rate_per_hour(speed);

        hour_range as u32 / 60
    
}
