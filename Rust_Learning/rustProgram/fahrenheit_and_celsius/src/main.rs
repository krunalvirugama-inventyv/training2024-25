// (1)Convert temperatures between Fahrenheit and Celsius.

// fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
//     (fahrenheit - 32.0) * 5.0 / 9.0
// }

// fn celsius_to_fahrenheit(celsius: f64) -> f64 {
//     (celsius * 9.0 / 5.0) + 32.0
// }

// fn main() {
//     let fahrenheit = 100.0;
//     let celsius = fahrenheit_to_celsius(fahrenheit);
//     println!("{fahrenheit}°F is equal to {celsius:.2}°C");

//     let celsius = 37.0;
//     let fahrenheit = celsius_to_fahrenheit(celsius);
//     println!("{celsius}°C is equal to {fahrenheit:.2}°F");
// }


// (2)Generate the nth Fibonacci number.

// fn fibonacci_recursive(n: u32) -> u32 {
//     if n <= 1 {
//         n
//     } else {
//         fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
//     }
// }

// fn main() {
//     let n = 10; // Example: Find the 10th Fibonacci number
//     let result = fibonacci_recursive(n);
//     println!("The {}th Fibonacci number is: {}", n, result);
// }


// (3) Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song. in rust

fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", 
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"
    ];

    let gifts = [
        "a Partridge in a Pear Tree.",
        "two Turtle Doves, and",
        "three French Hens,",
        "four Calling Birds,",
        "five Golden Rings,",
        "six Geese a-Laying,",
        "seven Swans a-Swimming,",
        "eight Maids a-Milking,",
        "nine Ladies Dancing,",
        "ten Lords a-Leaping,",
        "eleven Pipers Piping,",
        "twelve Drummers Drumming,"
    ];

    for day in 0..12 {
        println!(
            "On the {} day of Christmas, my true love gave to me:",
            days[day]
        );

        for gift in (0..day).rev() {
            println!("{}", gifts[gift]);
        }

       
    }
}
