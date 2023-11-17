# Game of life

This is implementation of game of life in rust.
It uses TUI library crossterm for printing directly in terminal. It sometimes glitter, so it's not recommended to set too high dimensions.
The default ones are big enough to generate a simple game.
In general, they should not extend over number of lines in terminal, otherwise the generation will be cut.

I am aware of some of the bad architectural choices and boundary issues in the code.  
I'm planning to fix some of them in the future, but if it bothers you, please feel free to send me suggestions.  
This project was initiated as for me to learn basics of Rust and image generation in terminal, but first and foremost I wanted to implement the "game of life", 
I'd like to build intuitions about what life is really and that seems to be one place everyone usually starts with, when thinking about life in the context of computation.  
How we define life is and will be one of the most important topics in the near decades as we gained technological advancements in contraception, AI development and abandonment of religion.
Though, we freed ourselves from god, some of the answers that it offered to us need to be revisited, for the questions are confusing and human being cannot stand a life of confusion. When faced with confusion one first desperately tries to explain it, then he denies the confusion, ultimately justifies the belief with whatever authority signals virtuous action. If we don't seek answers in this area, then the whole project that lead to abandonment of religious dogma fails, for what once fights against dogma, at some point becomes a dogma. The subjects change, the patterns stay the same. History repeats itself.  
If there's one thing humanity can do to transcend this cycle, it is to break with reactive thinking and challenge self - the basic tools of cognition. The same perception that allows one to see flaws in the system, are same that lead to the flaws. It is a difficult task to abandon them, but it's necessary in attempt of humanity to elevate itself into the next stage of intellectual development.  
To challenge those - we must understand what we are and who we are. Hence - to understand life (body) and to understand the phenomena of personhood (spirit).

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
    ./target/release/game-of-life "examples/glider.json"
    ```


