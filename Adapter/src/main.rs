use std::rc::Rc;

fn main() {
    Client::func1(&DefaultTarget);
    let t1 = Rc::new(Someusefullbehaviour::new(String::from("some new request")));
    let adapter = Adapter::new(t1);
    Client::func1(&adapter);
}

trait TargetAction {
    fn get_request(&self) -> String {
        String::from("default string")
    }
}
//default
struct DefaultTarget;

impl TargetAction for DefaultTarget {}

struct Someusefullbehaviour {
    data: String,
}
// pass this behaviour
impl Someusefullbehaviour {
    fn new(s: String) -> Self {
        Self { data: s }
    }
    fn specific_request(&self) -> String {
        format!("some specific request {}", self.data)
    }
}

struct Adapter {
    connector: Rc<Someusefullbehaviour>,
}
impl Adapter {
    fn new(a: Rc<Someusefullbehaviour>) -> Self {
        Self { connector: a }
    }
}

impl TargetAction for Adapter {
    fn get_request(&self) -> String {
        // new behaviour
        format!("Adapter beh: {}", self.connector.specific_request())
    }
}

struct Client;
impl Client {
    fn func1<T: TargetAction>(target: &T) {
        println!("code runs: {}", target.get_request());
    }
}
