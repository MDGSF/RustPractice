/*
 * https://cprimozic.net/blog/writing-a-hashmap-to-struct-procedural-macro-in-rust/
 * https://github.com/dtolnay/syn
 */

fn test1() {
    let v1 = vec![1, 2, 3];
    println!("v1 = {:?}", v1);
}

macro_rules! myvec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    }
}

fn test2() {
    let v2 = myvec![4, 5, 6];
    println!("v2 = {:?}", v2);
}

macro_rules! foo {
    (x => $e:expr) => {
        println!("mode X: {}", $e);
    };
    (y => $e:expr) => {
        println!("mode Y: {}", $e);
    };
}

fn test3() {
    foo!(x => "aaa");
}

macro_rules! o_O {
    (
        $(
            $x:expr; [ $( $y:expr ),* ]
        );*
    ) => {
        &[ $($($x+$y),*),* ]
    }
}

fn test4() {
    let v3: &[i32] = o_O!(
        10; [1,2,3];
        20; [4,5,6]
    );
    println!("v3 = {:?}", v3);
}

macro_rules! five_times {
    ($x:expr) => {
        5 * $x
    };
}

fn test5() {
    println!("five_times!(2 + 3) = {}", five_times!(2 + 3));
}

macro_rules! min {
    (
        $x:expr, $y:expr
    ) => {
        if $x < $y {
            $x
        } else {
            $y
        }
    };
}

fn test6() {
    println!("min(1,2) = {}", min!(1, 2));
}

macro_rules! log {
    (
        $msg:expr
    ) => {{
        let state: i32 = 1; //get_log_state();
        if state > 0 {
            println!("log({}): {}", state, $msg);
        }
    }};
}

fn test7() {
    let state: &str = "reticulating splines";
    log!(state);
}

macro_rules! fooitem {
    () => {
        fn x() {
            println!("I'm x function in fooitem macro");
        }
    };
}

fn test8() {
    fooitem!();
    x();
}

fn main() {
    test1();
    test2();
    test3();
    test4();
    test5();
    test6();
    test7();
    test8();
}
