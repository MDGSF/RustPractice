use std::time::SystemTime;

fn sum_of_n(n: i64) -> i64 {
    let mut sum: i64 = 0;
    for i in 0..=n {
        sum += i;
    }
    sum
}

fn sum_of_n1(n: i64) -> i64 {
    (1..=n).sum::<i64>()
}

fn sum_of_n2(n: i64) -> i64 {
    n * (n + 1) / 2
}

fn main() {
    for _i in 0..5 {
        let now = SystemTime::now();
        // let _sum = sum_of_n(500000);
        let _sum = sum_of_n1(500000);
        //let _sum = sum_of_n2(500000);
        let duration = now.elapsed().unwrap();
        let time = duration.as_millis();
        println!("func used {time} ms");
    }
}
