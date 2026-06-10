// fn main() {
//     let condition = false;
//     let number = if condition {5} else {6};
//     println!("The value of number is: {number}")
// }

// fn main() {
//     let mut counter = 0;
//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };
//     println!("The result is {result}")
// }

// fn main() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 10;

//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }
//         count += 1;
//     }
//     println!("End count = {count}");
// }

// most people will use the version below this one instead as its safer and makes more sense
// fn main() {
//     let mut number = 3;

//     while number != 0 {
//         println!("{number}!");
//         number -= 1;
//     }
//     println!("LIFTOFF!!!");
// }

// fn main() {
//     for number in (1..4).rev() {
//         println!("{number}!");
//     }
//     println!("LIFTOFF!");
// }

// Note this is error prone because it relies on knowing "index values"
// fn main () {
//     let a = [10, 20, 30, 40, 50];
//     let mut index = 0;
//     while index < 5 {
//         println!("the value is: {}", a[index]);
//         index += 1;
//     }
// }

// fn main() {
//     let a = [10, 20, 30, 40, 50];

//     for element in a {
//         println!("the value is: {element}")
//     }
// }

fn main() {
    temp_conv_to_celsius(32.0);
    temp_conv_to_fahrenheit(0.0);
    fibonacci(10);
}

fn temp_conv_to_celsius(fahrenheit: f64) {
    let result = (fahrenheit - 32.0) * ( 5.0 / 9.0);
    println!("{result}");
}

fn temp_conv_to_fahrenheit(celsius: f64) {
    let result = (celsius * 1.8) + 32.0;
    println!("{result}")
}

fn fibonacci(x: i32) {
    let mut n1 = 0;
    let mut n2 = 1;
    for _ in 1..x {
        println!("{n1}");
        let next = n1 + n2;
        n1 = n2;
        n2 = next;
    }

}