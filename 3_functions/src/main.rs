fn main() {
    // function return do not need explicit return and SHOULD NOT have ; at the end
    #[allow(unused)]
    fn return_five() -> i32 {
        5
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // return this result after breaking loop
        }
    };
    println!("Counter is {result}");

    /*
     * We can use ' in order to name a loop and call it to be broken from an inner loop
     */
    #[allow(unused_mut)]
    let mut count = 0;
    'counting_up: loop {
        println!("Count: {count}");
        let mut remaining = 10;

        loop {
            println!("reamining = {remaining}");
            if remaining == 0 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1
        }

        count += 1;
    }

    println!("End count = {count}");

    // Reverse range from [1, 4)
    for number in (1..4).rev() {
        println!("Number: {number}");
    }

    fn get_fib(n: usize) -> usize {
        if n == 0 {
            0
        } else if n == 1 { 
            1
        } else {
            get_fib(n - 1) + get_fib(n - 2)
        }
    }

    let mut fib = String::new();

    println!("Enter which fib number you want!");
    std::io::stdin()
        .read_line(&mut fib)
        .expect("Cannot read line!");

    let fib = fib.trim()
        .parse()
        .expect("Not a number!");
    let fib_num = get_fib(fib);

    println!("{fib}th fib number is {fib_num}");

}
