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
    const MIDNIGHT: i32 = 12;

    fn is_zero_hour_shift(&self, arrival_time: i32, departure_time: i32) -> bool {
        return arrival_time == departure_time;
    }

    fn all_work_after_midnight(&self, arrival_time: i32, departure_time: i32) -> bool {
        return self.the(arrival_time).is_after_midnight() && self.the(departure_time).is_after_midnight();
    }

    fn get_hours_before_bedtime(&self, arrival_time: i32, departure_time: i32, bedtime: i32) -> i32 {
        if !self.is_zero_hour_shift(arrival_time, departure_time) && self.the(arrival_time).is_before_midnight() {
            return bedtime - arrival_time;
        }
        return 0;
    }

    fn get_hours_after_bedtime(&self, arrival_time: i32, departure_time: i32, bedtime: i32) -> i32 {
        let hours: i32;
        if self.is_zero_hour_shift(arrival_time, departure_time) {
            hours = 0;
        } else if self.all_work_after_midnight(arrival_time, departure_time) {
            hours = departure_time - arrival_time;
        } else if self.the(departure_time).is_after_midnight() {
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
        } else if self.the(departure_time).is_after_midnight() {
            hours = departure_time;
        } else {
            hours = 0;
        }
        return hours;
    }

    fn is_valid_schedule(&self, arrival_time: i32, departure_time: i32) -> bool {
        if self.the(arrival_time).is_after_midnight() == self.the(departure_time).is_after_midnight() {
            return arrival_time <= departure_time;
        } else if self.the(arrival_time).is_after_midnight() && self.the(departure_time).is_before_midnight() {
            return false;
        } else {
            return true;
        }
    }

    fn the(&self, time: i32) -> TimeValidator {
        return TimeValidator { earliest_arrival: self.earliest_arrival, time };
    }
}

struct TimeValidator {
    earliest_arrival: i32,
    time: i32,
}

impl TimeValidator {
    fn is_before_midnight(&self) -> bool {
        return !self.is_after_midnight();
    }

    fn is_after_midnight(&self) -> bool {
        return self.time < self.earliest_arrival;
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
        if self.schedule.is_valid_schedule(arrival_time, departure_time) {
            earnings += self.rate.standard_rate * self.schedule.get_hours_before_bedtime(arrival_time, departure_time, bedtime);
            earnings += self.rate.house_sit_rate * self.schedule.get_hours_after_bedtime(arrival_time, departure_time, bedtime);
            earnings += self.rate.after_midnight_bonus * self.schedule.get_hours_after_midnight(arrival_time, departure_time);
        }
        return earnings;
    }
}

#[cfg(test)]
mod schedule_tests {
    use Schedule;

    const SCHEDULE: Schedule = Schedule { earliest_arrival: 5, latest_bedtime: 12, latest_shift_end: 4 };

    #[test]
    fn it_gets_hours_of_work_before_bedtime() {
        assert_eq!(SCHEDULE.get_hours_before_bedtime(5, 4, 12), 12 - 5);
        assert_eq!(SCHEDULE.get_hours_before_bedtime(4, 4, 12), 0);
        assert_eq!(SCHEDULE.get_hours_before_bedtime(8, 8, 12), 0);
        assert_eq!(SCHEDULE.get_hours_before_bedtime(1, 4, 12), 0);
        assert_eq!(SCHEDULE.get_hours_before_bedtime(5, 4, 5), 0);

        assert_eq!(SCHEDULE.get_hours_before_bedtime(2, 3, 8), 0);
    }

    #[test]
    fn it_gets_hours_of_work_after_bedtime() {
        assert_eq!(SCHEDULE.get_hours_after_bedtime(5, 12, 8), 4);
        assert_eq!(SCHEDULE.get_hours_after_bedtime(4, 4, 12), 0);
        assert_eq!(SCHEDULE.get_hours_after_bedtime(1, 4, 12), 3);
        assert_eq!(SCHEDULE.get_hours_after_bedtime(5, 4, 5), (12 - 5) + 4);

        assert_eq!(SCHEDULE.get_hours_after_bedtime(2, 3, 8), 1);
    }

    #[test]
    fn it_gets_hours_of_work_after_midnight() {
        assert_eq!(SCHEDULE.get_hours_after_midnight(5, 12), 0);
        assert_eq!(SCHEDULE.get_hours_after_midnight(4, 4), 0);
        assert_eq!(SCHEDULE.get_hours_after_midnight(8, 8), 0);
        assert_eq!(SCHEDULE.get_hours_after_midnight(5, 4), 4);
        assert_eq!(SCHEDULE.get_hours_after_midnight(2, 3), 1);
    }

    #[test]
    fn it_validates_times() {
        assert_eq!(SCHEDULE.is_valid_schedule(5, 5), true);
        assert_eq!(SCHEDULE.is_valid_schedule(12, 12), true);
        assert_eq!(SCHEDULE.is_valid_schedule(4, 4), true);
        assert_eq!(SCHEDULE.is_valid_schedule(3, 4), true);
        assert_eq!(SCHEDULE.is_valid_schedule(8, 9), true);
    }

    #[test]
    fn it_invalidates_times() {
        assert_eq!(SCHEDULE.is_valid_schedule(6, 5), false);
        assert_eq!(SCHEDULE.is_valid_schedule(3, 2), false);
        assert_eq!(SCHEDULE.is_valid_schedule(3, 8), false);
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
        assert_eq!(BABYSITTER.get_earnings(5, 8, 8), (8 - 5) * RATE.standard_rate);
    }

    #[test]
    fn it_gets_house_sit_earnings() {
        assert_eq!(BABYSITTER.get_earnings(8, 12, 8), 4 * RATE.house_sit_rate);
    }

    #[test]
    fn it_gets_after_midnight_earnings() {
        assert_eq!(BABYSITTER.get_earnings(12, 4, 12), 4 * (RATE.house_sit_rate + RATE.after_midnight_bonus));
    }

    #[test]
    fn it_gets_all_earnings() {
        let mut expected: i32;

        expected = ((12 - 5) * RATE.house_sit_rate) + (4 * (RATE.house_sit_rate + RATE.after_midnight_bonus));
        assert_eq!(BABYSITTER.get_earnings(5, 4, 5), expected);

        expected = ((12 - 5) * RATE.standard_rate) + (4 * (RATE.house_sit_rate + RATE.after_midnight_bonus));
        assert_eq!(BABYSITTER.get_earnings(5, 4, 12), expected);

        expected = ((8 - 5) * RATE.standard_rate) + ((12 - 8) * RATE.house_sit_rate) + (4 * (RATE.house_sit_rate + RATE.after_midnight_bonus));
        assert_eq!(BABYSITTER.get_earnings(5, 4, 8), expected);

        expected = ((8 - 5) * RATE.standard_rate) + ((11 - 8) * RATE.house_sit_rate);
        assert_eq!(BABYSITTER.get_earnings(5, 11, 8), expected);
    }

    #[test]
    fn it_gets_only_bonus_earnings() {
        let expected: i32 = RATE.house_sit_rate + RATE.after_midnight_bonus;
        assert_eq!(BABYSITTER.get_earnings(4 - 2, 4 - 1, 12 - 4), expected);
    }

    #[test]
    fn it_gets_no_earnings() {
        assert_eq!(BABYSITTER.get_earnings(5, 5, 12), 0);
        assert_eq!(BABYSITTER.get_earnings(12, 12, 12), 0);
        assert_eq!(BABYSITTER.get_earnings(4, 4, 12), 0);
        assert_eq!(BABYSITTER.get_earnings(5, 5, 5), 0);
        assert_eq!(BABYSITTER.get_earnings(2, 2, 8), 0);
    }

    #[test]
    fn it_gets_no_earnings_for_invalid_times() {
        assert_eq!(BABYSITTER.get_earnings(6, 5, 12), 0);
        assert_eq!(BABYSITTER.get_earnings(3, 2, 12), 0);
        assert_eq!(BABYSITTER.get_earnings(3, 8, 12), 0);
    }
}