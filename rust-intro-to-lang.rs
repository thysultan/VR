/**
 * @types
 */

 
// signed integers(i8, i16, i32, i64 and isize), i32 is the main actor
// unsigned integers(u8, u16, u32, u64 and usize), 
// floating point, (f32, f64) f64 is the main actor
// char,  unicode scalar values like 'a', 'α' and '∞'
// bool, true or false
// arrays, [1, 2, 3]
// vectors, re-sizable arrays
// tuples, (1, true)

 
/**
 * @examples
 */


let integer: i32       = 10;
let float: f64         = 7.2;
let boolean: bool      = true;
let string: &str       = "String"
let string             = "String" // ^ same thing
let character          = 'x'; // uses single tick ` ' `
let arr: [&str; 3]     = ["String 1", "String 2", "String 3"]; // fixed size array
let arr                = ["String 1", "String 2", "String 3"]; // ^ same thing
let mut arr: Vec<&str> = vec!["String 1", "String 2", "String 3"]; // a dynamic array (vector)
let mut arr:           = vec!["String 1", "String 2", "String 3"]; // same thing
let tpl: (i32, &str)   = (1, "hello");
let tpl                = (1, "hello"); // ^ same thing
let fc: fn(i32) -> i32 = foo;


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
 *   !    `negation' i.e !true == false
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
for x in 0..11 {
    print(i)
}

// prints 10-1
for x in 11..0 {
    print(i)
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

// all of the above arrays are all immutable, below are mutable arrays called vectors
let mut ints: Vec<&i32>  = vec![1, 2, 3];
let mut strs: Vec<&str>  = vec!["A", "B", "C"];
let mut buls: Vec<&bool> = vec![true, false, true];
let mut flts: Vec<&f64>  = vec![1.0, 2.0, 3.0];

// we could leave the explicit mention of Vec<>
let mut ints = vec![1, 2, 3];
let mut strs = vec!["A", "B", "C"];
let mut buls = vec![true, false, true];
let mut flts = vec![1.0, 2.0, 3.0];

str.append("D")             // str => ["A", "B", "C", "D"]
str.insert(1, "F")          // str => ["A", "F", "B", "C", "D"]

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

// ... to be continued ->