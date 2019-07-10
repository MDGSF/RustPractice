/*
&i32         // a reference
&'a i32      // a reference with an explicit lifetime
&'a mut i32  // a mutable reference with an explicit lifetime
*/

/*
lifetime annotation 是用来标记函数的输入参数和返回值之间的关系的。
如果输入参数和返回值之间没有借用关系，则不需要加 lifetime annotation。

longest 这个函数应用了所有的规则，都无法确定所有的生命周期，所以需要手动标注生命周期。
*/
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/*
longest2 的返回值是一份拷贝，和输入参数没有关系，所有不用加 lifetime annotation。
*/
fn longest2(x: &str, y: &str) -> String {
    if x.len() > y.len() {
        x.to_string().clone()
    } else {
        y.to_string().clone()
    }
}

fn test1() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let result = longest2(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // announce_and_return_part 这个函数应用规则 1 和规则 3 可以确定所有的生命周期。
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn test2() {
    let noval = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = noval.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    /*
    变量 i 中的 part 租借了 first_sentence，所以变量 i 的生命周期不能超过 first_sentence
    */
}

/*
first_word1 和 first_word2 都是合法的，因为 rust 的编译器能够自动推断出这种情况。
first_word1 这个函数应用规则 1 和规则 2 可以确定所有的生命周期。
*/
fn first_word1(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word2<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

/*
lifetime elision rule
当出现 lifetime elision rule 的情况时，rust 编译器就很可能能够自动推断出
函数的输入参数和返回值之间的生命周期关系。
函数参数中的生命周期叫做 input lifetimes
函数返回值中的生命周期叫做 output lifetimes

规则 1：函数的每个输入参数都拥有自己的生命周期标记。
    例如： fn foo<'a>(x: &'a i32);
    例如： fn foo<'a, 'b>(x: &'a i32, y: &'b i32);

规则 2：如果函数只有一个 input lifetime annotation ，那么把这个 input lifetime annotation 应用
    到所有的输出参数上。
    例如： fn foo<'a>(x: &'a i32) -> &'a i32;

规则 3：如果函数的多个参数中有一个是 &self 或者是 &mut self，那么把这个参数的
    input lifetime annotation 应用到所有的输出参数上。


注意：
    1. 只有租借的类型需要判断生命周期。
    2. 通过这 3 个规则检查之后，如果还无法确定所有的生命周期，rust 会报错，要求开发人员手动标注生命周期。

*/

fn main() {
    test1();
    test2();
}
