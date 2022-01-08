use rand::Rng;

fn main() {
    let actions = 4;
    let states = 6;
    let termination_state = states-1;

    let mut q = vec![vec![0.0; actions]; states];
    let mut r = vec![vec![-1; actions]; states];
    let mut ns = vec![vec![0; actions]; states];

    init(&mut r, &mut ns);

    let epochs = 1_000;

    let mut rng = rand::thread_rng();
    let mut state = rng.gen_range(0..states);

    for _ in 0..epochs-1 {
        state = stuff(state, &mut q, &r, &ns);
        // println!("Next State {}", state);
        while state != termination_state {
            state = stuff(state, &mut q, &r, &ns);
            // println!("Next State {}", state);
        }
    }

    display_matrix(&q);

    let mut rng = rand::thread_rng();
    let mut next = rng.gen_range(0..states);
    println!("Starting in state {}", next);
    next = get_path(next, &q, &ns);
    while next != termination_state {
        next = get_path(next, &q, &ns);
    }
}

fn get_path(state: usize, q: &Vec<Vec<f64>>, ns: &Vec<Vec<usize>>) -> usize {
    let mut max_value = f64::MIN;
    let mut max_col = 0;
    for col in 0..q[state].len() {
        let current_value = q[state][col];
        if current_value > max_value {
            max_value = current_value;
            max_col = col;
        }
    }

    let next_state = ns[state][max_col];
    
    let direction: &str;

    match max_col {
        0 => direction = "Up",
        1 => direction = "Down",
        2 => direction = "Left",
        _ => direction = "Right"
    }

    // println!("Max Value {}, Next State {}", max_value, next_state);
    println!("Move {} to state {}", direction, next_state);

    next_state
}

fn stuff(state: usize, q: &mut Vec<Vec<f64>>, r: &Vec<Vec<i32>>, ns: &Vec<Vec<usize>>) -> usize {
    let alpha = 0.1;
    let gamma = 0.6;
    let actions = q[0].len();

    // Step 2: Pick an action (use random for now)
    let mut rng = rand::thread_rng();
    let action = rng.gen_range(0..actions);

    // Step 3: Based on action, determine next state (hardcode state 1 for now)
    let next_state = ns[state][action];
    // let next_state = 1;

    // Step 4: Choose highest action reward from next state
    let next_action = get_highest_reward(next_state, r);

    // Step 5: Update q
    q[state][action] = q[state][action] + alpha * (r[state][action] as f64 + gamma * q[next_state][next_action] - q[state][action]);

    // display_matrix(q);

    next_state
}

fn display_matrix(mat: &Vec<Vec<f64>>) {
    for state in 0..mat.len() {
        for action in 0..mat[0].len() {
            print!("{:2} ", mat[state][action]);
        }
        println!();
    }
}

fn get_highest_reward(state: usize, r: &Vec<Vec<i32>>) -> usize{
    // let mut max_value = i32::MIN;
    let mut max_value = 0;
    let mut max_col = 0;
    for col in 0..r[state].len() {
        let current_value = r[state][col];
        if current_value > max_value {
            max_value = current_value;
            max_col = col;
        }
    }

    // println!("Max Value {}, Max Index {}", max_value, max_col);

    max_col
}

fn init(r: &mut Vec<Vec<i32>>, ns: &mut Vec<Vec<usize>>) {
    const WALL: i32 = -10;
    const REWARD: i32 = 10;

    r[0][0] = WALL;
    r[0][2] = WALL; 

    r[1][0] = WALL;

    r[2][0] = WALL;
    r[2][1] = REWARD;
    r[2][3] = WALL;

    r[3][1] = WALL;
    r[3][2] = WALL;
    
    r[4][1] = WALL;
    r[4][3] = REWARD;

    r[5][1] = WALL;
    r[5][3] = WALL;

    // up, down left, right
    ns[0][0] = 0;
    ns[0][1] = 3;
    ns[0][2] = 0;
    ns[0][3] = 1;

    ns[1][0] = 1;
    ns[1][1] = 4;
    ns[1][2] = 0;
    ns[1][3] = 2;

    ns[2][0] = 2;
    ns[2][1] = 5;
    ns[2][2] = 1;
    ns[2][3] = 3;

    ns[3][0] = 0;
    ns[3][1] = 3;
    ns[3][2] = 3;
    ns[3][3] = 4;

    ns[4][0] = 1;
    ns[4][1] = 4;
    ns[4][2] = 3;
    ns[4][3] = 5;

    ns[5][0] = 2;
    ns[5][1] = 5;
    ns[5][2] = 4;
    ns[5][3] = 5;
}
