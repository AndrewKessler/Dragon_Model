use rand::Rng;
pub mod utils;
use utils::*;

//initialising functions
fn random_num(min: u64, max: u64) -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min, max)
}

fn commit(trial_dragon: Dragon, cap_opt_rig_number: u64, dragon_pool: &mut Vec<Dragon>) {
    //decision tree. look for cap repayment. elses op repayment closest to cap point. else eject
    let mut total_rigs = 0;
    total_rigs = count_all_rigs(&dragon_pool, total_rigs);
    let prof = profit(cap_opt_rig_number, total_rigs);
    let c: i64 = COST_PER_ASIC as i64 * cap_opt_rig_number as i64;
    let p: i64 =
        prof as i64 * BLOCKS_PER_WEEK as i64 * trial_dragon.capital_repayment_period as i64;
    let break_even: i64 = c - p;

    if break_even <= 0 {
        //println!("saving dragon as: {}", trial_dragon.capital_repayment_period);
        let commit_dragon = Dragon::new(
            true,
            trial_dragon.total_mining_rigs,
            cap_opt_rig_number as u64,
            trial_dragon.capital_repayment_period,
        );
        dragon_pool.push(commit_dragon);

    } else if break_even > 0 {
        let commit_dragon = Dragon::new(
            false,
            trial_dragon.total_mining_rigs,
            0,
            trial_dragon.capital_repayment_period,
        );
        dragon_pool.push(commit_dragon);
    }

}

//create a potential dragon thinking about joining
fn spawn_dragon(current_network_size: u64, dragon_pool: &mut Vec<Dragon>) {
    let mining_rigs_on_hand: u64 = 100 * random_num(1, current_network_size);
    let repayment_period: u64 = random_num(9, 24);
    let mut trial_dragon = Dragon::new(false, mining_rigs_on_hand, 0, repayment_period);

    let cap_opt_rig_number = optimise_capital(current_network_size, &(mining_rigs_on_hand as f64));

    commit(trial_dragon, cap_opt_rig_number, dragon_pool);
}

//initialising dragon types, methods and functions
#[derive(Debug, Copy, Clone)]
struct Dragon {
    participant: bool,
    total_mining_rigs: u64,
    deployed_mining_rigs: u64,
    capital_repayment_period: u64,
}

impl Dragon {
    fn new(
        participant: bool,
        total_mining_rigs: u64,
        deployed_mining_rigs: u64,
        capital_repayment_period: u64,
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

fn round_update(dragon_pool: &mut Vec<Dragon>) {
    let dragon_clone = dragon_pool.clone();
    for dragon in dragon_pool.iter_mut() {
        let mut total_rigs = 0;
        total_rigs = count_all_rigs(&dragon_clone, total_rigs) - dragon.deployed_mining_rigs;

        let cap_opt_rig_number = optimise_capital(total_rigs, &(dragon.total_mining_rigs as f64));

        let current_profit = profit(cap_opt_rig_number, total_rigs);
        let c: i64 = COST_PER_ASIC as i64 * cap_opt_rig_number as i64;
        let p: i64 =
            current_profit as i64 * BLOCKS_PER_WEEK as i64 * dragon.capital_repayment_period as i64;
        let break_even: i64 = p - c;

        if break_even > 0 {
            dragon.participant = true;
            //TODO update deployed rig number
            dragon.deployed_mining_rigs = cap_opt_rig_number;
        } else if break_even < 0 {
            dragon.participant = false;
        }
    }
}

fn main() {
    //initialise dragon pool
    let mut dragon_pool = Vec::new();
    let mut total_rigs = 0;
    let mut halt: bool = false;
    //initial network of individual miners collectively represent the first dragon
    let mut network = Dragon::new(true, 10000, 10000, 15);
    dragon_pool.push(network);

    //I don't want to loose total_rigs by passing it to some fn.
    //I want to actually calc "break even" period using this number in fn commit()
    total_rigs = count_all_rigs(&dragon_pool, total_rigs);
    //instantiate dragon
    spawn_dragon(total_rigs, &mut dragon_pool);

    //after first dragon each successive dragon could create unbalancing of prev dragons
    // equilibrium must be found before moving on
    for i in 1..20 {
        println!("loop NUMBER: {}", i);
        let mut dragon_sum = 0;
        let mut dragon_crash = 0;
        while !halt {
            round_update(&mut dragon_pool);

            //check if round is stable enough
            for dragon in &dragon_pool {
                dragon_sum += 1;
                if dragon.participant == false {
                    dragon_crash += 1;
                }
                if dragon_crash < 2 {
                    halt = true;
                }
            }
        }
        //instantiate dragon
        total_rigs = count_all_rigs(&dragon_pool, total_rigs);
        spawn_dragon(total_rigs, &mut dragon_pool);
    }

    for dragon in &dragon_pool {
        println!("Pool entry: {:?}", dragon);
    }

    total_rigs = count_all_rigs(&dragon_pool, total_rigs);
    println!("total network size after count: {}", total_rigs);
}
