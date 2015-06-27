// We'll use external implementation of random number generator. Note that we use self::* scope
// to actually use something from this library because we are inside the game module and use
// expects absolute path.
extern crate rand;
use self::rand::Rng;

// Here we define a enum type that we'll use to print a table of hints for digits and their
// potential positions in the secret number. Here we use a derive pragma that tells the Rust to
// throw a couple of traits to our new type. Eq and PartialEq are used for comparison between
// different variables of this type. Copy and Clone traits are needed to actually create a copy of
// a value. Note that we use "pub" operator before enum definition, so the main program will be
// able to see this type after it will import it into it's scope.
#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Hint {
	// We don't really know anything about a digit in hinted position
	Unknown,
	// The digit maybe is here
	Maybe,
	// The digit is definitely present at this position
	Here,
	// The digit is definitely not here
	NotHere,
}

// Here is our game structure that contains data that we'll need to play. This definition is public
// so it can be called from the outside of this module.
pub struct Game {

	// This is a private definition of a fixed-length array of four elements that contain unsigned
	// integers of 8 bit length. This variable will not be seen from outside this module.
	secret_number: [u8; 4],

	// We'll keep the count of guess attempts in this public unsigned 32 bit integer
	pub tries: u32,

	// This is a two-dimmensional array (10 lines, 4 columns) of Hint enum values. We'll store our
	// estimations on potential digit positions here.
	pub hint_table: [[Hint; 4]; 10],
}

// This is an implementation of our Game type. It stores methods and associated functions of our
// game object.
impl Game {

	// This is a constructor. The "new" name is not special, but it is common to call standard
	// object constructors like this. We need to specify that it is public to access it from
	// outside the module. This is not an object method, because it doesn't have a reference to
	// special variable called "self" in it's arguments list.
	pub fn new() -> Game {
		// So we construct an actual object
		Game {
			// We'll use our own function that randomizes the secret number. See below for details.
			secret_number: Game::generate_secret(),

			// We start with zero guess attempts at the beginning of the game
			tries: 0,

			// Here we initialize our hint table. This syntax makes sure that whole 10x4 table of
			// Hint typed values being filled with Hint::Unknown values.
			hint_table: [[Hint::Unknown; 4]; 10],
		}
	}

	// This is a public method (see the &self argument?), which purpose is to check whether a
	// supplemented string contains a secret number. It returns bool value as a result.
	pub fn guess(&self, variant: &str) -> bool {

		// First, we want to parse the string to our inner representation of number, which is an
		// array of four u8s
		let input = Game::from_string(variant);

		// Loop through both secret number array and input array
		for i in 0..4 {
			// If some position is different between the two, we immediately return with a false
			if self.secret_number[i] != input[i] {
				return false
			}
		}

		// If we've looped through the whole array and haven't got a false, then we're safe to
		// assume that the input totally matches our secret number. Yay, we've won! Also, note that
		// we don't use return operator. This is because whole our function body is an expression
		// and last line of an expression becomes it's final result. Just make sure not to put a
		// semicolon at the end of this line so Rust will know that it should be returned.
		true
	}

	// We'll use this method to look up digit positions of our number and tell our player about
	// found cows and bulls
	pub fn try(&mut self, variant: &str) -> (u8, u8) {

		// First, we want to parse the string to our inner representation of number, which is an
		// array of four u8s
		let input = Game::from_string(variant);

		// Define mutable integers to count the cows and bulls
		let mut cows = 0;
		let mut bulls = 0;

		// Loop through four digits
		for i in 0..4 {

			// For every two digits matched between the input and secret arrays, we add a bull
			if self.secret_number[i] == input[i] {
				bulls += 1;
			}

			// Crossloop through two arrays to find existing digits that don't match positions to
			// count cows
			for j in 0..4 {
				if i != j && self.secret_number[i] == input[j] {
					cows += 1;
				}
			}
		}

		// Register that we tried another guess
		self.tries += 1;

		// Return a tuple of cows and bulls
		(cows, bulls)
	}

	// This method uses simple heuristics to add digit position hints to our respective table. Note
	// that it doesn't use the secret number and all assumptions that can be made inside it can be
	// made by player using logic and a piece of paper (or a good memory).
	pub fn analyze(&mut self, variant: &str, cows: u8, bulls: u8) {

		// Again, parse string to array
		let input = Game::from_string(variant);

		// First case is most useful. When there are no cows or bulls, we can be sure that the
		// secret number does not contain any digit from our guess.
		if cows == 0 && bulls == 0 {

			// For every for input digits...
			for v in &input {

				// ..we loop through four available positions in secret number...
				for j in 0..4 {

					// ...and set a hint Hint::NotHere
					self.hint_table[*v as usize][j] = Hint::NotHere;

					// Note the funny "*v as usize" construct. Since every element of an input
					// array is represented by a reference, we should dereference it to simple u8
					// and only then cast it as special usize type that is used for array indices.
					// And since j is just a generic int (and Rust is not sure about it's exact
					// type at this moment), we can ommit type casting to usize, because Rust is
					// smart enough to do that for us.
				}
			}
		}

		// Another useful case is when a sum of cows and bulls is four. That means, that every
		// digit of a secret number is represented in the guess.
		if cows + bulls == 4 {
			// So we loop through all 10 possible digits from 0 to 9
			for i in 0..10 {

				// We check if this particular digit is mentioned in the guess
				let mut is_present = false;
				for v in &input {

					// Remember, when we loop through the array, we get references to cells, not
					// their value. So we must dereference it to compare with common integer.
					// At this point, undefined int i is being compare with a definite u8, so Rust
					// thinks "Aha! This i one must be also u8!" and from this point treats it like
					// a u8.
					if i == *v {
						is_present = true;
					}
				}

				// So, this digit i is not present in the input number
				if !is_present {
					// We run through every possible position for this digit and mark it as not
					// possible
					for j in 0..4 {
						// Since i was compared to v earilier, we must type cast it to usize so
						// it can be used as an array index
						self.hint_table[i as usize][j] = Hint::NotHere;
					}
				}
			}
		}

		// But what can we think of when there are some bulls in the guess? We can suspect every
		// digit of the guess to be at it's position.
		if bulls > 0 {
			for i in 0..4 {
				// Note that when we initialize the new binding from an array cell, we don't need
				// to dereference it as it was when we looped through input using for .. in
				let v = input[i] as usize;

				// For every previously unknown position we can assume that maybe (just maybe!)
				// this digit could be here
				if self.hint_table[v][i] == Hint::Unknown {
					self.hint_table[v][i] = Hint::Maybe;
				}
			}
		}

		// Another useful case is when every match we have is a bull. We can use previously known
		// hints to calculate some positions of a guess.
		if cows == 0 && bulls > 0 {
			// Loop through input digits and count how many of them are definitely not on their
			// positions for this guess
			let mut c = 0;
			for i in 0..4 {
				let v = input[i] as usize;
				if self.hint_table[v][i] == Hint::NotHere {
					c += 1;
				}
			}

			// If the sum of found bulls plus the sum of "definitely not here" digits is four, we
			// can assume, that every previously unknown positioned digits are at their right
			// positions now
			if c + bulls == 4 {
				for i in 0..4 {
					let v = input[i] as usize;
					if self.hint_table[v][i] == Hint::Unknown {
						self.hint_table[v][i] == Hint::Here;
					}
				}
			}
		}

		// And the last case. When we have exclusively cows. That means that none of mentioned
		// digits are at their positions this time.
		else if cows > 0 && bulls == 0 {
			for i in 0..4 {
				let v = input[i] as usize;

				// For every position that was unclear previously, we mark it as definitely "no"
				if self.hint_table[v][i] == Hint::Maybe || self.hint_table[v][i] == Hint::Unknown {
					self.hint_table[v][i] = Hint::NotHere;
				}
			}
		}
	}

	// This method is used to check whether a proposed number consists of unique digits or has
	// duplicates
	pub fn check_unique_digits(&self, variant: &str) -> bool {

		// As usual, parse the string to array
		let input = Game::from_string(variant);

		// Crossloop through this array with itself to find duplicates
		for i in 0..4 {
			for j in (i + 1)..4 {
				if input[i] == input[j] {
					return false;
				}
			}
		}

		true
	}

	// Private function to generate random sequence of four unique decimal digits
	fn generate_secret() -> [u8; 4] {
		// Init the random number generator
		let mut rng = rand::thread_rng();

		// Create an array of decimal digits
		let mut array = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

		// Randomly shuffle the array using the generator
		rng.shuffle(&mut array);

		// Return first four digits of resulting array
		[ array[0], array[1], array[2], array[3] ]
	}

	// Private function that parses the string to an array of digits
	fn from_string(value: &str) -> [u8; 4] {
		// Create a mutable array and populate it with four zeroes
		let mut array = [0u8; 4];

		// Use chars() method to create an iterator over every character of the string. Then for
		// every iterated char value covert them to string and parse the string to u8 type. The
		// result of String::parse() method has a Result type, so we must unwrap it to get an
		// actual value. We have to assume that this Result is always successfull for this.
		// Otherwise, it'll throw a non-intercepted exception and the application will crash
		// horribly. After the mapping procedure, we will receive a collection object that must
		// be "consumed", as they call it here in Rust. We use the collect() consumer to wrap the
		// data into a vector of u8 integers.
		let input = value
			.chars()
			.map( |x| x.to_string().parse::<u8>().unwrap() )
			.collect::<Vec<u8>>();

		// Then we loop through array indexes and assign them the values of the vector that we've
		// got above. We use get() to access the vector value of particular index, then unwrap the
		// Result which gives us the reference to desired value, which, in turn, we dereference to
		// an actual number.
		for i in 0..4 {
			array[i] = *input.get(i).unwrap();
		}

		// That is all, return generated array
		array
	}
}
