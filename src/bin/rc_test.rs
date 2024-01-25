    use std::rc::Rc;
    fn main() {
        let s = Rc::new(String::from("hello"));
        println! ("{:? }", s.bytes());
    }
