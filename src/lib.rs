
mod mods;
#[cfg(test)]
mod tests {
    use crate::mods::studentinfo::Student;
    // 注意这个惯用法：在 tests 模块中，从外部作用域导入所有名字。
    use super::*;

    #[test]
    fn test_add() {
        let s=Student::new(1,"jack".to_string(), "address".to_string());
        assert_eq!(s.id, 2);
    }

    #[test]
    fn test_bad_add() {
        // 这个断言会导致测试失败。注意私有的函数也可以被测试！
        assert_eq!(1, 3);
    }
}