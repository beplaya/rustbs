
struct Babysitter {

}

impl Babysitter {
    pub fn get_earnings(&self) -> i32 {
        return 0;
    }
}



#[cfg(test)]
mod tests {
    use Babysitter;

    #[test]
    fn it_gets_no_earnings() {
        let babysitter = Babysitter {};
        assert_eq!(babysitter.get_earnings(), 0)
    }
}