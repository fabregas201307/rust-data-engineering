use serde::{Deserialize, Serialize};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3_log;
// use quantlib::quantlib::QuantLib;
// use quantlib::QuantLib;
// use quantlib::instruments::FixedRateBond;
use quantlib;

// use statrs::statistics::Statistics;
// use statsmodels::Regression;

// use ndarray::Array2;
// use ndarray_stats::SummaryStatisticsExt;

#[pyclass]
#[derive(Debug, Serialize, Deserialize)]
struct TaxCalculator {
    coupon_rate: f64,
    date_start: String,
    date_first_coupon: String,
    date_purchase: String,
    date_maturity: String,
    price_issue: f64,
    price_purchase: f64,
    price_maturity: f64,
    cusip: String,
    day_count: String,
    data_first_call_at_par: String,
    frequency: i32,
    tax_exempt: bool,
    include_MD_accrual_in_income: bool,
    use_clean_price: bool,
    use_adjusted_ytm: bool,
    simulate_kaotay: bool,
    market_discrount_menthod: String,
    acquisition_premium_method: String,
}

#[pymethods]
impl TaxCalculator {
    #[new]
    fn new(
        coupon_rate: f64,
        date_start: String,
        date_first_coupon: String,
        date_purchase: String,
        date_maturity: String,
        price_issue: f64,
        price_purchase: f64,
        price_maturity: f64,
        cusip: String,
        day_count: String,
        data_first_call_at_par: String,
        frequency: i32,
        tax_exempt: bool,
        include_MD_accrual_in_income: bool,
        use_clean_price: bool,
        use_adjusted_ytm: bool,
        simulate_kaotay: bool,
        market_discrount_menthod: String,
        acquisition_premium_method: String,
    ) -> Self {
        if coupon_rate < 0.0 {
            panic!("Coupon rate must be greater than or equal to zero");
        } else{
            println!("Coupon rate is: {}", coupon_rate);
        }
        
        let _coupon_rate = coupon_rate;
        // prcess dates with quantlib library
        let ql = QuantLib::new();
        let mut _date_start = ql.get_ql_date(date_start);
        let mut _date_first_coupon = ql.get_ql_date(date_first_coupon);
        let mut _date_purchase = ql.get_ql_date(date_purchase);
        let mut _date_maturity = ql.get_ql_date(date_maturity);
        let mut _date_first_call_at_par = ql.get_ql_date(data_first_call_at_par);

        TaxCalculator {
            coupon_rate,
            date_start,
            date_first_coupon,
            date_purchase,
            date_maturity,
            price_issue,
            price_purchase,
            price_maturity,
            cusip,
            day_count,
            data_first_call_at_par,
            frequency,
            tax_exempt,
            include_MD_accrual_in_income,
            use_clean_price,
            use_adjusted_ytm,
            simulate_kaotay,
            market_discrount_menthod,
            acquisition_premium_method,
        }
    }
}
