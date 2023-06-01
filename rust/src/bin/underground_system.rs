use std::collections::HashMap;
#[derive(Clone)]
struct Transaction {
    start_station_name: String,
    end_station_name: String,
    start_time: i32,
    end_time: i32,
}

struct UndergroundSystem {
    // a hash set of ID and transaction
    transactions: HashMap<i32, Transaction>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl UndergroundSystem {
    fn new() -> Self {
        Self {
            transactions: HashMap::new(),
        }
    }

    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        self.transactions.insert(
            id,
            Transaction {
                start_station_name: station_name,
                end_station_name: "".to_string(),
                start_time: t,
                end_time: 0,
            },
        );
    }

    fn check_out(&self, id: i32, station_name: String, t: i32) {
        // check if the id is in the transactions
        if let Some(transaction) = self.transactions.get(&id) {
            // update the end station name and end time
            let mut transaction = transaction.clone();
            transaction.end_station_name = station_name;
            transaction.end_time = t;
            // update the transaction
            self.transactions.insert(id, transaction.clone());
        }
    }

    fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        // takes the average time it takes to travel from start_station to end_station.
        // The average time is computed from all the previous traveling from start_station to end_station that happened directly.
        // Call to getAverageTime is always valid.
        let mut total_time = 0;
        let mut total_count = 0;
        for (_, transaction) in self.transactions.iter() {
            if transaction.start_station_name == start_station
                && transaction.end_station_name == end_station
            {
                total_time += transaction.end_time - transaction.start_time;
                total_count += 1;
            }
        }
        total_time as f64 / total_count as f64
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
