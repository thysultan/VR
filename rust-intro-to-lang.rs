/**
 * @types
 */

 
// signed integers(i8, i16, i32, i64 and isize), i32 is the main actor
// unsigned integers(u8, u16, u32, u64 and usize), 
// floating point, (f32, f64) f64 is the main actor
// char,  unicode scalar values like 'a', 'α' and '∞'
// bool, true or false
// arrays, [1, 2, 3]
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
let arr: [&str; 3]     = ["String 1", "String 2"]; 
let arr                = ["String 1", "String 2"]; // same thing
let tpl: (i32, &str)   = (1, "hello");
let tpl                = (1, "hello"); // same thing
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