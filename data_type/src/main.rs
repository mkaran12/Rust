fn main() {
    //method is a function that lives on a value. It's an action we can ask the value to execute.
    //value.method()
//     let value: i32 = -15;
//     println!("{}", value.abs());

//     let empty_space: &str = "     my content     ";
//     println!("{}", empty_space.trim());

//     println!("{}", value.pow(2));
//      println!("{}", value.pow(3));
//
     let eight_bit_unsigned: u8 = 255;
     println!("8 bit unsigned integer: {}", eight_bit_unsigned);
/*
data types is statically typed language -> type of every variable must be known at compile time.
scalar typers - > sype that hold a single value
4 primary scalar types:
1. integers
     categories of integers:
        a. signed - can represent negative and positive numbers symbol i
        b. unsigned - can represent only positive numbers symbol u
2. floating-point numbers
3. booleans
4. characters
a bit is the smalletst unit of data in a computer and can have a value of either 0 or 1.
a byte is made up of 8 bits and can represent a wider range of values.

8bits is equal to 1 byte

type f32  precison is 6-9 digits of precision
type f64  precision is 15-17 digits of precision

type    smallest              largest
i8      -128                  127
u8      0                     255
i16     -32,768               32,767
u16     0                     65,535
i32     -2,147,483,648        2,147,483,647
u32     0                     4,294,967,295
i64     -9,223,372,036,854,775,808         9,223,372,036,854,775,807
u64     0                     18,446,744,073,709,551,615   
i128    -170,141,183,460,469,231,731,687,303,715,884,105,728   170,141,183,460,469,231,731,687,303,715,884,105,727
u128    0                     340,282,366,920,938,463,463,374,607,431,768,211,455
isize   depends on architecture

//declare data type after variable name
let some_values =20u32;
*/
 let some_values = 20u32;
}