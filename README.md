<div align="center">

  <h1>Rusty Connect-4 / TOOTOTTO</h1>

  <strong>[IN PROGRESS]</strong>
  
  <strong>A simple game created by Defrim with Rust and WebAssembly.</strong>

  <sub>Built with 🦀🕸 by <a href="https://rustwasm.github.io/">The Rust and WebAssembly Working Group</a></sub>
</div>

## About

Connect-4 and TOOTOTTO games which include:
- base games with seamless mode-switching
- Computerized opponent(s) for the players with multiple difficulty levels.
- A connection to an underlying database that would hold the player information
and game results using MongoDB
- A GUI that can handle the color vision deficiency (a.k.a. Color blindness) in users.

Game Rules:
While in the web game, the two games may only be played if you are logged into an existing account in the database, which can be executed using the login and create account buttons. The leaderboards may be viewed without logging in, and they are ordered by amount of wins first and then winrate. 

Based off of:
- [Connect-4][connect4]
- [TOOTOTTO][TOOTOTTO]

[connect4]: https://en.wikipedia.org/wiki/Connect_Four
[TOOTOTTO]: https://nyc.cs.berkeley.edu/wiki/Toot_and_Otto


## Instructions

### 1. Install `wasm-pack`

[Install wasm here][installwasm]

[installwasm]: https://rustwasm.github.io/wasm-pack/installer/

### 2. MongoDB setup

- Install mongoDB and connect to “localhost:27017”
- Create a database named ‘tempFromCompass’, with a table ‘players’


### 3. Build and run the server

- Navigate to the server project directory
```
cargo build
```
```
cargo run
```
-> The terminal should display “server running…”


### 4. Build wasm

- In a separate IDE window, navigate to wasm-project3 (the wasm project file)
```
cargo build
```
```
wasm-pack build
```

- If there isn't a www directory, create one:
```
npm init wasm-pack www
```
Then navigate to this directory and execute:
```
npm install
```
Otherwise, continue to step 5!

### 5. Build and run the web pack

- Open a CMD terminal (NOT a powershell terminal)
- Navigate to the www directory

WINDOWS:
```
set NODE_OPTIONS= --openssl-legacy-provider && npm run start
```
MACOS / UNIX:
```
npm run start
```
- Click on the generated link in the output -> should be <a href="http://localhost:8081/">this link</a>

NOTE:
- the server MUST be run before the game code and the MongoDB connection must be established


## Known Code Limitations

- There is a bug in the game pages that does not allow you to navigate to Home
using the Home navigation button mid-game
- There can only be one user logged in at a time, which means that the second player’s wins/losses in 2 player mode are not tracked.
- There is a potential for the boards to become full of pieces, at which time all the columns to drop pieces in will become disabled and the game will need to manually reset with the restart button or by refreshing the page. The bots will also be stuck in an infinite loop at this point trying to enter a piece.


