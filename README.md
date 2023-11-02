# Game of life

This is implementation of game of life in rust.
It prints the window in a terminal line-by-line, hence it's not recommended to use too big dimensions.

# Running

1. Configure parameters (see constants at the top of *main.rs*) - display size, delay between next generations, max number of generations:

    ```
    const WIDTH: usize = 80;
    const HEIGHT: usize = 60;
    const TIME_DELAY_MILISECONDS: u64 = 50;
    const NUM_OPERATIONS: usize = 1000;
    ```

2. Build:

    ```
    cargo build -r
    ```

3. Run:

    ```
    ./target/release/game-of-life <path to pattern>
    ```

    i.e.:
    ```
    ./target/release/game-of-life "examples/pattern5.json"
    ```


