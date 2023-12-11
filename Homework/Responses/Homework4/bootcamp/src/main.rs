// Write a 'fizz buzz' function that will be called from
// your main function.
// 1. The function should have a loop counting up to 301
// 2. If the count is divisible by 3, print "fizz"
// 3. If the count is divisible by 5 print "buzz"
// 4. If the count is divisible by 3 and 5 print "fizz buzz"
// 5. At the end print the number of times "fizz buzz"
// occurred.
 
fn fiz_buzz() {
    let mut count = 0;
    let mut fizz_buzz_count = 0;
    while count <= 301 {
        if count % 3 == 0 && count % 5 == 0 {
            fizz_buzz_count += 1;
            println!("fizz buzz count: {}", fizz_buzz_count);
        } else if count % 3 == 0 {
            println!("fizz");
        } else if count % 5 == 0 {
            println!("buzz");
        }
        count += 1;
    }
    let duration = current_time_stamp.elapsed();
    println!("fizz buzz count: {}", fizz_buzz_count); //fizz buzz count: 21
}

fn main() {
    fiz_buzz();
}
