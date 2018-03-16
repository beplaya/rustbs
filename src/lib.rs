
struct Babysitter {

}

impl Babysitter {
    pub fn get_earnings(&self, arrivalTime: i32, departureTime: i32, bedtime: i32) -> i32 {
        return 0;
    }

    fn get_hours_before_bedtime(&self, arrivalTime: i32, departureTime: i32, bedtime: i32) -> i32 {
        let hours : i32;
        if arrivalTime == departureTime {
            hours = 0;
        } else if arrivalTime < 5 {
            hours = 0;
        } else {
            hours = bedtime - arrivalTime;
        }
        return hours;
    }
}



#[cfg(test)]
mod babysitter_tests {
    use Babysitter;

    #[test]
    fn it_gets_no_earnings() {
        let babysitter = Babysitter {};
        assert_eq!(babysitter.get_earnings(5, 5, 12), 0);
        assert_eq!(babysitter.get_earnings(12, 12, 12), 0);
        assert_eq!(babysitter.get_earnings(4, 4, 12), 0);
    }

    #[test]
    fn it_gets_hours_of_work_before_bedtime() {
        let babysitter = Babysitter {};
        assert_eq!(babysitter.get_hours_before_bedtime(5, 4, 12), 12 - 5);
        assert_eq!(babysitter.get_hours_before_bedtime(4, 4, 12), 0);
        assert_eq!(babysitter.get_hours_before_bedtime(1, 4, 12), 0);
        assert_eq!(babysitter.get_hours_before_bedtime(5, 4, 5), 0);
    }


}