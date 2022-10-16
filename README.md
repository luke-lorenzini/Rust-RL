# Rust-RL
Reinforcement Q-Learning with Rust

Simple example to demonstrate Q-Learning written in Rust.

Mostly an implementation from
https://www.learndatasci.com/tutorials/reinforcement-q-learning-scratch-python-openai-gym/


```
cargo build
cargo run
```

Successful output should look something like:

```
Starting in state 2
Move Right to state 3
Move Right to state 4
Move Right to state 5
Move Down to state 11
Move Down to state 17
Move Down to state 23
Move Down to state 29
Move Down to state 35
```

This impementation can get stuck in the get_path() function, which looks like:

```
Move Left to state 9
Move Right to state 10
...
```
in the terminal.
