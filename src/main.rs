//two MASSIVE errors. deployed_mining_rigs should not behave deterministically (it won't after round update. not so bad)
// round update must be "live" dragon by dragon. Old function is only round by round. count(immutable dragon_pool which is bad)

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
    let break_even: i64 = p - c;

    if break_even > 0 {
        //println!("saving dragon as: {}", trial_dragon.capital_repayment_period);
        let commit_dragon = Dragon::new(
            dragon_pool.len() as u64,
            true,
            trial_dragon.total_mining_rigs,
            cap_opt_rig_number as u64,
            trial_dragon.capital_repayment_period,
        );
        dragon_pool.push(commit_dragon);
    } else if break_even <= 0 {
        let commit_dragon = Dragon::new(
            dragon_pool.len() as u64,
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
//TODO check network_size = 0 or 1
if current_network_size < 1 {println!("Panik net size: {}", current_network_size);}
    let mining_rigs_on_hand: u64 = 100 * random_num(1, current_network_size + 2);
    //in weeks (9 - 24 months) low bounding 4 weeks a month on 9 and high bounding 4.5 weeks on 24
    let repayment_period: u64 = random_num(36, 108);
    let mut trial_dragon = Dragon::new(0, false, mining_rigs_on_hand, 0, repayment_period);

    let cap_opt_rig_number = optimise_capital(current_network_size, &(mining_rigs_on_hand as f64));
    let mut total_rigs = 0;
    total_rigs = count_all_rigs(&dragon_pool, total_rigs);
//doubled up on breakeven alaysis
    /*let prof = profit(cap_opt_rig_number, total_rigs);
    let c: i64 = COST_PER_ASIC as i64 * cap_opt_rig_number as i64;
    let p: i64 =
        prof as i64 * BLOCKS_PER_WEEK as i64 * trial_dragon.capital_repayment_period as i64;
    let break_even: i64 = p - c;
    //println!("break_even: {}", break_even);*/
//    if break_even > 0 {
        commit(trial_dragon, cap_opt_rig_number, dragon_pool);
//    } //else if break_even <= 0 {
      //    }
}

//initialising dragon types, methods and functions
#[derive(Debug, Copy, Clone)]
struct Dragon {
    dragon_ID: u64,
    participant: bool,
    total_mining_rigs: u64,
    deployed_mining_rigs: u64,
    capital_repayment_period: u64,
}

impl Dragon {
    fn new(
        dragon_ID: u64,
        participant: bool,
        total_mining_rigs: u64,
        deployed_mining_rigs: u64,
        capital_repayment_period: u64,
    ) -> Dragon {
        Dragon {
            dragon_ID,
            participant,
            total_mining_rigs,
            deployed_mining_rigs,
            capital_repayment_period,
        }
    }
}
//clean
fn count_all_rigs(dragon_pool: &Vec<Dragon>, mut total_rigs: u64) -> u64 {
    total_rigs = 0;
    for dragon in dragon_pool {
        if dragon.participant == true {
            total_rigs += dragon.deployed_mining_rigs;
        }
    }
    total_rigs
}


fn round_update(dragon_pool: &mut Vec<Dragon>) {
let mut counter = 0;
let dragon_clone = dragon_pool.clone();
let mut round_network_size = 0;

    //get number of current miners at start
    for dragon in dragon_clone {
        if dragon.participant == true {
            round_network_size += dragon.deployed_mining_rigs;
        }
    }
    //update miner stats. Use clone
    for dragon in dragon_pool.iter_mut() {
        counter += 1;
        let cap_opt_rig_number = optimise_capital(round_network_size, &(dragon.total_mining_rigs as f64));

        let current_profit = profit(cap_opt_rig_number, round_network_size);
        let c: i64 = COST_PER_ASIC as i64 * cap_opt_rig_number as i64;
        let p: i64 =
        //cap repayment in weeeks
            current_profit as i64 * BLOCKS_PER_WEEK as i64 * dragon.capital_repayment_period as i64;
        let break_even: i64 = p - c;
            println!("break_even: {} on dragon: {} particitation: {}", break_even, counter, dragon.participant );
        if break_even > 0 {
            dragon.participant = true;
            //TODO update deployed rig number
            round_network_size -= dragon.deployed_mining_rigs;
            dragon.deployed_mining_rigs = cap_opt_rig_number;
            round_network_size += dragon.deployed_mining_rigs;
        }
        else if break_even < 0 {
            dragon.participant = false;
            round_network_size -= dragon.deployed_mining_rigs;
            dragon.deployed_mining_rigs = 0;
        }
    }

}

fn main() {
    //initialise dragon pool
    let mut dragon_pool = Vec::new();
    let mut total_rigs = 0;
    let mut halt: bool = false;
    //initial network of individual miners collectively represent the first dragon
    let mut network = Dragon::new(0, true, 10000, 10000, 15);
    dragon_pool.push(network);
    total_rigs = count_all_rigs(&dragon_pool, total_rigs);

    //instantiate dragon
    spawn_dragon(total_rigs, &mut dragon_pool);
    let mut dragon_sum = 0;
    //after first dragon each successive dragon could create unbalancing of prev dragons
    // equilibrium must be found before moving on
    for i in 1..20 {
        println!("loop NUMBER: {}", i);
        dragon_sum = 0;
        //while !halt {

        round_update(&mut dragon_pool);

        //check if round is stable enough
        for dragon in &dragon_pool {
            dragon_sum += 1;
        }
        println!("dragon_sum: {}", dragon_sum);
        //println!("dragon_sum: {} & dragon_crash: {}", dragon_sum, dragon_crash);
        //}//end while loop
        //        halt = false;
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
