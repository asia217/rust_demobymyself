
// `pub(crate)` 使得函数只在当前 crate 中可见
pub(crate) fn function() {
    println!("called `function()`");
}
fn private_function() {
    println!("called `my_mod::private_function()`");
}
   // 在同一模块中，项可以访问其它项，即使它是私有的。
 pub fn indirect_access() {
    print!("called `my_mod::indirect_access()`, that\n> ");
    private_function();
}

//mod嵌套mod
pub mod nested {
    pub fn function() {
        println!("called `my_mod::nested::function()`");
    }
}

//mod嵌套mod
pub mod nested2 {
    pub fn function() {
        println!("called `my_mod::nested::function()`");
    }
}
