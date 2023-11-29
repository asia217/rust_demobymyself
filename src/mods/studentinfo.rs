use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Student {
    pub id: i32,
    pub name: String,
    pub address: String,
}

    
impl TryFrom<i32> for Student {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value % 2 == 0 {
            true => {
                let student = Student::new(value, "jack".to_string(), "address".to_string());
                Ok(student)
            }
            false => Err(()),
        }
    }
}

impl ToString for Student {
    fn to_string(&self) -> String {
        format!("{}*{}*{}", self.id, self.name, self.address)
    }
}

impl Student {
    //静态方法  静态方法不需要被实例调用
    pub fn new(i: i32, n: String, a: String) -> Student {
        Student {
            id: i,
            name: n,
            address: a,
        }
    }
    //这是一个实例方法（instance method）
    pub fn getname(&self) -> &str {
        &self.name
    }
    //这个方法要求调用者是可变的
    pub fn setid(&mut self,newid:i32) {
        self.id = newid
    }
    //这个方法会 “消耗” 调用者的资源
    pub fn getaddress(self) -> String {
         self.address
    }
}


