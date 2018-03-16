const EARLIEST_ARRIVAL_TIME: i32 = 5;
const LATEST_BEDTIME: i32 = 12;

struct Babysitter {
    standard_rate: i32,
    house_sit_rate: i32,
    after_midnight_bonus: i32
}

#[allow(dead_code)]
impl Babysitter {


    fn get_earnings(&self, arrival_time: i32, departure_time: i32, bedtime: i32) -> i32 {
        let mut earnings: i32 = 0;
        if arrival_time != departure_time {
            earnings += self.standard_rate * self.get_hours_before_bedtime(arrival_time, departure_time, bedtime);
            earnings += self.house_sit_rate * self.get_hours_after_bedtime(arrival_time, departure_time, bedtime);
            earnings += self.after_midnight_bonus * self.get_hours_after_midnight(departure_time);
        }
        return earnings;
    }

    fn get_hours_before_bedtime(&self, arrival_time: i32, departure_time: i32, bedtime: i32) -> i32 {
        let hours: i32;
        if arrival_time == departure_time {
            hours = 0;
        } else if arrival_time < EARLIEST_ARRIVAL_TIME {
            hours = 0;
        } else {
            hours = bedtime - arrival_time;
        }
        return hours;
    }

    fn get_hours_after_bedtime(&self, arrival_time: i32, departure_time: i32, bedtime: i32) -> i32 {
        let hours: i32;
        if arrival_time == departure_time {
            hours = 0;
        } else if departure_time < EARLIEST_ARRIVAL_TIME {
            hours = (LATEST_BEDTIME - bedtime) + departure_time;
        } else {
            hours = departure_time - bedtime;
        }
        return hours;
    }

    fn get_hours_after_midnight(&self, departure_time: i32) -> i32 {
        let hours: i32;
        if departure_time < EARLIEST_ARRIVAL_TIME {
            hours = departure_time;
        } else {
            hours = 0;
        }
        return hours;
    }
}


#[cfg(test)]
mod babysitter_tests {
    use Babysitter;

    const BABYSITTER: Babysitter = Babysitter { standard_rate: 10, house_sit_rate: 6, after_midnight_bonus: 2 };

    #[test]
    fn it_gets_no_earnings() {
        assert_eq!(BABYSITTER.get_earnings(5, 5, 12), 0);
        assert_eq!(BABYSITTER.get_earnings(12, 12, 12), 0);
        assert_eq!(BABYSITTER.get_earnings(4, 4, 12), 0);
        assert_eq!(BABYSITTER.get_earnings(5, 5, 5), 0);
    }

    #[test]
    fn it_gets_hours_of_work_before_bedtime() {
        assert_eq!(BABYSITTER.get_hours_before_bedtime(5, 4, 12), 12 - 5);
        assert_eq!(BABYSITTER.get_hours_before_bedtime(4, 4, 12), 0);
        assert_eq!(BABYSITTER.get_hours_before_bedtime(1, 4, 12), 0);
        assert_eq!(BABYSITTER.get_hours_before_bedtime(5, 4, 5), 0);
    }

    #[test]
    fn it_gets_hours_of_work_after_bedtime() {
        assert_eq!(BABYSITTER.get_hours_after_bedtime(5, 12, 8), 4);
        assert_eq!(BABYSITTER.get_hours_after_bedtime(4, 4, 12), 0);
        assert_eq!(BABYSITTER.get_hours_after_bedtime(1, 4, 12), 4);
        assert_eq!(BABYSITTER.get_hours_after_bedtime(5, 4, 5), (12 - 5) + 4);
    }

    #[test]
    fn it_gets_hours_of_work_after_midnight() {
        assert_eq!(BABYSITTER.get_hours_after_midnight(12), 0);
        assert_eq!(BABYSITTER.get_hours_after_midnight(4), 4);
        assert_eq!(BABYSITTER.get_hours_after_midnight(4), 4);
        assert_eq!(BABYSITTER.get_hours_after_midnight(4), 4);
        assert_eq!(BABYSITTER.get_hours_after_midnight(4), 4);
    }

    #[test]
    fn it_gets_standard_earnings() {
        assert_eq!(BABYSITTER.get_earnings(5, 8, 8), 3 * BABYSITTER.standard_rate);
    }

    #[test]
    fn it_gets_house_sit_earnings() {
        assert_eq!(BABYSITTER.get_earnings(8, 12, 8), 4 * BABYSITTER.house_sit_rate);
    }

    #[test]
    fn it_gets_after_midnight_earnings() {
        assert_eq!(BABYSITTER.get_earnings(12, 4, 12), 4 * (BABYSITTER.house_sit_rate + BABYSITTER.after_midnight_bonus));
    }

    #[test]
    fn it_gets_all_earnings() {
        assert_eq!(BABYSITTER.get_earnings(5, 4, 5),
                   ((12 - 5) * BABYSITTER.house_sit_rate)
                       + (4 * (BABYSITTER.house_sit_rate + BABYSITTER.after_midnight_bonus)));
        assert_eq!(BABYSITTER.get_earnings(5, 4, 12),
                   ((12 - 5) * BABYSITTER.standard_rate)
                       + (4 * (BABYSITTER.house_sit_rate + BABYSITTER.after_midnight_bonus)));
        assert_eq!(BABYSITTER.get_earnings(5, 4, 8),
                   ((8 - 5) * BABYSITTER.standard_rate)
                       + ((12 - 8) * BABYSITTER.house_sit_rate)
                       + (4 * (BABYSITTER.house_sit_rate + BABYSITTER.after_midnight_bonus)));
        assert_eq!(BABYSITTER.get_earnings(5, 11, 8),
                   ((8 - 5) * BABYSITTER.standard_rate)
                       + ((11 - 8) * BABYSITTER.house_sit_rate));
    }
}