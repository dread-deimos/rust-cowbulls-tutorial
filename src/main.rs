/*
    Before we start, I'll tell about the game that is being implenented here. I believe it has many
    names accross the globe, but to me it was known as "Cows and Bulls" since my school days when
    we've played the game between the classes with my friends.

    This game is for two players. One player has a passive role. He makes up a secret four-digit
    number (digits must be unique and the number can start with zero) and writes it down somewhere
    safe from gazes of the player two.

    Then player two makes guesses on what this number could be and player one looks up the number
    and counts the "cows" and "bulls", where "cows" is number of digits that player two guessed
    right, but they are not on right positions in the original secret number. The "bulls" is number
    of digits that are at their right place in the secret number. Player one doesn't tell which
    digits are "cows" and "bulls".

    For example, the secret number is 6437 and player two calls his guess:
    P2> 1234.
    P1> You've got one bull and one cow.
    P2> 1290.
    P1> There's nobody here.
    P2> 7364.
    P1> It's 4 cows, wow!
    P2> 6437.
    P1> Bingo! You've guessed it!

    The challenge of the game for player two is guess the secret number in the smallest amount of
    attempts. The pencil and a piece of paper usually helps to store collected data and analyze it
    to lower attempts count to minimum.

    In this game, which we program in Rust (v1.1.0) below, we let the computer be player one and
    the user to play as a player two.

    We'll also implement a simple tool to assist player with the collected data, which will help us
    to undestand a bit more about a game logic.
*/

// Import STDIN and STDOUT handle generators for input and output respectively.
// We need the std::io::Write trait so we can flush the output buffer later.
use std::io::{stdin, stdout};
use std::io::Write;

// Declare our game module and import it's definitions in this module's scope
pub mod game;
use game::*;

fn main() {
    // Create a game object. Note that it's mutable so we can change it's internal variables. It's
    // also worth to mention that mutability is spread on all struct variables and there's no way
    // to make some of them mutable and others not.
    let mut the_game = Game::new();

    // Just print an invitation line using a println! macros
    println!("Guess the number! (Enter 'q' to quit', 'h' for help)");

	loop {
        // We'll need an empty String object to feed it to IO object as buffer
		let mut input = String::new();

        // Print prompt decoration. We want an imput invitation that displays how many tries have
        // been used so far to guess the secret number
        print!("{} > ", the_game.tries);

        // Flush buffer so the decoration will be printed immedeately without line buffering.
        // Note, that this will fail if you don't include std::io::Write trait.
        let _ = stdout().flush();

        // Read from STDIN to our buffer variable. read_line() method returns a Result object so
        // we need to handle it's success and fail status.
    	stdin().read_line(&mut input)
			.ok()
			.expect("Failed to read input");

        // Trim string. Note that trim() method will return &str type, not String. But we don't
        // need the String type any further because we don't want this value to be borrowed.
        // We use "let" to redefine this binding with new type.
		let input = input.trim();

        // Now we attempt to parse the input string to a u32 integer. On success, we have a number
        // and will check if matches our secret number. On fail, we assume that user have entered
        // a command and we will try to figure out which one exactly.
        match input.parse::<u32>() {

            // Our first case. Since we must have four digits according to our game rules, we'll
            // check if input string's length is four. If not, we just print the warning and
            // proceed with the game loop.
            Ok(_) if input.len() != 4 => println!("Number of four digits is needed"),

            // Next, according to our game's rules, all digits of the numbers should be different.
            // So we call a game method that will check this for us. check_unique_digits() returns
            // a bool typed value.
            Ok(_) if !the_game.check_unique_digits(input) => println!("Digits must be unique"),

            // If previous checks have filtered us a valid number, we'll ask our game object to
            // check this number against the secret one. If it matches, we will get the true bool
            // value, print the win message with a number of tries needed to guess the right answer
            // and break the loop so game will end.
            Ok(_) if the_game.guess(input) => {
    			println!("You won in {} tries!", the_game.tries + 1);
                break;
    		},

            // If the input number was correct, but didn't match the secret value, we'll analyze it
            // and print the number of "cows" (digit exists, but doesn't match position) and
            // "bulls" (digit exists in secret number and matches position).
            Ok(_) => {
                // Here we ask the game object for tuple containing cows and bulls. We use
                // automatic type allocation and our code expects simple integers, so we don't
                // really need to think about particular type of these bindings at the moment.
                let (cows, bulls) = the_game.try(input);

                // We ask the game object to analyze acquired result and save hints on digit
                // positions that are being entered
                the_game.analyze(input, cows, bulls);

                // Now we print number of found matches if there was any
                if cows == 0 && bulls == 0 {
                    println!("Nothing found");
                } else {
                    println!("Found {} cows and {} bulls", cows, bulls);
                }
            }

            // So we failed to parse our input string as integer, so it must be a command. We start
            // another match operator.
            Err(_) => match input {

                // If we encounter an empty value, we just restart the game loop waiting for
                // non-empty command.
                "" => continue,

                // Here we define a pattern that matches one of these strings for a command that
                // quits the game. The break operator will end the game loop and since there's
                // nothing after this loop, the application will close returning user to shell.
    			"q" | "quit" | "exit" => break,

                // We'll want to show user a list of available commands, so we call the
                // print_help() function which will handle this for us.
                "h" | "help" | "?" => print_help(),

                // This command calls print_hint(). Read about it below.
                "s" | "stats" => print_hint(the_game.hint_table),

                // This simple command recursively calls the main() function effecrively restarting
                // the game. Make sure to break the loop, so we don't restart game loop after we
                // have returned from this call.

                // Note, that this kind of implementation is not really good one, but it'll be fine
                // for this tutorial. It's drawback is that with every game restart the local stack
                // increases and allocates a new game object that's data is being piled up in the
                // heap.
                "r" | "restart" => {
                    main();
                    break;
                },

                // In case of any other input, we just print a message that we didn't get what user
                // wanted and hint him to use help command.
    			_  => println!("Unknown command: \"{}\". Enter 'h' for help", input),
            },
        };
	}
}

// This function just prints out the list of available game commands
fn print_help() {
    println!("r, restart    - Restart game");
    println!("q, quit, exit - Quit game");
    println!("h, help, ?    - This text");
    println!("s, stats      - Check out some hints on potential digit positions");
    println!("<NNNN>        - Enter four unique digits to guess the number and win");
}

// This functions take a two-dimmensional array of special typed values (see more in the game
// module description)
fn print_hint(table: [[Hint; 4]; 10]) {

    // Print position numbers
    println!("   1 2 3 4");

    // Loop through 10 digits from 0 to 9 inclusively
    for i in 0..10 {

        // Print the digit
        print!("{}: ", i);

        // Then loop through four available positions
        for j in 0..4 {

            // For each value of enum type Hint (see definition in game module) we print respective
            // character
            print!("{} ", match table[i][j] {
                Hint::Unknown => " ",
                Hint::Maybe   => "?",
                Hint::Here    => "+",
                Hint::NotHere => "-",
            });
        }

        // Print the new line character to finish line for this digit
        print!("\n");
    }
}
