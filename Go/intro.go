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
var any interface{}   = nil
var fn func() int     = func () int { return 0 }
var arr [2]string     = ["String 1", "String 2"]
var char rune         = '⌘' // a rune with integer value 0x2318, uses single ''tick
var bytes []byte      = ["H", "e", 'l', 'l', 'o']


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
 * @conditionals
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


str = append(str, "D")                 // str => ["A", "B", "C", "D"]

str = append(str, 0)
copy(str[1+1:], s[1:])
s[1] = "F"                             // str => ["A", "F", "B", "C", "D"]

var ints2 []int = {4, 5, 6}

ints = append(ints, ints2...)          // ints => [1, 2, 3, 4, 5, 6]
ints = append(ints, []int{4, 5, 6}...) // ints => [1, 2, 3, 4, 5, 6]

buls = append(buls[:0], buls[0+1:]...) // buls => [false, true]

dbls[2] = 4.0                          // dbls => [1.0, 2.0, 4.0]


/**
 * ---------------------------------------------------------------------------------
 * 
 * @functions
 * 
 * ---------------------------------------------------------------------------------
 */


/**
 * ---------------------------------------------------------------------------------
 * 
 * @dictionaries
 * 
 * ---------------------------------------------------------------------------------
 */


/**
 * ---------------------------------------------------------------------------------
 * 
 * @tuples
 * 
 * ---------------------------------------------------------------------------------
 */


/**
 * ---------------------------------------------------------------------------------
 * 
 * @structs
 * 
 * ---------------------------------------------------------------------------------
 */

