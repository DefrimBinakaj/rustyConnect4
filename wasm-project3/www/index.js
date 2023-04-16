import * as wasm from "wasm-project3";

const URL = "http://localhost:8081/";

// functions to navigate between pages
function goToHome() { window.location.href = 'index.html'; }
function goToConnect4() { window.location.href = 'connect4.html'; }
function goToTootOtto() { window.location.href = 'toototto.html'; }
function goToLeaderboards() { window.location.href = 'leaderboards.html'; }

// listen for navigation button press
document.getElementById("home_nav").addEventListener("click", goToHome);
document.getElementById("connect4_nav").addEventListener("click", event => {
    goToConnect4();
});
document.getElementById("toot-otto_nav").addEventListener("click", event => {
    goToTootOtto();
});
document.getElementById("leaderboards_nav").addEventListener("click", goToLeaderboards);

if (window.location.href == URL + "connect4.html") {
    var game_mode = "Connect4";
    document.getElementById("restart").addEventListener("click", event => {
        window.location.reload();
    });
    var num_cols = 7;
    connect4();
} else if (window.location.href == URL + "toototto.html") {
    var game_mode = "Toot-Otto";
    document.getElementById("restart").addEventListener("click", event => {
        window.location.reload();
    });
    var num_cols = 6;
    toot_otto();
} else if (window.location.href == URL + "index.html") {
    signed_in().then(function (signedIn) {
        if (signedIn) {
            document.getElementById("userDisplay").innerHTML = localStorage.getItem("signedInAs");
        }
    })
    // sign in
    document.getElementById("loginRequest").addEventListener("click", event => {
        var usernameInput = document.getElementById("usernameInput").value;
        var passwordInput = document.getElementById("passwordInput").value;
        localStorage.setItem("signedInAs", usernameInput);
        localStorage.setItem("signedInPassword", passwordInput);
        // refreshes the page
        location.reload();
    });

    // create account
    document.getElementById("createAccountRequest").addEventListener("click", event => {
        const createUsernameInput = document.getElementById("usernameInput").value;
        const createPasswordInput = document.getElementById("passwordInput").value;
        if (createUsernameInput != "" && createPasswordInput != "") {
            (async function () {
                try {
                    const responseCreate = await fetch('http://localhost:8080/signup', {
                        method: 'POST',
                        headers: {
                            'Content-Type': 'application/json'
                        },
                        body: JSON.stringify(
                            { username: createUsernameInput, password: createPasswordInput }
                        )
                    });
                }
                catch (error) {
                    console.log(error);
                }
            })();
    
            wasm.notify("Account created!");
            location.reload();
        } else {
            wasm.notify("Please enter a valid username and password");
            location.reload();
        }
    });

    document.getElementById("logoutRequest").addEventListener("click", event => {
        localStorage.setItem("signedInAs", null);
        localStorage.setItem("signedInPassword", null);
        // refreshes the page
        location.reload();
    });

} else if (window.location.href == URL + "leaderboards.html") {
    console.log("Leaderboards");
    // leaderboard
    (async function () {
        try {
            const response = await fetch('http://localhost:8080/data');
            const json_string = await response.text();
            const json = JSON.parse(json_string);
            // console.log(json);


            const printLeaderboard = new String(json);
            document.getElementById('leaderboardoutput').innerHTML = printLeaderboard.replace(/\n/g, '<br>');

        }
        catch (error) {
            console.log(error);
        }
    })();    
}

async function signed_in() {
    try {
        // wasm.notify('Trying to sign in...');
        const username = localStorage.getItem("signedInAs");
        const password = localStorage.getItem("signedInPassword");
        if (username.length == 0 || username == null) {
            return false;
        }

        const responseSign = await fetch('http://localhost:8080/signin', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(
                { username, password }
            )
        });
        const signResult = await responseSign.json();
        const printSignature = new String(signResult);

        if (printSignature == "true") {
            console.log(`Signed in as ${username}`);
            return true;
        } else {
            return false;
        }
    } catch (error) {
        wasm.notify(error);
        console.log(error);
    }
}

function connect4() {
    console.log("Connect4 game.");
    document.getElementById("to_home").addEventListener("click", goToHome);
    var alt_colors = false;
    var opponent;
    signed_in().then(function (signedIn) {
        if (signedIn) {
            var player_name = localStorage.getItem("signedInAs");
            document.getElementById("notSignedIn").style.display = 'none';
            document.getElementById("signedIn").style.display = 'block';
            document.getElementById("userDisplay").innerHTML = player_name;
            document.getElementById("player_name").innerHTML = `    ${player_name}`;
            
            // Get the opponent from the modal
            document.getElementById("2_players").addEventListener("click", event => {
                opponent = 0;
                console.log(`Opponent selected as: 2nd Player`);
                document.getElementById("opponent_name").innerHTML = "    Player 2";
                document.getElementById("menu").style.visibility = 'hidden';
            });
            document.getElementById("easybot").addEventListener("click", event => {
                opponent = 1;
                console.log(`Opponent selected as: Easy Bot`);
                document.getElementById("opponent_name").innerHTML = "    Easy Bot";
                document.getElementById("menu").style.visibility = 'hidden';
            });
            document.getElementById("medbot").addEventListener("click", event => {
                opponent = 2;
                console.log(`Opponent selected as: Medium Bot`);
                document.getElementById("opponent_name").innerHTML = "    Medium Bot";
                document.getElementById("menu").style.visibility = 'hidden';
            });
            document.getElementById("hardbot").addEventListener("click", event => {
                opponent = 3;
                console.log(`Opponent selected as: Hard Bot`);
                document.getElementById("opponent_name").innerHTML = "    Hard Bot";
                document.getElementById("menu").style.visibility = 'hidden';
            });
    
            document.getElementById("alt_colors").addEventListener("click", event => {
                alt_colors = true;
                document.getElementById("alt_colors").style.visibility = 'hidden';
                document.getElementById("player1_icon").style.color = "black";
                document.getElementById("player2_icon").className = "fa fa-circle-o";
                document.getElementById("player2_icon").style.color = "black";
                var pieces = document.getElementsByClassName("piece1");
                while (pieces.length) {
                    pieces[0].className = "piece1_alt";
                }
                pieces = document.getElementsByClassName("piece2");
                while (pieces.length) {
                    pieces[0].className = "piece2_alt";
                }
            })
    
            // set game mode, then create board
            wasm.set_game(game_mode);
            wasm.new_board();
            
            var player = 1; // next player = 2
            document.getElementById("player1box").style.background = "greenyellow";
    
            // listen for column to drop piece in
            document.getElementById("Col1").addEventListener("click", event => {
                insert_piece(1, player, alt_colors, opponent);
            });
            document.getElementById("Col2").addEventListener("click", event => {
                insert_piece(2, player, alt_colors, opponent);
            });
            document.getElementById("Col3").addEventListener("click", event => {
                insert_piece(3, player, alt_colors, opponent);
            });
            document.getElementById("Col4").addEventListener("click", event => {
                insert_piece(4, player, alt_colors, opponent);
            });
            document.getElementById("Col5").addEventListener("click", event => {
                insert_piece(5, player, alt_colors, opponent);
            });
            document.getElementById("Col6").addEventListener("click", event => {
                insert_piece(6, player, alt_colors, opponent);
            });
            document.getElementById("Col7").addEventListener("click", event => {
                insert_piece(7, player, alt_colors, opponent);
            });
    
            // insert a piece onto the board by "dropping" it in a column
            function insert_piece(col, player_id, alt_colors, opponent) {
                // returns the row that the piece was inserted in
                var row = wasm.insert_piece_C4(col, player_id); 
                if (row > 0) {  // insert was successful
                    // set piece in the space
                    var id = "R" + row.toString() + "C" + col.toString();
                    if (alt_colors) {
                        document.getElementById(id).innerHTML = `<span class="piece${player_id}_alt"></span>`; 
                    } else {
                        document.getElementById(id).innerHTML = `<span class="piece${player_id}"></span>`;
                    }
                    if (row == 1) {  // column is now full
                        document.getElementById(`Col${col}`).disabled = true;
                    }
                    console.log(`Player ${player_id} inserted a piece at ${id}.`);
                    // check for win
                    var win = wasm.check_for_win_C4(row, col, player_id);
                    if (win) {
                        winner(player_id);
                    } else {
                        switch_player(opponent);
                    }
                } else {
                    wasm.notify(`Column ${col} is full.`);
                }
            }
    
            function insert_piece_bot(row, col, player_id, opponent) {
                console.log(`Bot inserted piece at R${row}C${col}.`);
                if (row > 0) {
                    var id = "R" + row.toString() + "C" + col.toString();
                    if (alt_colors) {
                        document.getElementById(id).innerHTML = `<span class="piece${player_id}_alt"></span>`; 
                    } else {
                        document.getElementById(id).innerHTML = `<span class="piece${player_id}"></span>`;
                    }
                    if (row == 1) {
                        document.getElementById(`Col${col}`).disabled = true;
                    }
                    var win = wasm.check_for_win_C4(row, col, player_id);
                    switch_player(opponent);
                } else {
                    wasm.notify(`Column ${col} is full.`);
                }
            }
    
            // switch from player 1 to 2 or vice versa, and change colors of player boxes
            function switch_player(opponent, alt_colors) {
                if (player == 1) {
                    player = 2;
                    document.getElementById("player1box").style.background = "";
                    document.getElementById("player2box").style.background = "greenyellow";
                    if (opponent == 1) { // easy bot
                        let data = [];
                        data = wasm.easy_bot_C4(player);
                        let row = data[0];
                        let column = data[1];
                        insert_piece_bot(row ,column, player, opponent);
                    } else if (opponent == 2) {  // medium bot
                        let data = [];
                        data = wasm.medium_C4(player);
                        let row = data[0];
                        let column = data[1];
                        insert_piece_bot(row, column, player, opponent);
                    } else if (opponent == 3) {  // hard bot
                        let data = [];
                        data = wasm.difficult_C4(player);
                        let row = data[0];
                        let column = data[1];
                        insert_piece_bot(row, column, player, opponent, alt_colors);
                    }
                } else if (player == 2) {
                    player = 1;
                    document.getElementById("player1box").style.background = "greenyellow";
                    document.getElementById("player2box").style.background = "";
                } else {
                    wasm.notify("There was an error switching players!");
                }
            }
        }
    }) 
}

function toot_otto() {
    console.log("Toot-Otto Game.")
    document.getElementById("to_home").addEventListener("click", goToHome);
    signed_in().then(function (signedIn) {
        if (signedIn) {
            var opponent;
            var player_name = localStorage.getItem("signedInAs");
            document.getElementById("notSignedIn").style.display = 'none';
            document.getElementById("signedIn").style.display = 'block';
            document.getElementById("userDisplay").innerHTML = player_name;
            document.getElementById("player_name").innerHTML = `    ${player_name}`;

            var player = 1; // next player = 2
            var player1_phrase = "TOOT";
            var player2_phrase = "OTTO";

            var opponent_name = "Player";
            // Get the opponent from the modal
            document.getElementById("2_players").addEventListener("click", event => {
                opponent = 0;
                opponent_name = "Player 2";
                console.log(`Opponent selected as: ${opponent_name}`);
                document.getElementById("menu").style.visibility = 'hidden';
                document.getElementById("player2box").innerHTML = `<h3 class="center" style="margin-left: 10px">${opponent_name}: ${player2_phrase}</h3>`;
            });
            document.getElementById("easybot").addEventListener("click", event => {
                opponent = 1;
                opponent_name = "Easy Bot";
                console.log(`Opponent selected as: ${opponent_name}`);
                document.getElementById("menu").style.visibility = 'hidden';
                document.getElementById("player2box").innerHTML = `<h3 class="center" style="margin-left: 10px">${opponent_name}: ${player2_phrase}</h3>`;
            });
            document.getElementById("medbot").addEventListener("click", event => {
                opponent = 2;
                opponent_name = "Medium Bot";
                console.log(`Opponent selected as: ${opponent_name}`);
                document.getElementById("menu").style.visibility = 'hidden';
                document.getElementById("player2box").innerHTML = `<h3 class="center" style="margin-left: 10px">${opponent_name}: ${player2_phrase}</h3>`;
            });
            document.getElementById("hardbot").addEventListener("click", event => {
                opponent = 3;
                opponent_name = "Hard Bot";
                console.log(`Opponent selected as: ${opponent_name}`);
                document.getElementById("menu").style.visibility = 'hidden';
                document.getElementById("player2box").innerHTML = `<h3 class="center" style="margin-left: 10px">${opponent_name}: ${player2_phrase}</h3>`;
            });

            // set game mode, then create board
            wasm.set_game(game_mode);
            wasm.new_board();

            // set phrases in player boxes
            document.getElementById("player1box").innerHTML = `<h3 class="center" style="margin-left: 10px">${player_name}: ${player1_phrase}</h3>`;
            document.getElementById("player2box").innerHTML = `<h3 class="center" style="margin-left: 10px">${opponent_name}: ${player2_phrase}</h3>`;
            
            // set color of current player box
            document.getElementById("player1box").style.background = "greenyellow";

            // set first letter and letter selection button colors
            var letter = 'T';
            document.getElementById("T_button").style.background = "greenyellow";
            document.getElementById("O_button").style.background = "white";

            // allow the player to switch phrases to play as TOOT or OTTO
            document.getElementById("switch_phrases").addEventListener("click", event => {
                var temp = player1_phrase;
                player1_phrase = player2_phrase;
                player2_phrase = temp;
                document.getElementById("player1box").innerHTML = `<h3 class="center" style="margin-left: 10px">${player_name}: ${player1_phrase}</h3>`;
                document.getElementById("player2box").innerHTML = `<h3 class="center" style="margin-left: 10px">${opponent_name}: ${player2_phrase}</h3>`;
            });

            // switch to the selected letter
            document.getElementById("T_button").addEventListener("click", event => {
                set_letter('T');
            });
            document.getElementById("O_button").addEventListener("click", event => {
                set_letter('O');
            });

            // listen for column to drop piece in
            document.getElementById("Col1").addEventListener("click", event => {
                insert_piece(1, letter, opponent);
            });
            document.getElementById("Col2").addEventListener("click", event => {
                insert_piece(2, letter, opponent);
            });
            document.getElementById("Col3").addEventListener("click", event => {
                insert_piece(3, letter, opponent);
            });
            document.getElementById("Col4").addEventListener("click", event => {
                insert_piece(4, letter, opponent);
            });
            document.getElementById("Col5").addEventListener("click", event => {
                insert_piece(5, letter, opponent);
            });
            document.getElementById("Col6").addEventListener("click", event => {
                insert_piece(6, letter, opponent);
            });

            // insert a piece onto the board by "dropping" it in a column
            function insert_piece(col, letter, opponent) {
                // disable switching phrases after first piece is played
                document.getElementById("switch_phrases").disabled = true;

                // returns the row that the piece was inserted in
                var row = wasm.insert_piece_TO(col, letter); 
                if (row > 0) {  // insert was successful
                    // set piece in the space
                    var id = "R" + row.toString() + "C" + col.toString();
                    document.getElementById(id).innerHTML = `<span class="piece${letter}">${letter}</span>`;
                    if (row == 1) {  // the column is now full
                        document.getElementById(`Col${col}`).disabled = true;
                    }
                    console.log(`Player ${player} inserted a '${letter}' piece at ${id}.`);
                    // check for win
                    var win = wasm.check_for_win_TO(row, col);
                    if (win == 1) { // TOOT player has won
                        if (player1_phrase == "TOOT") {winner(1);} else {winner(2);}
                    } else if (win == 2) {  // OTTO player has won
                        if (player1_phrase == "OTTO") {winner(1);} else {winner(2);}
                    } else if (win == 3) {  // tie
                        winner(3);
                    } else {
                        switch_player(opponent);
                    }
                } else {
                    wasm.notify(`Column ${col} is full.`);
                }
            }

            function insert_piece_bot(row, col, letter, player_id, opponent) {
                console.log(`Bot inserted piece at R${row}C${col}.`);
                if (row > 0) {
                    var id = "R" + row.toString() + "C" + col.toString();
                    document.getElementById(id).innerHTML = `<span class="piece${letter}">${letter}</span>`;
                    if (row == 1) {
                        document.getElementById(`Col${col}`).disabled = true;
                    }
                    var win = wasm.check_for_win_TO(row, col, player_id);
                    if (win == 1) { // TOOT player has won
                        if (player1_phrase == "TOOT") {winner(1);} else {winner(2);}
                    } else if (win == 2) {  // OTTO player has won
                        if (player1_phrase == "OTTO") {winner(1);} else {winner(2);}
                    } else if (win == 3) {  // tie
                        winner(3);
                    } else {
                        switch_player(opponent);
                    }
                } else {
                    wasm.notify(`Column ${col} is full.`);
                }
            }

            // switch from player 1 to 2 or vice versa, and change colors of player boxes
            function switch_player(opponent) {
                if (player == 1) {
                    player = 2;
                    document.getElementById("player1box").style.background = "";
                    document.getElementById("player2box").style.background = "greenyellow";
                    if (opponent == 1) { // easy bot
                        let data = [];
                        data = wasm.easy_otto();
                        let row = data[0];
                        let column = data[1];
                        let letter = data[2];
                        insert_piece_bot(row, column, letter, player, opponent);
                    } else if (opponent == 2) {  // medium bot
                        let data = [];
                        data = wasm.medium_TO(player);
                        let row = data[0];
                        let column = data[1];
                        let letter = data[2];
                        insert_piece_bot(row, column, letter, player, opponent);
                    } else if (opponent == 3) {  // hard bot
                        let data = [];
                        data = wasm.difficult_TO(player);
                        let row = data[0];
                        let column = data[1];
                        let letter = data[2];
                        insert_piece_bot(row, column, letter, player, opponent);
                    }
                } else if (player == 2) {
                    player = 1;
                    document.getElementById("player1box").style.background = "greenyellow";
                    document.getElementById("player2box").style.background = "";
                } else {
                    wasm.notify("There was an error switching players!");
                }
            }

            // set the letter of the piece to the clicked letter button and change colors
            function set_letter(new_letter) {
                document.getElementById(`${letter}_button`).style.background = "white";
                document.getElementById(`${new_letter}_button`).style.background = "greenyellow";
                letter = new_letter;
            }
        }
    })
}

// display the winning message, change player box colors, and disable all column buttons
function winner(player_id) {
    if (player_id == 0) {
        wasm.notify(`No winner...`);
        document.getElementById(`player1box`).style.background = "white";
        document.getElementById(`player2box`).style.background = "white";
    } else if (player_id == 3) {
        wasm.notify(`There was a tie!`);
        document.getElementById(`player1box`).style.background = "gold";
        document.getElementById(`player2box`).style.background = "gold";
    } else {
        if (player_id == 1) {
            // increase db wins
            const increaseUser = localStorage.getItem("signedInAs");
            (async function () {
                try {
                    console.log(increaseUser, " won");
                    const response = await fetch('http://localhost:8080/winc4', {
                        method: 'POST',
                        headers: {
                            'Content-Type': 'application/json'
                        },
                        body: JSON.stringify(
                            increaseUser
                        )
                    });
                    const incrRES = await response.json();
                }
                catch (error) {
                    console.log(error);
                }
            })();
        }
        else if (player_id == 2) {
            // increase db plays
            const increaseUser = localStorage.getItem("signedInAs");
            (async function () {
                try {
                    console.log(increaseUser, " lost");
                    const response = await fetch('http://localhost:8080/losec4', {
                        method: 'POST',
                        headers: {
                            'Content-Type': 'application/json'
                        },
                        body: JSON.stringify(
                            increaseUser
                        )
                    });
                    const incrRES = await response.json();
                }
                catch (error) {
                    console.log(error);
                }
            })();
        }
        wasm.notify(`Player ${player_id} has won!`);
        document.getElementById(`player${player_id}box`).style.background = "gold";
        document.getElementById(`player${3 - player_id}box`).style.background = "white";
    }
    for (var i = 1; i <= num_cols; i++) {
        document.getElementById(`Col${i}`).disabled = true;
    }

}






