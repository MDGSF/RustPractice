pub mod farewells;
pub mod greetings;

// 把goodbye导入到当前路径，外层就可以使用chinese::goodbye了。
pub use self::farewells::goodbye;

// 把hello导入到当前路径，外层就可以使用chinese::hello了。
pub use self::greetings::hello;
