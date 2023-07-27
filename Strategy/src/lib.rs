// select algorithm of behaviour in runtime
pub trait SomeAction {
    fn perform(&self);
}

pub struct Action1;

impl SomeAction for Action1 {
    fn perform(&self) {
        println!("Action 1 has been performed!");
    }
}
pub struct Action2;

impl SomeAction for Action2 {
    fn perform(&self) {
        println!("Action 2 has been performed!");
    }
}

pub mod machinery {
    use super::SomeAction;
    pub trait Machinery {
        fn get_some_action(&self) -> &dyn SomeAction;
        fn act(&self) {
            self.get_some_action().perform(); // perform cation
        }
    }
}

pub mod mashinaries {
    use super::machinery::Machinery;
    use super::SomeAction;
    pub struct MachineX {
        beh: Box<dyn SomeAction>,
    }
    impl MachineX {
        pub fn new(beh_passed: Box<dyn SomeAction>) -> Self {
            Self { beh: beh_passed }
        }
        pub fn set_beh(&mut self, beh: Box<dyn SomeAction>) {
            self.beh = beh;
        }
    }
    impl Machinery for MachineX {
        fn get_some_action(&self) -> &dyn SomeAction {
            &(*self.beh)
        }
    }

    pub struct MachineY {
        beh: Box<dyn SomeAction>,
    }

    impl Machinery for MachineY {
        fn get_some_action(&self) -> &dyn SomeAction {
            &(*self.beh)
        }
    }
    impl MachineY {
        pub fn new(beh_passed: Box<dyn SomeAction>) -> Self {
            Self { beh: beh_passed }
        }
        pub fn set_beh(&mut self, beh: Box<dyn SomeAction>) {
            self.beh = beh;
        }
    }
}
