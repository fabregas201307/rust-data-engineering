extern crate serde;
extern crate serde_json;
extern crate polars;
use log::{debug, error, info, warn};
use pyo3::exceptions::PyOSError;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3_log;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use ndarray::{Array, Array2};
use ndarray_stats:SummaryStatisticsExt;
use statrs::statistics::Statistics;
use statsmodels::Regression

use std::fmt;
use quantlib::quantlib::QuantLib;

//define a class with name TaxCalculator, which will be used in Python
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
    cusip="": String,
    day_count="30/360": String,
    data_first_call_at_par="1901-01-01": String,
    frequency=2: i32,
    tax_exempt=false: bool,
    include_MD_accrual_in_income=false: bool,
    use_clean_price=false: bool,
    use_adjusted_ytm=true: bool,
    simulate_kaotay=false: bool,
    market_discrount_menthod="ratable_accrual": String,
    acquisition_premium_method="scaled_oid": String,
}
// define a init function for the class TaxCalculator,  if coupon_rate < 0, raise an error
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
        } else if coupon_rate == 0.0 {
            use_adjusted_ytm = false;  // TODO: improve this
        }
        let _coupon_rate = coupon_rate;

        // prcess dates with quantlib library
        let ql = QuantLib::new();
        let mut _date_start = ql.get_ql_date(date_start);
        let mut _date_first_coupon = ql.get_ql_date(date_first_coupon);
        let mut _date_purchase = ql.get_ql_date(date_purchase);
        let mut _date_maturity = ql.get_ql_date(date_maturity);
        let mut _date_first_call_at_par = ql.get_ql_date(data_first_call_at_par);

        // check if _date_first_coupon equals to date_first_coupon


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