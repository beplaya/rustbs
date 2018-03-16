#[allow(dead_code)]
struct Rate {
    standard_rate: i32,
    house_sit_rate: i32,
    after_midnight_bonus: i32,
}

#[allow(dead_code)]
struct Schedule {
    earliest_arrival: i32,
    latest_bedtime: i32,
    latest_shift_end: i32,
}

#[allow(dead_code)]
impl Schedule {
    fn is_zero_hour_shift(&self, arrival_time: i32, departure_time: i32) -> bool {
        return arrival_time == departure_time;
    }

    fn all_work_after_midnight(&self, arrival_time: i32, departure_time: i32) -> bool {
        return arrival_time < self.earliest_arrival && departure_time < self.earliest_arrival;
    }

    fn get_hours_before_bedtime(&self, arrival_time: i32, departure_time: i32, bedtime: i32) -> i32 {
        let hours: i32;
        if self.is_zero_hour_shift(arrival_time, departure_time) {
            hours = 0;
        } else if arrival_time < self.earliest_arrival {
            hours = 0;
        } else {
            hours = bedtime - arrival_time;
        }
        return hours;
    }

    fn get_hours_after_bedtime(&self, arrival_time: i32, departure_time: i32, bedtime: i32) -> i32 {
        let hours: i32;
        if self.is_zero_hour_shift(arrival_time, departure_time) {
            hours = 0;
        } else if self.all_work_after_midnight(arrival_time, departure_time) {
            hours = departure_time - arrival_time;
        } else if departure_time < self.earliest_arrival {
            hours = (self.latest_bedtime - bedtime) + departure_time;
        } else {
            hours = departure_time - bedtime;
        }
        return hours;
    }

    fn get_hours_after_midnight(&self, arrival_time: i32, departure_time: i32) -> i32 {
        let hours: i32;
        if self.all_work_after_midnight(arrival_time, departure_time) {
            hours = departure_time - arrival_time;
        } else if departure_time < self.earliest_arrival {
            hours = departure_time;
        } else {
            hours = 0;
        }
        return hours;
    }
}

#[allow(dead_code)]
struct Babysitter {
    rate: Rate,
    schedule: Schedule,
}

#[allow(dead_code)]
impl Babysitter {
    fn get_earnings(&self, arrival_time: i32, departure_time: i32, bedtime: i32) -> i32 {
        let mut earnings: i32 = 0;
        earnings += self.rate.standard_rate * self.schedule.get_hours_before_bedtime(arrival_time, departure_time, bedtime);
        earnings += self.rate.house_sit_rate * self.schedule.get_hours_after_bedtime(arrival_time, departure_time, bedtime);
        earnings += self.rate.after_midnight_bonus * self.schedule.get_hours_after_midnight(arrival_time, departure_time);
        return earnings;
    }
}

#[cfg(test)]
mod schedule_tests {
    use Schedule;

    const SCHEDULE: Schedule = Schedule { earliest_arrival: 5, latest_bedtime: 12, latest_shift_end: 4 };

    #[test]
    fn it_gets_hours_of_work_before_bedtime() {
        assert_eq!(SCHEDULE.get_hours_before_bedtime(SCHEDULE.earliest_arrival, SCHEDULE.latest_shift_end, SCHEDULE.latest_bedtime), SCHEDULE.latest_bedtime - SCHEDULE.earliest_arrival);
        assert_eq!(SCHEDULE.get_hours_before_bedtime(SCHEDULE.latest_shift_end, SCHEDULE.latest_shift_end, SCHEDULE.latest_bedtime), SCHEDULE.latest_shift_end - SCHEDULE.latest_shift_end);
        assert_eq!(SCHEDULE.get_hours_before_bedtime(1, SCHEDULE.latest_shift_end, SCHEDULE.latest_bedtime), 0);
        assert_eq!(SCHEDULE.get_hours_before_bedtime(SCHEDULE.earliest_arrival, SCHEDULE.latest_shift_end, SCHEDULE.earliest_arrival), 0);

        assert_eq!(SCHEDULE.get_hours_before_bedtime(SCHEDULE.latest_shift_end - 2, SCHEDULE.latest_shift_end - 1, SCHEDULE.latest_bedtime - 4), 0);
    }

    #[test]
    fn it_gets_hours_of_work_after_bedtime() {
        assert_eq!(SCHEDULE.get_hours_after_bedtime(SCHEDULE.earliest_arrival, SCHEDULE.latest_bedtime, 8), 4);
        assert_eq!(SCHEDULE.get_hours_after_bedtime(SCHEDULE.latest_shift_end, SCHEDULE.latest_shift_end, SCHEDULE.latest_bedtime), 0);
        assert_eq!(SCHEDULE.get_hours_after_bedtime(1, SCHEDULE.latest_shift_end, SCHEDULE.latest_bedtime), 3);
        assert_eq!(SCHEDULE.get_hours_after_bedtime(SCHEDULE.earliest_arrival, SCHEDULE.latest_shift_end, SCHEDULE.earliest_arrival), (SCHEDULE.latest_bedtime - SCHEDULE.earliest_arrival) + SCHEDULE.latest_shift_end);

        assert_eq!(SCHEDULE.get_hours_after_bedtime(SCHEDULE.latest_shift_end - 2, SCHEDULE.latest_shift_end - 1, SCHEDULE.latest_bedtime - 4), 1);
    }

    #[test]
    fn it_gets_hours_of_work_after_midnight() {
        assert_eq!(SCHEDULE.get_hours_after_midnight(SCHEDULE.earliest_arrival, SCHEDULE.latest_bedtime), 0);
        assert_eq!(SCHEDULE.get_hours_after_midnight(SCHEDULE.earliest_arrival, SCHEDULE.latest_shift_end), SCHEDULE.latest_shift_end);
        assert_eq!(SCHEDULE.get_hours_after_midnight(SCHEDULE.earliest_arrival, SCHEDULE.latest_shift_end), SCHEDULE.latest_shift_end);
        assert_eq!(SCHEDULE.get_hours_after_midnight(SCHEDULE.earliest_arrival, SCHEDULE.latest_shift_end), SCHEDULE.latest_shift_end);
        assert_eq!(SCHEDULE.get_hours_after_midnight(SCHEDULE.earliest_arrival, SCHEDULE.latest_shift_end), SCHEDULE.latest_shift_end);

        assert_eq!(SCHEDULE.get_hours_after_midnight(SCHEDULE.latest_shift_end - 2, SCHEDULE.latest_shift_end - 1), 1);
    }

}

#[cfg(test)]
mod babysitter_tests {
    use Babysitter;
    use Schedule;
    use Rate;

    const SCHEDULE: Schedule = Schedule { earliest_arrival: 5, latest_bedtime: 12, latest_shift_end: 4 };
    const RATE: Rate = Rate { standard_rate: 10, house_sit_rate: 6, after_midnight_bonus: 2 };
    const BABYSITTER: Babysitter = Babysitter { rate: RATE, schedule: SCHEDULE };


    #[test]
    fn it_gets_standard_earnings() {
        assert_eq!(BABYSITTER.get_earnings(SCHEDULE.earliest_arrival, 8, 8), (8 - SCHEDULE.earliest_arrival) * RATE.standard_rate);
    }

    #[test]
    fn it_gets_house_sit_earnings() {
        assert_eq!(BABYSITTER.get_earnings(8, SCHEDULE.latest_bedtime, 8), SCHEDULE.latest_shift_end * RATE.house_sit_rate);
    }

    #[test]
    fn it_gets_after_midnight_earnings() {
        assert_eq!(BABYSITTER.get_earnings(SCHEDULE.latest_bedtime, SCHEDULE.latest_shift_end, SCHEDULE.latest_bedtime), SCHEDULE.latest_shift_end * (RATE.house_sit_rate + RATE.after_midnight_bonus));
    }

    #[test]
    fn it_gets_all_earnings() {
        let mut expected: i32;

        expected = ((SCHEDULE.latest_bedtime - SCHEDULE.earliest_arrival) * RATE.house_sit_rate) + (SCHEDULE.latest_shift_end * (RATE.house_sit_rate + RATE.after_midnight_bonus));
        assert_eq!(BABYSITTER.get_earnings(SCHEDULE.earliest_arrival, SCHEDULE.latest_shift_end, SCHEDULE.earliest_arrival), expected);

        expected = ((SCHEDULE.latest_bedtime - SCHEDULE.earliest_arrival) * RATE.standard_rate) + (SCHEDULE.latest_shift_end * (RATE.house_sit_rate + RATE.after_midnight_bonus));
        assert_eq!(BABYSITTER.get_earnings(SCHEDULE.earliest_arrival, SCHEDULE.latest_shift_end, SCHEDULE.latest_bedtime), expected);

        expected = ((8 - SCHEDULE.earliest_arrival) * RATE.standard_rate) + ((SCHEDULE.latest_bedtime - 8) * RATE.house_sit_rate) + (SCHEDULE.latest_shift_end * (RATE.house_sit_rate + RATE.after_midnight_bonus));
        assert_eq!(BABYSITTER.get_earnings(SCHEDULE.earliest_arrival, SCHEDULE.latest_shift_end, 8), expected);

        expected = ((8 - SCHEDULE.earliest_arrival) * RATE.standard_rate) + ((11 - 8) * RATE.house_sit_rate);
        assert_eq!(BABYSITTER.get_earnings(SCHEDULE.earliest_arrival, 11, 8), expected);
    }

    #[test]
    fn it_gets_only_bonus_earnings() {
        let expected: i32 = RATE.house_sit_rate + RATE.after_midnight_bonus;
        assert_eq!(BABYSITTER.get_earnings(SCHEDULE.latest_shift_end - 2, SCHEDULE.latest_shift_end - 1, SCHEDULE.latest_bedtime - 4), expected);
    }

    #[test]
    fn it_gets_no_earnings() {
        assert_eq!(BABYSITTER.get_earnings(SCHEDULE.earliest_arrival, SCHEDULE.earliest_arrival, SCHEDULE.latest_bedtime), 0);
        assert_eq!(BABYSITTER.get_earnings(SCHEDULE.latest_bedtime, SCHEDULE.latest_bedtime, SCHEDULE.latest_bedtime), 0);
        assert_eq!(BABYSITTER.get_earnings(SCHEDULE.latest_shift_end, SCHEDULE.latest_shift_end, SCHEDULE.latest_bedtime), 0);
        assert_eq!(BABYSITTER.get_earnings(SCHEDULE.earliest_arrival, SCHEDULE.earliest_arrival, SCHEDULE.earliest_arrival), 0);
    }
}