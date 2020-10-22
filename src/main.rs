use rand::Rng;
pub mod utils;
use utils::*;

//initialising functions
fn random_num(min: u64, max: u64) -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min, max)
}

/*
fn probability (min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    let rnd = rng.gen();
    if min<=rnd && rnd<=max {
        return rnd;
    } else {probability (min, max)}
}
*/

fn get_best_rig_number(
    current_network_size: u64,
    trial_dragon: Dragon,
    mining_rigs_on_hand: u64,
) -> (f64, f64) {
    //99% of the network is achieved with a hundred fold investment. No rational dragon will invest more
    //in weeks where a reasonable invester expects to offset capital cost of mining within 1 to 5 years
    //let break_even_period = 52*random_num(1,6);
    let mut profit_on_rigs_n0: f64 = 0.0;
    let mut profit_rigs_n1: f64 = 0.0;
    let mut best_rig_number: f64 = 0.0;
    let mut best_percent: f64 = 0.0;
    let mut percent_network: f64 = 0.0;
    let mut change_in_profit: f64 = 0.0;
    let mut total_cost: f64 = 0.0;
    let mut total_reward: f64 = 0.0;
    //    println!("trial dragon stats {:?}", trial_dragon);

    //niavely determine number of rigs that optimse profit for any capital amount possible

    for i in 0..mining_rigs_on_hand {
        percent_network = calc_percent_network(i, current_network_size);
        //calculate current proft
        total_cost = i as f64 * OPEX_COST; //+ break_even_period*OPEX_COST*BLOCKS_PER_WEEK)).into();
        total_reward = percent_network * REWARD;
        profit_rigs_n1 = total_reward - total_cost;
        //simple local maximum finder
        change_in_profit = profit_rigs_n1 - profit_on_rigs_n0;
        profit_on_rigs_n0 = profit_rigs_n1;
        //store optimal value
        if change_in_profit < 0.0 {
            println!("profit negative from rig number: {} onwards", i);
            best_rig_number = i as f64;
            best_percent = percent_network;
            break;
        }
    }
    println!("");
    println!("profit_on_rigs_n0 {}", profit_on_rigs_n0);
    println!("best rig number {}", best_rig_number);
    println!("");
    (best_percent, best_rig_number)
}

//create a potential dragon thinking about joining
fn spawn_dragon(current_network_size: u64, dragon_pool: &mut Vec<Dragon>) {
    let mining_rigs_on_hand: u64 = 100 * random_num(1, current_network_size);
    let mut trial_dragon = Dragon::new(false, mining_rigs_on_hand, 0, 0.0);
    let (best_percent, best_rig_number) =
        get_best_rig_number(current_network_size, trial_dragon, mining_rigs_on_hand);
    let mut opt_rig_number: u64 = 0;

    let cap_opt_rig_number =
        optimise_capital(current_network_size, best_rig_number, opt_rig_number);

    println!("optimal cap rig number {}", cap_opt_rig_number);
    println!("");

    //add to dragon pool if reasonable
    if best_rig_number > 0.0 {
        let commit_dragon = Dragon::new(
            true,
            trial_dragon.total_mining_rigs,
            cap_opt_rig_number as u64,
            0.0,
        );
        dragon_pool.push(commit_dragon);
    }

    println!("trial dragon stats {:?}", trial_dragon);
}

//initialising dragon types, methods and functions
#[derive(Debug, Copy, Clone)]
struct Dragon {
    participant: bool,
    total_mining_rigs: u64,
    deployed_mining_rigs: u64,
    capital_repayment_period: f64,
}

impl Dragon {
    fn new(
        participant: bool,
        total_mining_rigs: u64,
        deployed_mining_rigs: u64,
        capital_repayment_period: f64,
    ) -> Dragon {
        Dragon {
            participant,
            total_mining_rigs,
            deployed_mining_rigs,
            capital_repayment_period,
        }
    }
}

fn count_all_rigs(dragon_pool: &Vec<Dragon>, mut total_rigs: u64) -> u64 {
    total_rigs = 0;
    for dragon in dragon_pool {
        total_rigs += dragon.deployed_mining_rigs;
    }
    total_rigs
}

fn main() {
    //initialise dragon pool
    let mut dragon_pool = Vec::new();
    let mut total_rigs = 0;

    //initial network of individual miners collectively represent the first dragon
    let mut network = Dragon::new(true, 10000, 10000, 0.0);
    dragon_pool.push(network);
    total_rigs = count_all_rigs(&dragon_pool, total_rigs);
    //instantiate dragon
    spawn_dragon(total_rigs, &mut dragon_pool);
    println!("total network size after count: {}", total_rigs);
    //TODO after first dragon spawn each new dragon must force older dragons to re-evalute decision making

    //add dragon to dragon pool
    println!("number of dragons: {}", dragon_pool.len());
    println!("Some dragon pool values {:?}", dragon_pool[0]);
    println!(
        "deployed_mining_rigs {:?}",
        dragon_pool[0].deployed_mining_rigs
    );

    total_rigs = count_all_rigs(&dragon_pool, total_rigs);
    println!("total network size after count: {}", total_rigs);
}
