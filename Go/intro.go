/**
 * ---------------------------------------------------------------------------------
 * 
 * @types
 * 
 * ---------------------------------------------------------------------------------
 */


 
// bool, 
// string, strings are immutable, if you want in-place modifications to character data use an array of bytes []byte
// int, int8, int16, int32, int64, 
// uint, uint8, uint16, uint32, uint64, uintptr
// float32, float64
// byte
// interface{}, the <Any> type in go
// when you need an integer value you should use int 
// unless you have a specific reason to use a sized or unsigned integer type.


var integer int       = 10
var integer int       = int(double) 
var boolean bool      = true
var double float64    = 7.2320430234 
var double float64    = float64(integer)
var float float32     = 7.2
var float float32     = float32(integer)
var string string     = "String"                  // uses double "" tick
var any interface{}   = nil // interface{} is <Any> type
var fn func() int     = func () int { return 0 }
var arr [2]string     = ["String 1", "String 2"]
var char rune         = '⌘' // a rune with integer value 0x2318, uses single ''tick
var bytes []byte      = ["H", "e", 'l', 'l', 'o']


/**
 * ---------------------------------------------------------------------------------
 * 
 * @keywords
 * 
 * ---------------------------------------------------------------------------------
 */


// break        default      func         interface    select
// case         defer        go           map          struct
// chan         else         goto         package      switch
// const        fallthrough  if           range        type
// continue     for          import       return       var


/**
 * ---------------------------------------------------------------------------------
 * 
 * @Mutability
 * 
 * ---------------------------------------------------------------------------------
 */


// anything created with `var` is mutable, anything created with `const` is immutable

var foo = 1 // mutable
const faz = 1 // immutable

// Error! constants can't be changed
faz = true

// OK!
foo = 1

// NOTE: variables that start with an Uppercase letter (i.e "Something") are exported by default
// this ties into packages and imports that will not get covered in this intro but a good
// styleguide to be aware of non-the-less.


/**
 * ---------------------------------------------------------------------------------
 * 
 * @if-else conditionals
 * 
 * ---------------------------------------------------------------------------------
 */


/**
 * if `CONDITION`{ 
 *      `STATEMENT(s)`
 * } else if `CONDITION` { 
 *      `STATEMENT(s)`
 * } else { 
 *      `STATEMENT(s)`
 * }
 */
 

/**
 * @examples
 */


if ((1 > 2 || 1 > 4) && true == true) {
	fmt.Println("1 is greater than 2 or 4")
	// NOTE: though fmt is a default package that comes with golang 
	// you cannot use it without first importing it. import ("fmt")
} else {
	fmt.Println('1 is less than 2')
}


/**
 * ---------------------------------------------------------------------------------
 * 
 * @switch conditionals
 * 
 * ---------------------------------------------------------------------------------
 */


/**
 * switch `CONDITION` { 
 *     case `CASE`: {
 *         `STATEMENT(s)`
 *     }
 * }
 */


/**
 * @examples
 */
switch 0 {
    case 0: {
        print("0")
    }
    case 1: {
        print("1")
    }
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


/**
 * a for loop without the optionals
 * 
 * for ; `condition`; {
 *     `statements`
 * }
 */

var i = 1
for ; i <= 10; {
    fmt.Println(i)
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
 * for `initialization`; `condition`; `increment` {
 *     `statements`
 * }
 *
 * the `initialization`  and `increment` are optional
 * 
 * for ; `condition`; {
 *     `statements`
 * }
 */

/**
 * @examples
 */

// prints 1-10
for var i = 1; i <= 10; i = i + 1 {
    fmt.Println(i)
}

// prints 10-1
for var i = 10; i > 0; --i {
    fmt.Println(i)
}


/**
 * ---------------------------------------------------------------------------------
 * 
 * @arrays
 * 
 * ---------------------------------------------------------------------------------
 */


var ints [3]int     = {1, 2, 3}
var strs [3]string  = {"A", "B", "C"}
var buls [3]bool    = {true, false, true}
var dbls [3]float64 = {1.0, 2.0, 3.0}

// all of the above arrays are all immutable length arrays, 
// slices are what you would use for arrays of varying length
// a slice is declared just like an array except we leave out the element count
var ints []int     = {1, 2, 3}
var strs []string  = {"A", "B", "C"}
var buls []bool    = {true, false, true}
var dbls []float64 = {1.0, 2.0, 3.0}

var length = len(strs)                 // 3
var value = strs[0]                    // "A"
var slice = str[1:2]                   // ["B", "C"]

strs = append(str, "D")                 // str => ["A", "B", "C", "D"]

// insert
// 1. make sure there is enough room
strs = append(strs, 0)
// 2. move all elements of strs up one slot
copy(strs[1+1:], strs[1:])             // format -> copy(str[i+1:], s[i:])
// insert the new element at the now free position
strs[1] = "F"                          // str => ["A", "F", "B", "C", "D"]

// concatenation
var ints2 []int = {4, 5, 6} // the slice to concatenate
var ints []int = append(ints, ints2...)          // ints => [1, 2, 3, 4, 5, 6]
var ints []int = append(ints, []int{4, 5, 6}...) // ints => [1, 2, 3, 4, 5, 6]

// move all elements of buls down one slot
copy(buls[0:], buls[0+1:]) // format -> copy(buls[i:], buls[i+1:])
// assign nil to element to remove
buls[len(buls)-1] = nil
// remove element
buls = buls[:len(buls)-1]                        // buls => [false, true]

// assignment
dbls[2] = 4.0                                    // dbls => [1.0, 2.0, 4.0]


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
 * func name(list of parameters) return type {
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


func toUpperCase (str string, num int, arr string) string {
	return str
}

// which you can call the function like
toUpperCase("A String", 10, ["String"])

// where () is a tuple, an empty tuple that represents the Void type
func doNothing () -> () {
	print("nothing")
}

// even though golang does not have tuples
// functions can return multiple values
func countdown() (int, int) {
	return 1, 2
}

// use case
var a, b = countdown() // a => 1, b => 2


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

// syntax(map[`KeyType`]`ElementType`) i.e 
// var foo: map[string]int = {key: value}

var dictionary: map[string]int = {
    "one": 1,
    "two": 2,
    "three": 3
}

var emptyDictionary: map[int]int = {}

// access specific elements using subscript syntax, 
// pass the key of the value within square brackets immediately after name of dictionary. 
dictionary["one"] // 1

// simplest way to add a value to a dictionary is by using the subscript syntax:
var stringsAsInts: map[string]int = {
    "zero": 0,
    "one": 1,
    "two": 2
}

// adds a new key->value pair to the dictionary
stringsAsInts["three"] = 3

// using the subscript syntax you can change a value associated with a key:
stringsAsInts["three"] = 10

// to remove a value from the dictionary 
delete(dictionary, "three")

// to iterate over the dictionary
// NOTE: the := syntax is short that you can use withint function scope
// to declare and assign variables, i.e var num int = 1, could be num := 1
for key, value := range stringsAsInts {
    fmt.Println("key: {} val: {}", key, val)
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

type Location struct {
	latitude float64
	longitude float64
}

// you can instantiate one and store it in a constant or variable like any other type
var pizzaLocation Location = Location{44.9871, -93.2758}

// nested structs
type DeliveryRange struct {
	range float64
	center Location
}

var storeLocation Location = Location{44.9871, -93.2758}
var pizzaRange DeliveryRange = DeliveryRange{200, storeLocation}

// access
fmt.Println(pizzaRange.range)           // => 200

// since pizzaRange.center is a Location struct
fmt.Println(pizzaRange.center.latitude) // => 44.9871

// re-assign
pizzaRange.range = 250

// ...

// golang does not have classes. however, you can define methods on types.
// a method is a function with a special receiver argument.
// the receiver appears in its own argument list between the func keyword and the method name.

// we import the math package because we are going to use pow and sqrt
import "math"

type DeliveryRange struct {
	range float64
	center Location
}

// (this DeliveryRange) is the receiver
func (this DeliveryRange) isInRange(customer: Location) bool {
	var difference float64 = Sqrt(Pow((this.customer.latitude - this.center.latitude), 2) +
		Pow((this.customer.longitude - this.center.longitude), 2))

	return difference < this.range
}

// instantiate
var ranger DeliveryRange = DeliveryRange{150, Location{"44.9871,-93.2758"}}
var customer Location = Location{"44.9850,-93.2750"}
 
// call method
ranger.isInRange(customer) // => true!


// you could also declare methods with pointer receivers.
// which allows you to modify the value to which the receiver points

/**
 * @examples
 */

type Vertex struct {
	X, Y float64
}

// cannot modify X and Y
func (v Vertex) Abs() float64 {
	return math.Sqrt(v.X*v.X + v.Y*v.Y)
}

// can modify X and Y
func (v *Vertex) Scale(f float64) {
	v.X = v.X * f
	v.Y = v.Y * f
}
