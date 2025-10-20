/*
Rust Variables and Type Annotations Exercise

1. Declare a `season` variable set to a string with your favorite season. 
   Provide an explicit type annotation. The type of a string is a `&str`. 
   We'll discuss what the & symbol means later in the course.

2. Declare a `points_scored` variable set to 28. Provide an explicit type 
   annotation. The type of an integer is `i32`.

3. It's time to update the team's score. Declare the `points_scored` variable 
   to be mutable. Set its new value to 35.

4. Declare a `TOUCHDOWN_POINTS` constant at the file level set to the value 6.

5. Declare a `event_time` variable set to a string of "06:00".

6. Use variable shadowing to redeclare `event_time` set to an integer of 6.

7. Use interpolation to print out all of the declared variables and constants 
   in a println! call. Practice with direct interpolation, sequential arguments, 
   and numeric arguments.

8. Declare a `favorite_beverage` variable set to a string of your favorite drink. 
   Use an underscore to silence the compiler warning about the variable being unused.
*/

fn main(){
    let _season : &str = "Autumn";
    let mut points_scored : i32 = 28;
    points_scored = 35;
    const _TOUCHDOWN_POINTS: i32 = 6;
    let _event_time : &str = "06:00";
    let _event_time : i32 = 6;
    println!("Season: {}, Points Scored: {}, Touchdown Points: {}, Event Time: {} or {}", 
             _season, points_scored, _TOUCHDOWN_POINTS, _event_time, _event_time);
    let _favorite_beverage : &str = "Coffee";
}