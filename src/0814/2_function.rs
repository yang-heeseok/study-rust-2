fn main() {
    call_me();

    call_me_2(3);
    
    call_me_3(5);

    let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price));

    let answer = square(3);
    println!("The square of 3 is {answer}");
}

fn call_me(){}

fn call_me_2(num: u64) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn call_me_3(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn is_even(num: i64) -> bool {
    num % 2 == 0
}

fn sale_price(price: i64) -> i64{
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

fn square(num: i32) -> i32 {
    num * num
}