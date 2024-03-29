use q_learn::Xxx;
use rand::Rng;

fn main() {
    // Actions are up, down, left, right
    let actions = 4;

    // Representation of a 6x6 matrix
    let states = 36;
    let termination_state = states - 1;

    // Matrices are 36x4. States enumerated as rows
    // and columns are actions (up, down, left, right)
    let mut q = vec![vec![0.0; actions]; states];
    let mut r = vec![vec![-1; actions]; states];
    let mut ns = vec![vec![0; actions]; states];

    init(&mut q, &mut r, &mut ns);

    let epochs = 1_000;

    let mut rng = rand::thread_rng();
    let mut state = rng.gen_range(0..states);
    let rl = Xxx::new(0.1, 0.6, 0.1);

    for _ in 0..epochs - 1 {
        // Make sure we don't start in the termination state
        while state == termination_state {
            state = rng.gen_range(0..states);
        }

        state = rl.get_next_state(state, &mut q, &r, &ns);
        while state != termination_state {
            state = rl.get_next_state(state, &mut q, &r, &ns);
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

    let direction = match max_col {
        0 => "Up",
        1 => "Down",
        2 => "Left",
        _ => "Right",
    };

    // println!("Max Value {}, Next State {}", max_value, next_state);
    println!("Move {} to state {}", direction, next_state);

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

/**
 * initialize the matrices
 *
 * q is the q matrix
 * r is the reward / penalty matrix
 * ns is the next-state matrix
 */
fn init(q: &mut Vec<Vec<f64>>, r: &mut Vec<Vec<i32>>, ns: &mut Vec<Vec<usize>>) {
    const WALL: i32 = -10; // penalty for hitting a wall
    const REWARD: i32 = 10; // reward for termnation state
                            // const TRAP: i32 = -5;

    let mut rng = rand::thread_rng();
    // let n2: f64 = rng.gen_range(0.0..0.001);

    for row in 0..q.len() {
        for col in 0..q[row].len() {
            q[row][col] = rng.gen_range(0.0..0.001);
        }
    }

    // R is reward matrix. Initialized to -1
    // update any execptional positions
    r[0][0] = WALL; // move up from 0,0 is penalty
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

    r[29][1] = REWARD; // move down to reward state
    r[34][3] = REWARD; // move right to reward state

    // up, down left, right
    ns[0][0] = 0; // move up from 0,0 to 0 (can't move up from 0,0)
    ns[0][1] = 6; // move down from 0,0 to 6
    ns[0][2] = 0; // move left from 0,0 to 0 (can't move left from 0,0)
    ns[0][3] = 1; // move right from 0,0 to 1

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
