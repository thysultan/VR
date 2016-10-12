/**
 * ---------------------------------------------------------------------------------
 * 
 * @types
 * 
 * ---------------------------------------------------------------------------------
 */

 
// number, boolean, string, function, undefined, null, Array, Object

 
/**
 * @examples
 */


var integer           = 10
var integer           = double|0
var boolean           = true
var double            = 7.2
var float             = 7.2
var string            = "String" // uses double "" or single '' or even `` ticks
var any               = null
var fn                = () -> {}


/**
 * ---------------------------------------------------------------------------------
 * 
 * @Mutability
 * 
 * ---------------------------------------------------------------------------------
 */


// anything created with `var` or `let` is mutable, anything created with `const` is immutable

var foo = 1 // mutable
let bar = 1 // mutable
const faz = 1 // immutable

// Error! constants can't be changed
faz = true;

// OK!
foo = 1
// OK!
bar = 1


/**
 * ---------------------------------------------------------------------------------
 * 
 * @conditionals
 * 
 * ---------------------------------------------------------------------------------
 */


/**
 * if (`CONDITION`) { 
 *      `STATEMENT(s)`
 * } else if (`CONDITION`) { 
 *      `STATEMENT(s)`
 * } else { 
 *      `STATEMENT(s)`
 * }
 */
 

/**
 * @examples
 */

  
if ((1 > 2 || 1 > 4) && true == true) {
	console.log("1 is greater than 2 or 4");
} else {
	console.log('1 is less than 2');
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
 *   %    `modulo`
 *   +    `addition`
 *   -    `subtraction`
 *   *    `multiplication`
 *   /    `division`
 *   ==  `identical to`
 *   !=  `not identical to`
 *   !==   `not equal`
 *   ===   `equal`
 *   ?    `conditional (ternary) operator`
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
 * while (`condition`) {
 * 		`statements`
 * }
 */

/**
 * @examples
 */

var i = 1
while (i <= 10) {
    console.log(i)
    i = i + 1
}



/**
 * ---------------------------------------------------------------------------------
 * 
 * @for loops
 * 
 * ---------------------------------------------------------------------------------
 */


/**
 * for (`initialization`; `condition`; `increment`) {
 *     `statements`
 * }
 */

/**
 * @examples
 */

// prints 1-10
for (var i = 1; i <= 10; i = i + 1) {
    console.log(i)
}

// prints 10-1
for (var i = 10; i > 0; --i) {
    console.log(i)
}


/**
 * ---------------------------------------------------------------------------------
 * 
 * @arrays
 * 
 * ---------------------------------------------------------------------------------
 */


// though arrays in javascript can hold any type of value arrays can act
// as typed arrays post VM's optimization if they remain monomorphic
var ints           = [1, 2, 3]
var strs           = ["A", "B", "C"]
var buls           = [true, false, true]
var dbls           = [1.0, 2.0, 3.0]

var length = strs.length      // 2
var value  = strs[0]          // "A"
var slice  = strs.slice(1)    // ["B", "C"]

str.push("D")                 // str => ["A", "B", "C", "D"]
str.splice(0, 0, "F")         // str => ["A", "F", "B", "C", "D"]

ints = ints.concat([4, 5, 6]) // ints => [1, 2, 3, 4, 5, 6]


buls.splice(0, 1)             // buls => [false, true]

dbls[2] = 4.0                 // dbls => [1.0, 2.0, 4.0]


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
 * function name(list of parameters) {
 * 		statements
 * }
 *
 */

/**
 * @examples
 */

function toUpperCase (str, num, arr) {
	return str;
}

// which you can call the function like
toUpperCase("A String", 10, ["String"]);

// default parameters
func countdown(from, to = 1) {
	console.log(from + to);
}
countdown(5, 3)

// sometimes we want a method that receives any number of arguments. 
// this is a variadic function. 
function sumStringLengths() {
    var sum = 0;

    for (var key in arguments) {
        var value = arguments[key];
		sum += value.length;
    }

    return sum;
}

// call the method with 2 arguments.
let result = sumStringLengths("bird", "cat")
console.log(result)


/**
 * ---------------------------------------------------------------------------------
 * 
 * @dictionaries
 * 
 * ---------------------------------------------------------------------------------
 */


// A dictionary is a container that stores multiple values
// each value is associated with a unique key, which acts as an identifier for 
// that value within the dictionary

// syntax({KeyType: ValueType}) i.e {key: value, key: value, ...}
// 
var dictionary = {
    one: 1,
    two: 2,
    three: 3
}

var emptyDictionary = {}

// simplest way to add a values to a dictionary:
var stringsAsInts = {
    zero: 0,
    one: 1,
    two: 2
}

// adds a new key->value pair to the dictionary
stringsAsInts["three"] = 3

// using the subscript syntax you can change a value associated with a key:
stringsAsInts["three"] = 10

// to remove a value from the dictionary 
// you can use the delete keyword
delete stringsAsInts["three"]


/**
 * ---------------------------------------------------------------------------------
 * 
 * @classes
 * 
 * ---------------------------------------------------------------------------------
 */


// you introduce classes with the class keyword with the definition within a pair of braces:
class SomeClass {
    // class definition goes here
}

// NOTE: Whenever you define a new class, you effectively define a brand new type. 
// give types UpperCamelCase names (such as SomeClass and SomeStructure here) 
// to match the capitalization of standard JS types (such as Object, Array). 
// conversely, always give properties and methods lowerCamelCase names 
// (such as frameRate and incrementCount) to differentiate them from type names.

class VideoMode {
    constructor () {
        this.resolution = Resolution()
        this.interlaced = false
        this.frameRate = 0.0
        this.name = ""
    }
}

// the syntax for creating instances
let someResolution = new Resolution()
let someVideoMode = new VideoMode()

/**
 * @examples
 */

class SurveyQuestion {
    constructor(text) {
        this.text = text;
        this.response = "";
    }
    ask() {
        console.log(this.text);
    }
}

let cheeseQuestion = new SurveyQuestion("Do you like cheese?")
cheeseQuestion.ask()
// prints "Do you like cheese?"
cheeseQuestion.response = "Yes, I do like cheese."
