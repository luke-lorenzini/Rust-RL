use rand::Rng;

pub struct Xxx {
    alpha: f64,
    gamma: f64,
    epsilon: f64,
}

impl Xxx {
    pub fn new(alpha: f64, gamma: f64, epsilon: f64) -> Xxx {
        Xxx {
            alpha,
            gamma,
            epsilon,
        }
    }

    pub fn get_action(&self, state: usize, q: &Vec<Vec<f64>>) -> usize {
        let mut rng = rand::thread_rng();
        let n2: f64 = rng.gen();

        if n2 < self.epsilon {
            // action = env.action_space.sample() # Explore action space
            let actions = q[0].len();
            let mut rng = rand::thread_rng();
            rng.gen_range(0..actions)
        } else {
            // action = np.argmax(q_table[state]) # Exploit learned values
            self.get_highest_q_index(state, q)
        }
    }

    pub fn get_next_state(
        &self,
        state: usize,
        q: &mut Vec<Vec<f64>>,
        r: &Vec<Vec<i32>>,
        ns: &Vec<Vec<usize>>,
    ) -> usize {
        // let gamma = 0.6;

        let action = self.get_action(state, q);

        // Step 3: Based on action, determine next state
        let next_state = ns[state][action];

        // Step 4: Choose highest action reward from next state
        let next_action = self.get_highest_reward(next_state, r);

        // Step 5: Update q
        q[state][action] = q[state][action]
            + self.alpha
                * (r[state][action] as f64 + self.gamma * q[next_state][next_action]
                    - q[state][action]);

        next_state
    }

    fn get_highest_reward(&self, state: usize, r: &Vec<Vec<i32>>) -> usize {
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

    fn get_highest_q_index(&self, state: usize, q: &Vec<Vec<f64>>) -> usize {
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
}
