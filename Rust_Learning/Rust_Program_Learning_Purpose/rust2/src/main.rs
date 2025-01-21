fn main() {
    println!("{}",fibo(10));
}

fn fibo(num : u32) -> u32 {
    let mut first = 0;
    let mut second = 1;

    if num==0 {
        return first;   
    }

    if num==1 {
        return second;
    }
    // 1...num

    for _ in 0..(num-1) {
        let temp = second;
        second = first + second;
        first = temp;
    }

    return second;
}