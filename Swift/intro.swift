/**
 * ---------------------------------------------------------------------------------
 * 
 * @types
 * 
 * ---------------------------------------------------------------------------------
 */

 
// Int, Bool, String, Double, Any, Float, Void, Array, Tuple, () -> T, Character, Generic Types <T>

 
/**
 * @examples
 */


var integer: Int      = 10
var integer: Int      = Int(double) 
var boolean: Bool     = true
var double: Double    = 7.2320430234 
var double: Double    = Double(integer)
var float: Fouble     = 7.2
var float: Fouble     = Float(integer)
var string: String    = "String"                  // uses double "" tick
var any: Any?         = nil                       // ? indicates the value could be nil
var fn: () -> Int     = func foo() -> Int {}
var arr: [String]     = ["String 1", "String 2"]
var tpl: Tuple        = (name: "Sultan", age: 25)
var gen: T?                                       // generic type
var char: Character   = "A"


/**
 * ---------------------------------------------------------------------------------
 * 
 * @Mutability
 * 
 * ---------------------------------------------------------------------------------
 */


// anything created with `var` is mutable, anything created with `let` is immutable

var foo: Int = 1 // mutable
let bar: Int = 1 // immutable

// Error! The type of a variable can't be changed
foo = true

var foo: Any = 1

// Ok!
foo = true



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
	print("1 is greater than 2 or 4");
} else {
	print("1 is less than 2");
}

// short hand
(1 > 2 || 1 > 4) && true == true ? print("1 is greater than 2 or 4") : print("1 is less than 2")


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
 *   ===  `identical to`
 *   !==  `not identical to`
 *   ??   `the null coalescing operator`
 *
 *   note that “identical to” (represented by three equals signs, or ===) 
 *   does not mean the same thing as “equal to” (represented by two equals signs, or ==):
 *   “identical to” means that two constants or variables of class type 
 *   refer to exactly the same class instance.
 *   “equal to” means that two instances are considered “equal” or “equivalent” in value
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
    print(i)
    i = i + 1
}


/**
 * ---------------------------------------------------------------------------------
 * 
 * @for-in loops
 * 
 * ---------------------------------------------------------------------------------
 */


/**
 * for `value` in `range` {
 *     `statements`
 * }
 */

/**
 * @examples
 */

// prints 1-10
for i in 1...10 {
    print(i)
}

// prints 0-9
for i in 0..<10 {
    print(i)
}


/**
 * ---------------------------------------------------------------------------------
 * 
 * @for loops
 * 
 * ---------------------------------------------------------------------------------
 */


/**
 * for `initialization`; `condition`; `increment` {
 *     `statements`
 * }
 */

/**
 * @examples
 */

// prints 1-10
for var i = 1; i <= 10; i = i + 1 {
    print(i)
}

// prints 10-1
for var i = 10; i > 0; --i {
    print(i)
}

// just executing a statement multiple times but don’t care about having an index?
// the convention in for loops is to use _ as the loop variable name 
// when you don’t intend to use the variable in the loop.

// for examplet to print “Hello World” 5 times you can use:
for _ in 1...5 {
    print("Hello World")
}


/**
 * ---------------------------------------------------------------------------------
 * 
 * @arrays
 * 
 * ---------------------------------------------------------------------------------
 */


var ints: [Int]    = [1, 2, 3]
var strs: [String] = ["A", "B", "C"]
var buls: [Bool]   = [true, false, true]
var dbls: [Double] = [1.0, 2.0, 3.0]

// initialize an empty array
var empty: [Any]()

var length = strs.count     // 2
var value  = strs[0]        // "A"
var slice  = str[1...2]     // ["B", "C"]

str.append("D")             // str => ["A", "B", "C", "D"]
str.insert("F", atIndex: 1) // str => ["A", "F", "B", "C", "D"]

ints += [4, 5, 6]           // ints => [1, 2, 3, 4, 5, 6]

buls.removeAtIndex(0)       // buls => [false, true]

dbls[2] = 4.0               // dbls => [1.0, 2.0, 4.0]

var ints: [Int] = [1, 2, 3, 4, 5, 6]
ints[2...4]     = [0, 0]    // ints => [1, 2, 0, 0, 6]
ints[2...4]     = [0, 0, 0] // ints => [1, 2, 0, 0, 0]


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
 * func name(list of parameters) -> return type {
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
 */

/**
 * @examples
 */

func toUpperCase (str: String, num: Int, arr: [String]) -> String {
	return str;
}

// which you can call the function like
toUpperCase("A String", 10, ["String"]);
// or
toUpperCase("A String", num: 10, arr: ["String"])

// where () is a tuple, though an empty tuple in this case representing the Void type
func doNothing () -> () {
	print("nothing");
}

// one could write the same thing as
func doNothing () {
	print("nothing");
}

// default parameters
func countdown(from: Int, to: Int = 1) {
	print(from + to);
}
countdown(5, to: 3)

// swift will automatically assign an external parameter name to 
// any parameter with a default value so that it’s purpose is clear. 
// to stop this from happening write an underline (_) 
// before the parameter with default value.
func countdown(from: Int, _ to: Int = 1) {
	print(from + to);
}
countdown(5, 3)

// function parameters are constants by default
// to change the value of a parameter declare the parameter with the var keyword. 
// The changes made to the parameter will only be present inside the function call.
func digits(var number: Int) {
	number = number / 10
}

// variable parameters can only be changed inside a function. 
// If you want the function to change the value of a parameter 
// and you want those changes to persist after the function call, 
// define the parameter as an inout parameter.

// Keep in mind that you can only pass variables as in-out parameters. 
// you cannot pass a constant or a literal value, because they cannot be changed. 
// you have to write an ampersand (&) in front of the variable name when calling the function. 
// that will indicate that the variable can be modified by the function.
func doubleANumber(inout number: Int) {
    number = number * 2
}

var n = 10

doubleANumber(&n)

print(n) // => 20

// sometimes we want a method that receives any number of arguments. 
// this is a variadic func. we use an ellipsis to indicate a variadic method
// then we can use the argument as an array in the method body. 
func sumStringLengths(values: String...) -> Int {
    var sum: Int = 0

    for value in values {
		sum += value.characters.count
    }

    return sum
}

// call the method with 2 arguments.
let result = sumStringLengths("bird", "cat")
print(result)


/**
 * ---------------------------------------------------------------------------------
 * 
 * @dictionaries
 * 
 * ---------------------------------------------------------------------------------
 */


// A dictionary is a container that stores multiple values of the same type. 
// each value is associated with a unique key, which acts as an identifier for 
// that value within the dictionary

// syntax([KeyType: ValueType]) i.e 
// var foo = [key: value, key: value, ...] or 
// var foo: [String: String] = [key: value, key: value, ...] or 
// var foo: Dictionary<String, String> = [key: value, key: value, ...]

var dictionary: [String: Int] = [
    "one" : 1,
    "two" : 2,
    "three" : 3
]

var emptyDictionary: [Int: Int] = [:]

// access specific elements using subscript syntax, 
// pass the key of the value within square brackets immediately after name of dictionary. 
// because it’s possible not to have a value associated with the provided key 
// the subscript will return an optional value of the value type.

// to unwrap the value returned by the subscript you can do one of two things: 
// use optional binding or force the value if you know for sure it exists.

dictionary["one"] // Optional(1)

// unwaraping the optional using optional binding
if let twoAsInt = dictionary["two"] {
    print(twoAsInt) // 2
}

// unwaraping the optional using the forced value operator (!)
dictionary["one"]! // 1

// nil-Coalescing operator ??
value = dictionary["one"] ?? "OK"
// the same as doing
value != nil ? dictionary["one"]! : "OK"

// simplest way to add a value to a dictionary is by using the subscript syntax:
var stringsAsInts: [String: Int] = [
    "zero" : 0,
    "one" : 1,
    "two" : 2
]

// adds a new key->value pair to the dictionary
stringsAsInts["three"] = 3

// using the subscript syntax you can change a value associated with a key:
stringsAsInts["three"] = 10

// to remove a value from the dictionary 
// you can use the subscript syntax to set the value to nil
stringsAsInts["three"] = nil

// to iterate over the dictionaryyou could do
for (key, value) in stringsAsInts {

}


/**
 * ---------------------------------------------------------------------------------
 * 
 * @tuples
 * 
 * ---------------------------------------------------------------------------------
 */


// a tuple is a group of zero or more values represented as one value.
// the type of a tuple is determined by the values it has. 
// so ("tuple", 1, true) will be of type (String, Int, Bool). 
// () is the empty tuple – it has no elements. It also represents the Void type.

// named tuple declare, assign
let person = (name:"Midhun", age:7)

(0.0, 4.5);
("a", 4usize, true);

// access
person.name
person.age

// unnamed tuple declare, assign
let person = ("Midhun", 7)

// access
person.0
person.1

// you can use a tuple to return multiple values from a function as a single compound value.
func abc() -> (Int, Int, String) {
    return (3, 5, "Carl")
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
	let latitude: Double
	let longitude: Double
}

// you can instantiate one and store it in a constant or variable like any other type
let pizzaLocation = Location(latitude: 44.9871, longitude: -93.2758)

// nested structs
struct DeliveryRange {
	var range: Double
	let center: Location
}

let storeLocation = Location(latitude: 44.9871, longitude: -93.2758)
var pizzaRange = DeliveryRange(range: 200, center: storeLocation)

// access
print(pizzaRange.range)           // => 200

// since pizzaRange.center is a Location struct
print(pizzaRange.center.latitude) // => 44.9871

// re-assign
pizzaRange.range = 250

// much like a struct can have constants and variables, it can also define its own functions:
struct DeliveryRange {
	var range: Double
	let center: Location
 
	func isInRange(customer: Location) -> Bool {
    	let difference = sqrt(pow((customer.latitude - center.latitude), 2) +
			pow((customer.longitude - center.longitude), 2))

    	return difference < range
	}
}

// instantiate
let range = DeliveryRange(range: 150, center: Location(coordinateString: "44.9871,-93.2758"))
let customer = Location(coordinateString: "44.9850,-93.2750")
 
// call method
range.isInRange(customer) // => true!

// the main difference between classes and structs is 
// that structs are value types and classes are reference types.
// when you make a copy of a value type, 
// it copies all the data from the thing you are copying into the new variable. 
// they are 2 seperate things and changing one does not affect the other

// when you make a copy of a reference type, 
// the new variable refers to the same memory location as the thing you are copying. 
// this means that changing one will change the other 
// since they both refer to the same memory location


/**
 * ---------------------------------------------------------------------------------
 * 
 * @classes
 * 
 * ---------------------------------------------------------------------------------
 */


// classes and structures have a similar definition syntax. 
// you introduce classes with the class keyword and structures with the struct keyword. 
// Both place their entire definition within a pair of braces:
class SomeClass {
    // class definition goes here
}
struct SomeStructure {
    // structure definition goes here
}

// NOTE: Whenever you define a new class or structure, you effectively define a brand new Swift type. 
// give types UpperCamelCase names (such as SomeClass and SomeStructure here) 
// to match the capitalization of standard Swift types (such as String, Int, and Bool). 
// conversely, always give properties and methods lowerCamelCase names 
// (such as frameRate and incrementCount) to differentiate them from type names.

struct Resolution {
    var width = 0
    var height = 0
}
class VideoMode {
    var resolution = Resolution()
    var interlaced = false
    var frameRate = 0.0
    var name: String?
}

// the syntax for creating instances is very similar for both structures and classes:
let someResolution = Resolution()
let someVideoMode = VideoMode()

/**
 * @examples
 */

class SurveyQuestion {
    let text: String
    var response: String?
    init(text: String) {
        self.text = text
    }
    func ask() {
        print(text)
    }
}
let cheeseQuestion = SurveyQuestion(text: "Do you like cheese?")
cheeseQuestion.ask()
// prints "Do you like cheese?"
cheeseQuestion.response = "Yes, I do like cheese."

// NOTE: you can assign a value to a constant property at any point during initialization, 
// as long as it is set to a definite value by the time initialization finishes. 
// once a constant property is assigned a value, it can’t be further modified.


// Swift provides a default initializer for any structure or class 
// that provides default values for all of its properties and 
// does not provide at least one initializer itself. 
// The default initializer simply creates a new instance with all of its properties 
// set to their default values.
class ShoppingListItem {
    var name: String?
    var quantity = 1
    var purchased = false
}
var item = ShoppingListItem()
