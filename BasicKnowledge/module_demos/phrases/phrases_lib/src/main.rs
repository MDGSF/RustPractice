/*
Rust的模块文件系统规则如下。
1）如果foo模块没有子模块，将foo模块的代码放在foo.rs文件中。
2）如果foo模块有子模块，有两种处理方式。
将foo模块的代码放在foo.rs文件中，并将其子模块所在文件存放在foo/文件夹，推荐采用这种方式。
将foo模块的代码放在foo/mod.rs文件中，并将其子模块所在文件存放在foo/文件夹。


 Rust遵循以下规则在文件系统中寻找多层模块。
1）main.rs、lib.rs和mod.rs文件中出现“mod xxx;”，默认优先寻找同级文件夹下的xxx.rs文件。
2）如果同级文件夹下的xxx.rs文件不存在，则寻找xxx/mod.rs文件，
  即与xxx.rs文件所在同级文件夹的xxx文件夹下的mod.rs文件。
3）如果其他文件如yyy.rs文件中出现“mod xxx;”，默认寻找yyy/xxx.rs文件，
  即与yyy.rs文件所在同级文件夹的yyy文件夹下的xxx.rs文件。


Rust提供了use和as两个关键字，使用use可以将指定的模块或函数引入本地作用域，
但不会将其子模块也引入；使用as可以对模块或函数进行重命名。


导入函数容易引起命名冲突。因此，use的最佳实践是导入模块而不是直接导入函数。


crate关键字表示当前crate，crate::a::b::c表示从根模块开始的绝对路径。
self关键字表示当前模块，self::a表示当前模块中的子模块a。
  self关键字最常用的场景是“use a::{self, b};”，表示导入当前模块a及其子模块b。
super关键字代表当前模块的父模块，super::a表示当前模块的父模块中的子模块a。


pub use 可以把深层的模块、函数导入到当前模块。
*/

//use phrases_lib::chinese::farewells as cn_farewells;
//use phrases_lib::chinese::greetings as cn_greetings;
use phrases_lib::chinese;
use phrases_lib::english::{farewells, greetings};

fn main() {
  //println!("Hello in chinese: {}", cn_greetings::hello());
  //println!("Goodbye in chinese: {}", cn_farewells::goodbye());

  println!("Hello in chinese: {}", chinese::hello());
  println!("Goodbye in chinese: {}", chinese::goodbye());
  println!("Hello in English: {}", greetings::hello());
  println!("Goodbye in English: {}", farewells::goodbye());
}
