/**
 * @types
 */

 
// signed integers(i8, i16, i32, i64 and isize)
// unsigned integers(u8, u16, u32, u64 and usize), 
// floating point, (f32, f64)
// char, unicode scalar values like 'a', 'α' and '∞'
// bool, true or false
// arrays, [1, 2, 3]
// vectors, re-sizable arrays <Vec>
// tuples, (1, true)

 
/**
 * @examples
 */


let integer: i32       = 10;
let double: f64        = 7.2320430234;
let float: f32         = 7.2;
let boolean: bool      = true;
let string: &str       = "String" // uses double "" tick
let string             = "String" // ^ same thing
let character          = 'x'; // uses single tick ` ' `
let arr: [&str; 3]     = ["String 1", "String 2", "String 3"]; // fixed size array
let arr                = ["String 1", "String 2", "String 3"]; // ^ same thing
let mut arr: Vec<&str> = vec!["String 1", "String 2", "String 3"]; // a dynamic array (vector)
let mut arr:           = vec!["String 1", "String 2", "String 3"]; // same thing
let tpl: (i32, &str)   = (1, "hello");
let tpl                = (1, "hello"); // ^ same thing
let fc: fn(i32) -> i32 = foo;

// you can also use <T: Type> to case a custom type
// for example <T: Any>


/**
 * ---------------------------------------------------------------------------------
 * 
 * @Mutability
 * 
 * ---------------------------------------------------------------------------------
 */

// everything is immutable  but this can be overridden using the mut modifier.

let foo: i32 = 1; // immutable
let mut bar: i32 = 1; // mutable

// Error! The type of a variable can't be changed
bar = true;


/**
 * ---------------------------------------------------------------------------------
 * 
 * @conditionals
 * 
 * ---------------------------------------------------------------------------------
 */


/**
 * if `CONDITION`{ 
 *  	`STATEMENT(s)`
 * } else { 
 * 		`STATEMENT(s)`
 * }
 */
 

/**
 * @examples
 */

  
if (1 > 2 || 1 > 4) && true == true {
	println!("1 is greater than 2 or 4");
} else {
	println!("1 is less than 2");
}


/**
 * ---------------------------------------------------------------------------------
 * 
 * @operators
 * 
 * ---------------------------------------------------------------------------------
 */


/**
 *   <    `less than` 
 *   >    `greater than` 
 *   <=   `less than or equal to`
 *   >=   `greater than or equal to`
 *   !    `NOT`
 *   &&   `AND`
 *   ||   `OR`
 *   %    `modulas`
 *   +    `addition`
 *   -    `subtraction`
 *   *    `multiplication`
 *   /    `division`
 *   !=   `not equal`
 *   ==   `equal`
 * 
 */


 /**
  * ---------------------------------------------------------------------------------
  * 
  * @while loops
  * 
  * ---------------------------------------------------------------------------------
  */

  // A while loop performs a set of statements until a condition becomes false.

 /**
  * while `condition` {
  * 		`statements`
  * }
  */

 /**
  * @examples
  */


var i = 1
while i <= 10 {
    println!(i);
    i = i + 1;
}


/**
 * ---------------------------------------------------------------------------------
 * 
 * @for loops
 * 
 * ---------------------------------------------------------------------------------
 */


/**
 * for `var` in `expression` {
 *     `statements`
 * }
 */

/**
 * @examples
 */

// prints 1-10
for i in 0..11 {
    println!(i);
}

// prints 10-1
for i in 11..0 {
    println!(i);
}


// ...to be continued ->


/**
 * ---------------------------------------------------------------------------------
 * 
 * @arrays
 * 
 * ---------------------------------------------------------------------------------
 */


let ints: [i32; 3]  = [1, 2, 3];
let strs: [&str; 3] = ["A", "B", "C"];
let buls: [bool; 3] = [true, false, true];
let flts: [f64; 3]  = [1.0, 2.0, 3.0];

let length = strs.len();     // 2
let value  = strs[0];        // "A"

// slices point to a section of an array
let slice  = &str[1 .. 2];   // ["B", "C"]
let slice: [&str] = &strs;   // ["A", "B", "C"]

// all of the above arrays are all immutable length arrays, 
// vectors are what you would use for arrays of changine length
let mut ints: Vec<&i32>  = vec![1, 2, 3];
let mut strs: Vec<&str>  = vec!["A", "B", "C"];
let mut buls: Vec<&bool> = vec![true, false, true];
let mut flts: Vec<&f64>  = vec![1.0, 2.0, 3.0];

// we could leave out the explicit mention of Vec<>
let mut ints = vec![1, 2, 3];
let mut strs = vec!["A", "B", "C"];
let mut buls = vec![true, false, true];
let mut flts = vec![1.0, 2.0, 3.0];

str.append("D")             // str => ["A", "B", "C", "D"]
str.insert(1, "F")          // str => ["A", "F", "B", "C", "D"]

// concatenation
let str2: Vec<&str> = vec![4, 5, 6];
str.append(&mut str2);     // ints => [1, 2, 3, 4, 5, 6]

buls.remove(0);            // buls => [false, true]
flts[2] = 4.0              // flts => [1.0, 2.0, 4.0]

// trying to access an index that does not exist
let v = vec![1, 2, 3];
println!("Item 7 is {}", v[7]);

// then the current thread will panic with a message like this:
// thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 7'

// If you want to handle out-of-bounds errors without panicking, 
// you can use methods like get or get_mut that return None when given an invalid index:
let v = vec![1, 2, 3];
match v.get(7) {
    Some(x) => println!("Item 7 is {}", x),
    None => println!("Sorry, this vector is too short.")
}


/**
 * ---------------------------------------------------------------------------------
 * 
 * @functions
 * 
 * ---------------------------------------------------------------------------------
 */

/**
 * general syntax
 * 
 * fn name(list of parameters) -> return type {
 * 		statements
 * }
 *
 * // some functions don’t return any values
 * 
 * func name(list of parameters) {
 * 		statements
 * }
 *
 * // parameters are followed by their type.
 * // expressions without a `;` a treated as return values
 * // but you can also use return explicity
 */

/**
 * @examples
 */

fn toUpperCase(str: &str, num: &i32, arr: Vec<&str>) -> &str {
	str // returns str, `return str;` is also valid but `str;` will return ()
}

// rust has special syntax for ‘diverging functions’, 
// which are functions that do not return:
fn doesNotReturn() -> ! {
    println!("this function never returns!");
}

// not to be confused with functions that return () `an empty tuple`
fn doesReturn() -> () {
    str;
}

// you can also create variable bindings which point to functions:
let toUpperCase: fn(str: &str, num: &i32, arr: Vec<&str>) -> &str = toUpperCase;
// we can write the same thing as, the types will be infered
let toUpperCase = toUpperCase;

// which you can call the function like
toUpperCase("A String", 10, ["String"]);


/**
 * ---------------------------------------------------------------------------------
 * 
 * @dictionaries
 * 
 * ---------------------------------------------------------------------------------
 */


// a dictionary is a container that stores multiple values of the same type. 
// each value is associated with a unique key, which acts as an identifier for 
// that value within the dictionary

// create
let mut dictionary = ::std::collections::HashMap::new();

// insert
contacts.insert("one", 1);
contacts.insert("two", 2);
contacts.insert("three", 3);


// access
match dictionary.get("one") {
    Some(value) => // do something with it if it exists
    None => // do something if it does not exist
}

// what if we want to replace a keys value
let mut stringsAsInts = HashMap::new();

stringsAsInts.insert("zero", 0);
stringsAsInts.insert("one", 1);
stringsAsInts.insert("two", 2);

contacts.insert("three", 3);

// using .insert you can change a value associated with a key:
contacts.insert("three", 10);

// to remove a value from the dictionary 
// you can use the .remove method on the dictionary
stringsAsInts.remove("The Adventures of Sherlock Holmes");

// to iterate over the dictionary you could do
for (key, val) in stringsAsInts.iter() {
    println!("key: {} val: {}", key, val);
}
// or
for key in stringsAsInts.keys() {
    println!("{}", key);
}

// there are no map literals in rust. however, you can create a macro
macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(m.insert($key, $value);)+m
        }
     };
);

// use case
let names = map!{1 => "one",2 => "two"};


/**
 * ---------------------------------------------------------------------------------
 * 
 * @tuples
 * 
 * ---------------------------------------------------------------------------------
 */

// a tuple is a group of zero or more values represented as one value.
// the type of a tuple is determined by the values it has. 
// so ("tuple", 1, true) will be of type (&str, i32, bool). 
// () is the empty tuple – it has no elements. it also represents the void-like type.
// tuple elements in rust do not have a name

// explicit type
type Person<'a> = (&str, i32);
let person: Person<'static> = ("Midhun", 7);

// infered type 
let person = ("Midhun", 7);

// access
let (name, age) = person;
// or
person.0
person.1

// you can use a tuple to return multiple values from a function as a single compound value.
fn abc() -> (i32, i32, &str) {
    return (3, 5, "Carl"); // see @functions section this can also become just `(3, 5, "Carl")`
}


/**
 * ---------------------------------------------------------------------------------
 * 
 * @structs
 * 
 * ---------------------------------------------------------------------------------
 */


// syntax begins with the `struct` keyword followed by the `name` and a pair of curly braces. 
// everything in-between the curly braces is a `member` of the struct.
struct Location {
    latitude: f64,
    longitude: f64,
}

// tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);

// you can instantiate one and store it in a constant or variable like any other type
let pizzaLocation: Location = Location { latitude: 44.9871, longitude: -93.2758 }

// nested structs
struct DeliveryRange {
	range: f64
	center: Location
}

let storeLocation: Location = Location { latitude: 44.9871, longitude: -93.2758}
let mut pizzaRange: DeliveryRange = DeliveryRange { range: 200, center: storeLocation}

// access
println!(pizzaRange.range)           // => 200

// since pizzaRange.center is a Location struct
println!(pizzaRange.center.latitude) // => 44.9871

// re-assign
pizzaRange.range = 250


// much like a struct can have constants and variables, it can also define its own functions:
struct DeliveryRange {
	range: f64
	center: Location
}

// above we’ve made a struct that represents a DeliveryRange. 
// we then write an impl block, and inside it, define a method, isInRange.
impl DeliveryRange {
	// methods take a special first parameter, 
	// of which there are three variants: self, &self, and &mut 
	// `self` if it’s a value on the stack, `&self` if it’s a reference, and `&mut self` if it’s a mutable reference.
	// we mainly use `&self` and `&mut self`
	// it this case we use &self because we do not need to mutate center and range
	// NOTE: functions without the explicit self parameter are considered associated functions, 
	// which can be called without a specific instance.
	fn isInRange(&self, customer: Location) -> bool {
		let center: Location = self.center;
    	let difference = ((customer.latitude - center.latitude).powi(2) + (customer.longitude - center.longitude).powi(2)).sqrt();

    	return difference < self.range;
	}
}

// instantiate
let range: DeliveryRange = DeliveryRange(150, Location(44.9871, -93.2758));
let customer: Location = Location(44.9850, -93.2750);
 
// call method
range.isInRange(customer) // => true!
