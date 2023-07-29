use std::rc::Rc;

fn main() {
    Client::execute_request(DefaultTarget);
    let new_1 = Rc::new(NewTargetFunctionallity::new("hehehe".to_string()));
    let adapter = Adapter::new(new_1);
    Client::execute_request(adapter);
}

trait TargetAction {
    fn get_request(&self) -> String {
        String::from("default string")
    }
}
//default
struct DefaultTarget;

impl TargetAction for DefaultTarget {}

struct NewTargetFunctionallity {
    data: String,
}

impl NewTargetFunctionallity {
    fn new(s: String) -> Self {
        Self { data: s }
    }
    fn specitic_request(&self) -> String {
        format!("specific request {}", self.data)
    }
    fn backgrund_work(&self) {
        println!("doing something in background");
    }
}

struct Adapter {
    to_adapt: Rc<NewTargetFunctionallity>,
}

impl Adapter {
    fn new(a: Rc<NewTargetFunctionallity>) -> Self {
        Self { to_adapt: a }
    }
}

impl TargetAction for Adapter {
    fn get_request(&self) -> String {
        self.to_adapt.backgrund_work();
        self.to_adapt.specitic_request()
    }
}

struct Client;
impl Client {
    fn execute_request<T: TargetAction>(arg: T) {
        println!("client executes: {}", arg.get_request());
    }
}
