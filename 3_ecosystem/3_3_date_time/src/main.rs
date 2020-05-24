use chrono::offset::LocalResult;
use chrono::prelude::*;
use std::convert::TryFrom;

fn main() {
    println!("Implement me!");
}

const NOW: &str = "2019-06-26 23:56:04";

struct User(Date<Utc>);

impl User {
    fn with_birthdate(year: i32, month: u32, day: u32) -> Self {
        User(Utc.ymd(year, month, day))
    }

    /// Returns current age of [`User`] in years.
    fn age(&self) -> u16 {
        let dt = NaiveDateTime::parse_from_str(NOW, "%Y-%m-%d %H:%M:%S");
        let now = dt.unwrap().date();
        let y_diff = now.year() - self.0.year();
        let m_dif = now.month() as i64 - self.0.month() as i64;
        let d_diff = now.day() as i64 - self.0.day() as i64;
        println!("{},{},{}", y_diff, m_dif, d_diff);
        if y_diff <= 0 {
            return 0;
        } else if m_dif > 0 || (m_dif == 0 && d_diff >= 0) {
            return u16::try_from(y_diff).unwrap();
        } else {
            return u16::try_from(y_diff - 1).unwrap();
        }
    }

    /// Checks if [`User`] is 18 years old at the moment.
    fn is_adult(&self) -> bool {
        self.age() >= 18
    }
}

#[cfg(test)]
mod age_spec {
    use super::*;

    #[test]
    fn counts_age() {
        for ((y, m, d), expected) in vec![
            ((1990, 6, 4), 29),
            ((1990, 7, 4), 28),
            ((0, 1, 1), 2019),
            ((1970, 1, 1), 49),
            ((2019, 6, 25), 0),
        ] {
            let user = User::with_birthdate(y, m, d);
            assert_eq!(user.age(), expected);
        }
    }

    #[test]
    fn zero_if_birthdate_in_future() {
        for ((y, m, d), expected) in vec![
            ((2032, 6, 25), 0),
            //((2016, 6, 27), 0),
            ((3000, 6, 27), 0),
            ((9999, 6, 27), 0),
        ] {
            let user = User::with_birthdate(y, m, d);
            assert_eq!(user.age(), expected);
        }
    }

    #[test]
    fn is_adult_border_values() {
        for ((y, m, d), expected) in vec![
            ((2032, 6, 25), false),
            ((2001, 06, 26), true),
            ((2001, 06, 27), false),
            ((2002, 06, 26), false),
            ((2001, 07, 26), false),
        ] {
            let user = User::with_birthdate(y, m, d);
            assert_eq!(user.is_adult(), expected);
        }
    }
}
