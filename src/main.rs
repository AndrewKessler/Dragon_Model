use rand::Rng;

//initialising values
// USD based prices per block per rig
const REWARD: f64 = 400.0;
const MAX_PROFIT_REDUCTION: f64 = 0.85;
const CAPITAL_REDUCTION: f64 = 0.7;
//to prevent f64 errors on u64 operations we divide
// setting OPEX_COST to 0.001 really shows off capital optimisation derivative func
const OPEX_COST: f64 = 0.01;
//what percent of the inital rate of profit sets optimum capital investment
const OPT_PERCENT: f64 = 0.25;
//const BLOCKS_PER_WEEK: u64 = 20160;
//const COST_PER_ASIC: u64 = 3000;

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

fn calc_percent_network(dragon_rigs_deployed: u64, network_size: u64) -> f64 {
    let n = dragon_rigs_deployed as f64 / network_size as f64;
    n / (n + 1.0)
}

fn opt_cap(best_rig_number: f64, current_network_size: u64) -> f64 {
    let optimal_rig_number: f64 = best_rig_number * CAPITAL_REDUCTION;
    let optimal_percent_network: f64 =
        calc_percent_network(optimal_rig_number as u64, current_network_size);
    let optimal_reward: f64 = optimal_percent_network * REWARD;

    let optimal_cost: f64 = optimal_rig_number * OPEX_COST;
    let new_profit: f64 = optimal_reward - optimal_cost;
    new_profit
}
fn profit (number_of_rigs: u64, current_network_size: u64) -> f64 {
    let percent_network: f64 = calc_percent_network(number_of_rigs, current_network_size);
    let reward: f64 = percent_network * REWARD;
    let cost: f64 = number_of_rigs as f64 * OPEX_COST;
    let profit: f64 = reward - cost;
    profit
}

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
    let cap_optimum: f64 = first_derivative*OPT_PERCENT;

    println!("optimal derivative: {}", cap_optimum);
    //calc first derivative
    for i in (2..best_rig_number as u64).step_by(2) {
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

    println!("optimal rig number {}", opt_rig_number);
    let o_profit = profit(opt_rig_number, current_network_size);
    println!("optimal profit {}", o_profit);
    println!("");
    //add to dragon pool if reasonable
    if best_rig_number > 0.0 {
        let commit_dragon = Dragon::new(
            true,
            trial_dragon.total_mining_rigs,
            best_rig_number as u64,
            best_percent,
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
    percent_current_network: f64,
    //min_profit: f64,
}

impl Dragon {
    fn new(
        participant: bool,
        total_mining_rigs: u64,
        deployed_mining_rigs: u64,
        percent_current_network: f64,
    ) -> Dragon {
        Dragon {
            participant,
            total_mining_rigs,
            deployed_mining_rigs,
            percent_current_network,
        }
    }
    /*
        fn update_percent_network (&mut self, network_size: u64) {
            let n = self.deployed_mining_rigs as f64/network_size as f64;
            self.percent_current_network = n/(n+1.0);
        }
    */
}

//initialising network types, methods and functions
#[derive(Debug, Clone)]
struct Graph {
    all_mining_rigs: u64,
    number_of_dragons: u64,
}

impl Graph {
    fn new(all_mining_rigs: u64, number_of_dragons: u64) -> Graph {
        Graph {
            all_mining_rigs,
            number_of_dragons,
        }
    }

    fn update_amr(&mut self, new_dragon_rig: u64) {
        self.all_mining_rigs += new_dragon_rig;
    }
}

fn main() {
    //initialise dragon pool

    //

    let mut dragon_pool = Vec::new();

    //initial network conditions
    let mut network = Graph::new(10000, 1);

    //instantiate dragon
    spawn_dragon(network.all_mining_rigs, &mut dragon_pool);
    network.update_amr(dragon_pool[0].deployed_mining_rigs);
    println!("new network size {:?}", network.all_mining_rigs);
    //TODO after first dragon spawn each new dragon must force older dragons to re-evalute decision making

    //add dragon to dragon pool
    println!("number of dragons: {}", dragon_pool.len());
    println!("Some dragon pool values {:?}", dragon_pool[0]);
    println!(
        "deployed_mining_rigs {:?}",
        dragon_pool[0].deployed_mining_rigs
    );

    println!("");
}
