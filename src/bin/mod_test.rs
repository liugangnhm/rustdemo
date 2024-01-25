fn main() {
    mod1::mod1_pub_f2();
}

mod mod1 {

    fn mod1_pri_f1() {}
    pub fn mod1_pub_f2() {}

    fn test1() {
        mod2::mod2_pub_f2();
        // mod2::mod2_pri_f2();
    }

    mod mod2 {

        fn mod2_pri_f1() {}
        pub fn mod2_pub_f2() {}

        fn test1() {
            super::mod1_pub_f2();
            super::mod1_pri_f1();
        }
    }
}
