fn main() {
    let actions = 4;
    let states = 6;

    let mut q = vec![vec![0.0; actions]; states];
    let r = vec![vec![-1.0; actions]; states];

    for state in 0..q.len() {
        for action in 0..q[0].len() {
            print!("{} ", q[state][action]);
        }
        println!();
    }

    for state in 0..r.len() {
        for action in 0..r[0].len() {
            print!("{} ", r[state][action]);
        }
        println!();
    }

    let alpha = 0.6;
    let gamma = 0.1;

    let state = 0;
    let action = 0;

    q[state][action] = q[state][action] * alpha * (r[state][action] + gamma * q[state+1][action] - q[state][action]);
}
