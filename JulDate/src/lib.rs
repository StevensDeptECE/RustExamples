/*=========================================================================

  Date Time Library
  Author: David Krauthamer and Dov Kruger
  Based on

  Represent a Julian Date as a double precision number of days relative
  to the epoch, J0 -4712-01-01 12:00:00 
  fraction 0 is defined as 12:00:00 (noon)

=========================================================================*/

#[derive(PartialEq, PartialOrd, Debug, Copy, Clone)]
struct JulDate {
    jd: f64,
}

impl JulDate {

pub fn from_gregorian_date(ymd: YearMonthDay) -> Self {
  JulDate { jd: Self::calculate_from_gregorian(ymd) - 0.5 }
}

pub fn from_gregorian_date_and_time(ymd: YearMonthDay, hour: u8, min: u8, sec: f64) -> Self {
    let date = Self::calculate_from_gregorian(ymd);
    let to_fractional_day = 1.0/86400.0;

    let hour: f64 = hour.into();
    let min: f64 = min.into();
    let hms = (hour * 3600.0 + min * 60.0 + sec) * to_fractional_day;
    JulDate {  jd: date + hms }
}

pub fn new(jd: f64) -> Self {
  Self { jd }
}

pub fn isLeapYear(y: i32) -> bool {
    return y % 4 == 0 && y % 100 != 0 || y % 400 == 0;
}

fn calculate_from_gregorian(ymd : YearMonthDay) -> f64 {
  let year = ymd.year;
  let month = ymd.month;
  let day = ymd.day;
  let a = (1461 * (year + 4800 + (month - 14) / 12)) / 4;
  let b = (367 * (month - 2 - 12 * ((month - 14) / 12))) / 12;
  let c = (3 * ((year + 4900 + (month - 14) / 12) / 100)) / 4;
  let d = day - 32075;
  (a + b - c + d).into()
}

pub fn to_year_month_day(&self) -> YearMonthDay {
    //TODO: objective is to make sure the object can't have a year that doesn't work
  let jd: i32 = self.jd as i32;
  let mut l = jd + 68569; // from Jan Meeus Astronomical ALgorithms
  let n = (4 * l) / 146097;
  l = l - (146097 * n + 3) / 4;
  let i = (4000 * (l + 1)) / 1461001;
  l = l - (1461 * i) / 4 + 31;
  let j = (80 * l) / 2447;
  let day = l - (2447 * j) / 80;
  l = j / 11;

  let month = j + 2 - 12 * l;
  let year = 100 * (n - 49) + i + l;
  YearMonthDay { year, month, day }
}

pub fn julian_day(ymd : YearMonthDay) -> f64 {
    let ymd0 = YearMonthDay::new(ymd.year, 1, 1);
    Self::calculate_from_gregorian(ymd) - Self::calculate_from_gregorian(ymd0) + 1.0
}

pub fn weekday(&self) -> i64 {
    self.jd as i64 % 7 // TODO: check the number that is 0, should be sunday. If not subtract an offset until it works
}

}

impl std::ops::Add<f64> for JulDate {
    type Output=JulDate;

    // Required method
    fn add(self, rhs: f64) -> Self::Output {
      Self {jd: self.jd + rhs}
    }
}

impl std::ops::Add<i64> for JulDate {
    type Output=JulDate;

    // Required method
    fn add(self, rhs: i64) -> Self::Output {
      Self {jd: self.jd + rhs as f64 }
    }
}

impl std::ops::Sub for JulDate {
    type Output=f64;

    // Required method
    fn sub(self, rhs: Self) -> f64 {
      self.jd - rhs.jd
    }
}

impl std::default::Default for JulDate {
  fn default() -> Self {
    Self { jd: 0.0 }
  }
}



struct YearMonthDay {
  year: i32,
  month: i32,
  day: i32,
}

impl YearMonthDay {
  pub fn new(year: i32, month: i32, day: i32) -> Self {
    // TODO validation lol
  Self {year, month, day}
    }
}


  /**
   * Compare Julian Dates for equality.
   *
   * @param jd The Julian Date to be compared with this.
   * @param rel_tol Relative tolerance.
   * @param abs_tol Absolute tolerance.
   * @return True if the dates are equivalent.
   *
   */
/*
  bool IsClose(JD const& jd, double const& rel_tol = DateTimeBase::rel_fod,
               double const& abs_tol = 0.0) const
  {
    if (*this == jd)
    {
      return true;
    }
    return this->jdn == jd.GetJDN() &&
        DateTimeBase::isclose(this->fod, jd.GetFoD(), rel_tol, abs_tol);
  }
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let jd0: JulDate = JulDate::default();
        let y2k: JulDate = JulDate::from_gregorian_date(YearMonthDay::new(2000, 1, 1));
        let y2kp1: JulDate = JulDate::from_gregorian_date(YearMonthDay::new(2000, 1, 2));
        let y2k_feb1: JulDate = JulDate::from_gregorian_date(YearMonthDay::new(2000, 2, 1));
        let y2k_feb2: JulDate = JulDate::from_gregorian_date(YearMonthDay::new(2000, 2, 2));
        let y2k_feb28: JulDate = JulDate::from_gregorian_date(YearMonthDay::new(2000, 2, 28));
        let y2k_feb29: JulDate = JulDate::from_gregorian_date(YearMonthDay::new(2000, 2, 29));
        let y2k_mar01: JulDate = JulDate::from_gregorian_date(YearMonthDay::new(2000, 3, 1));
        
        let y2k_dec31: JulDate = JulDate::from_gregorian_date(YearMonthDay::new(2000, 12, 31));
        let y2001_01_01: JulDate = JulDate::from_gregorian_date(YearMonthDay::new(2001, 1, 1));

        assert_eq!(jd0.jd, 0.0);
        assert_eq!(y2k.jd, 2451544.5000000);
        assert_eq!(y2kp1.jd, 2451545.5000000);
        assert_eq!(y2k + 1, y2kp1);
        assert_eq!(y2k_feb1 + 1, y2k_feb2);
        assert_eq!(y2k_feb28 + 1, y2k_feb29);
        assert_eq!(y2k_feb29 + 1, y2k_mar01);
        assert_eq!(y2k_dec31 + 1, y2001_01_01);
    }
}
