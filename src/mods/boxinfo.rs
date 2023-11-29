pub mod mybox {
    pub struct Openbox<T>{
        pub content:T
    }

    #[derive(Debug)]
    pub struct Closebox<T>{
        content:T
    }

   impl<T> Closebox<T> {
    pub fn new(s: T) -> Closebox<T> {
         Closebox { content: s }
     }
  }
}