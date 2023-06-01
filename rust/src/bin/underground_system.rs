use std::collections::HashMap;

pub struct UndergroundSystem {
    check_ins: HashMap<i32, (String, i32)>,
    travel_times: HashMap<(String, String), (i32, i32)>,
}

impl UndergroundSystem {
    pub fn new() -> Self {
        Self {
            check_ins: HashMap::new(),
            travel_times: HashMap::new(),
        }
    }

    pub fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        self.check_ins.insert(id, (station_name, t));
    }

    pub fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        if let Some((start_station, start_time)) = self.check_ins.remove(&id) {
            let key = (start_station, station_name);
            let (total_time, count) = self.travel_times.get(&key).unwrap_or(&(0, 0)).clone();
            self.travel_times
                .insert(key, (total_time + (t - start_time), count + 1));
        }
    }

    pub fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        if let Some((total_time, count)) = self.travel_times.get(&(start_station, end_station)) {
            *total_time as f64 / *count as f64
        } else {
            0.0
        }
    }
}

/**
 * Your UndergroundSystem object will be instantiated and called as such:
 * let obj = UndergroundSystem::new();
 * obj.check_in(id, stationName, t);
 * obj.check_out(id, stationName, t);
 * let ret_3: f64 = obj.get_average_time(startStation, endStation);
 */

pub fn main() {
    todo!()
}
