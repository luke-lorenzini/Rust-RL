use rand::Rng;

fn main() {
    let actions = 4;
    let states = 36;
    let termination_state = states-1;

    let mut q = vec![vec![0.0; actions]; states];
    let mut r = vec![vec![-1; actions]; states];
    let mut ns = vec![vec![0; actions]; states];

    init(&mut q, &mut r, &mut ns);

    let epochs = 100_000;

    let mut rng = rand::thread_rng();
    let mut state = rng.gen_range(0..states);

    // Make sure we don't start in the termination state
    while state == termination_state {
        state = rng.gen_range(0..states);
        // println!("Chose wrong start state");
    }

    for _ in 0..epochs-1 {
        state = stuff(state, &mut q, &r, &ns);
        while state != termination_state {
            state = stuff(state, &mut q, &r, &ns);
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

    let action = get_action(state, &q);

    // Step 3: Based on action, determine next state
    let next_state = ns[state][action];

    // Step 4: Choose highest action reward from next state
    let next_action = get_highest_reward(next_state, r);

    // Step 5: Update q
    q[state][action] = q[state][action] + alpha * (r[state][action] as f64 + gamma * q[next_state][next_action] - q[state][action]);

    next_state
}

fn get_action(state: usize, q: &Vec<Vec<f64>>) -> usize {
    let epsilon = 0.1;
    let mut rng = rand::thread_rng();
    let n2: f64 = rng.gen();
    // println!("n2 {}", n2);

    let action;

    if n2 < epsilon {
        // action = env.action_space.sample() # Explore action space
        let actions = q[0].len();
        let mut rng = rand::thread_rng();
        action = rng.gen_range(0..actions);
    }
    else {
        // action = np.argmax(q_table[state]) # Exploit learned values
        action = get_highest_q_index(state, &q);
    }

    action
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
    let mut max_value = i32::MIN;
    // let mut max_value = 0;
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

fn get_highest_q_index(state: usize, q: &Vec<Vec<f64>>) -> usize{
    let mut max_value = f64::MIN;
    // let mut max_value = 0;
    let mut max_col = 0;
    for col in 0..q[state].len() {
        let current_value = q[state][col];
        if current_value > max_value {
            max_value = current_value;
            max_col = col;
        }
    }

    // println!("Max Value {}, Max Index {}", max_value, max_col);

    max_col
}

fn init(q: &mut Vec<Vec<f64>>, r: &mut Vec<Vec<i32>>, ns: &mut Vec<Vec<usize>>) {
    const WALL: i32 = -10;
    const REWARD: i32 = 10;
    // const TRAP: i32 = -5;

    let mut rng = rand::thread_rng();
    // let n2: f64 = rng.gen_range(0.0..0.001);

    for row in 0..q.len() {
        for col in 0..q[row].len() {
            q[row][col] = rng.gen_range(0.0..0.001);
        }
    }

    // r[0][0] = WALL;
    // r[0][2] = WALL; 

    // r[1][0] = WALL;
    // r[1][3] = TRAP;

    // r[2][0] = WALL;
    // r[2][1] = REWARD;
    // r[2][3] = WALL;

    // r[3][1] = WALL;
    // r[3][2] = WALL;
    
    // r[4][1] = WALL;
    // r[4][3] = REWARD;

    // r[5][0] = TRAP;
    // r[5][1] = WALL;
    // r[5][3] = WALL;

    r[0][0] = WALL;
    r[1][0] = WALL;
    r[2][0] = WALL;
    r[3][0] = WALL;
    r[4][0] = WALL;
    r[5][0] = WALL;

    r[30][1] = WALL;
    r[31][1] = WALL;
    r[32][1] = WALL;
    r[33][1] = WALL;
    r[34][1] = WALL;
    r[35][1] = WALL;

    r[0][2] = WALL;
    r[6][2] = WALL;
    r[12][2] = WALL;
    r[18][2] = WALL;
    r[24][2] = WALL;
    r[30][2] = WALL;

    r[5][3] = WALL;
    r[11][3] = WALL;
    r[17][3] = WALL;
    r[23][3] = WALL;
    r[29][3] = WALL;
    r[35][3] = WALL;

    r[29][1] = REWARD;
    r[34][3] = REWARD;

    // up, down left, right
    ns[0][0] = 0;
    ns[0][1] = 6;
    ns[0][2] = 0;
    ns[0][3] = 1;

    ns[1][0] = 1;
    ns[1][1] = 7;
    ns[1][2] = 0;
    ns[1][3] = 2;

    ns[2][0] = 2;
    ns[2][1] = 8;
    ns[2][2] = 1;
    ns[2][3] = 3;

    ns[3][0] = 3;
    ns[3][1] = 9;
    ns[3][2] = 2;
    ns[3][3] = 4;

    ns[4][0] = 4;
    ns[4][1] = 10;
    ns[4][2] = 3;
    ns[4][3] = 5;

    ns[5][0] = 5;
    ns[5][1] = 11;
    ns[5][2] = 4;
    ns[5][3] = 5;

    ns[6][0] = 0;
    ns[6][1] = 12;
    ns[6][2] = 6;
    ns[6][3] = 7;

    ns[7][0] = 1;
    ns[7][1] = 13;
    ns[7][2] = 6;
    ns[7][3] = 8;

    ns[8][0] = 2;
    ns[8][1] = 14;
    ns[8][2] = 7;
    ns[8][3] = 9;

    ns[9][0] = 3;
    ns[9][1] = 15;
    ns[9][2] = 8;
    ns[9][3] = 10;

    ns[10][0] = 4;
    ns[10][1] = 16;
    ns[10][2] = 9;
    ns[10][3] = 11;

    ns[11][0] = 5;
    ns[11][1] = 17;
    ns[11][2] = 10;
    ns[11][3] = 11;

    ns[12][0] = 6;
    ns[12][1] = 18;
    ns[12][2] = 12;
    ns[12][3] = 13;

    ns[13][0] = 7;
    ns[13][1] = 19;
    ns[13][2] = 12;
    ns[13][3] = 14;

    ns[14][0] = 8;
    ns[14][1] = 20;
    ns[14][2] = 13;
    ns[14][3] = 15;

    ns[15][0] = 9;
    ns[15][1] = 21;
    ns[15][2] = 14;
    ns[15][3] = 16;

    ns[16][0] = 10;
    ns[16][1] = 22;
    ns[16][2] = 15;
    ns[16][3] = 17;

    ns[17][0] = 11;
    ns[17][1] = 23;
    ns[17][2] = 16;
    ns[17][3] = 17;

    ns[18][0] = 12;
    ns[18][1] = 24;
    ns[18][2] = 18;
    ns[18][3] = 19;

    ns[19][0] = 13;
    ns[19][1] = 25;
    ns[19][2] = 18;
    ns[19][3] = 20;

    ns[20][0] = 14;
    ns[20][1] = 26;
    ns[20][2] = 19;
    ns[20][3] = 21;

    ns[21][0] = 15;
    ns[21][1] = 27;
    ns[21][2] = 20;
    ns[21][3] = 22;

    ns[22][0] = 16;
    ns[22][1] = 28;
    ns[22][2] = 21;
    ns[22][3] = 23;

    ns[23][0] = 17;
    ns[23][1] = 29;
    ns[23][2] = 22;
    ns[23][3] = 23;

    ns[24][0] = 18;
    ns[24][1] = 30;
    ns[24][2] = 24;
    ns[24][3] = 25;

    ns[25][0] = 19;
    ns[25][1] = 31;
    ns[25][2] = 24;
    ns[25][3] = 26;

    ns[26][0] = 20;
    ns[26][1] = 32;
    ns[26][2] = 25;
    ns[26][3] = 27;

    ns[27][0] = 21;
    ns[27][1] = 33;
    ns[27][2] = 26;
    ns[27][3] = 28;

    ns[28][0] = 22;
    ns[28][1] = 34;
    ns[28][2] = 27;
    ns[28][3] = 29;

    ns[29][0] = 23;
    ns[29][1] = 35;
    ns[29][2] = 28;
    ns[29][3] = 29;

    ns[30][0] = 24;
    ns[30][1] = 30;
    ns[30][2] = 30;
    ns[30][3] = 31;

    ns[31][0] = 25;
    ns[31][1] = 31;
    ns[31][2] = 30;
    ns[31][3] = 32;

    ns[32][0] = 26;
    ns[32][1] = 32;
    ns[32][2] = 31;
    ns[32][3] = 33;

    ns[33][0] = 27;
    ns[33][1] = 33;
    ns[33][2] = 32;
    ns[33][3] = 34;

    ns[34][0] = 28;
    ns[34][1] = 34;
    ns[34][2] = 34;
    ns[34][3] = 35;

    ns[35][0] = 29;
    ns[35][1] = 35;
    ns[35][2] = 34;
    ns[35][3] = 35;
}
