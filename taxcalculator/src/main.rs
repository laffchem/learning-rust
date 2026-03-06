
fn main() {
    let jv_old = 382_947.00;
    let av_old = 275_140.00;
    let jv_new = 505_000.00;
    let loan_amount: f64 = 505_000.00;
    let interest_rate = 4.75;
    let loan_term_years = 30.0;
    let home_insurance = 1800.00;
    let principal_interest = principal_interest_calculator(loan_amount, interest_rate, loan_term_years);
    let taxes = portability_calculator(jv_old, av_old, jv_new);
    let monthly_home_insurance = home_insurance / 12.0;
    let monthly_hoa = 93.00;
    let total_monthly_payment = principal_interest + taxes + monthly_home_insurance + monthly_hoa;
    let monthly_no_hoa: f64 = principal_interest + taxes + monthly_home_insurance;
    let total_difference: f64 = total_monthly_payment - (3064.00 + 104.00);
    println!("Monthly Payment without HOA: ${}", format!("{:.2}", monthly_no_hoa));
    println!("Total Monthly Payment (PITI): ${}", format!("{:.2}", total_monthly_payment));
    println!("Total difference in Payment: ${}", format!("{:.2}", total_difference))

}

fn portability_calculator(jv_old: f64, av_old: f64, jv_new: f64 ) -> f64 {
    let soh_diff = jv_old - av_old;
    let port_amount = f64::min(soh_diff, 500000.00);
    let tax_rate = 14.0625;
    let homestead_exemption = 55000.00;
    if jv_new >= jv_old {
        println!("Portability Amount: ${}", format!("{:.2}", port_amount));
        let av_new = jv_new - port_amount - homestead_exemption;
        println!("New Assessed Value: ${}", format!("{:.2}", av_new));
        let annual_tax = (av_new * tax_rate) / 1000.00;
        println!("New Annual Tax Amount: ${}", format!("{:.2}", annual_tax));
        let monthly_tax = annual_tax / 12.0;    
        println!("New Monthly Tax Amount: ${}", format!("{:.2}", monthly_tax));
        return monthly_tax;
    } else {
        let adjusted_port_amount = (port_amount / jv_old) * jv_new;
        println!("New Portability Amount: ${}", format!("{:.2}", adjusted_port_amount));
        let av_new = jv_new - adjusted_port_amount - homestead_exemption;
        println!("New Assessed Value: ${}", format!("{:.2}", av_new));
        let annual_tax = (av_new * tax_rate) / 1000.00;
        println!("New Annual Tax Amount: ${}", format!("{:.2}", annual_tax));
        let monthly_tax = annual_tax / 12.0;
        println!("New Monthly Tax Amount: ${}", format!("{:.2}", monthly_tax));
        return monthly_tax;
    }
}

fn principal_interest_calculator(loan_amount: f64, interest_rate: f64, loan_term_years: f64) -> f64 {
    let monthly_interest_rate = interest_rate / 100.0 / 12.0;
    let number_of_payments = loan_term_years * 12.0;
    let numerator = monthly_interest_rate * (1.0 + monthly_interest_rate).powf(number_of_payments);
    let denominator = (1.0 + monthly_interest_rate).powf(number_of_payments) - 1.0;
    let monthly_payment = loan_amount * (numerator / denominator);
    println!("Monthly Principal & Interest Payment: ${}", format!("{:.2}", monthly_payment));
    return monthly_payment;
}