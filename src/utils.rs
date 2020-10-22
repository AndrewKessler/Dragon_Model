//initialising values
// USD based prices per block per rig
pub const REWARD: f64 = 400.0;
//to prevent f64 errors on u64 operations we divide
// setting OPEX_COST to 0.001 really shows off capital optimisation derivative func
pub const OPEX_COST: f64 = 0.0005;
//what percent of the inital rate of profit sets optimum capital investment
pub const OPT_PERCENT: f64 = 0.25;
//const BLOCKS_PER_WEEK: u64 = 20160;
//const COST_PER_ASIC: u64 = 3000;

pub fn calc_percent_network(dragon_rigs_deployed: u64, network_size: u64) -> f64 {
    let n = dragon_rigs_deployed as f64 / network_size as f64;
    n / (n + 1.0)
}

pub fn profit(number_of_rigs: u64, current_network_size: u64) -> f64 {
    let percent_network: f64 = calc_percent_network(number_of_rigs, current_network_size);
    let reward: f64 = percent_network * REWARD;
    let cost: f64 = number_of_rigs as f64 * OPEX_COST;
    let profit: f64 = reward - cost;
    profit
}

pub fn optimise_capital(
    current_network_size: u64,
    best_rig_number: &f64,
    mut opt_rig_number: u64,
) -> u64 {
    //optimise capital investment on derivative
    let mut i_profit: f64 = 0.0;
    let i_percent_network: f64 = calc_percent_network(2, current_network_size);
    let i_reward: f64 = i_percent_network * REWARD;
    let i_cost: f64 = 2.0 * OPEX_COST;
    let j_profit: f64 = i_reward - i_cost;
    let first_derivative: f64 = (j_profit - i_profit) / 2.0;
    println!("first derivative: {}", first_derivative);
    //use the best rate of change to autobenchmark

    //TODO custom dragon rate selection
    let cap_optimum: f64 = first_derivative * OPT_PERCENT;

    println!("optimal derivative: {}", cap_optimum);
    //calc first derivative


    for i in (2..best_rig_number.round() as u64).step_by(2) {
        let i_percent_network: f64 = calc_percent_network(i, current_network_size);
        let i_reward: f64 = i_percent_network * REWARD;
        let i_cost: f64 = i as f64 * OPEX_COST;
        let j_profit: f64 = i_reward - i_cost;
        let first_derivative: f64 = (j_profit - i_profit) / 2.0;
        //println!("derivative: {}", first_derivative);
        i_profit = j_profit;
        //check for capital cap_optimum
        if cap_optimum > first_derivative {
            opt_rig_number = i;
            break;
        }

        //println!("profit k: {} sec derivative: {}", i_profit, first_derivative_b);
    } //end for loop

    let o_profit = profit(opt_rig_number, current_network_size);
    println!("optimal profit {}", o_profit);
    opt_rig_number
}
