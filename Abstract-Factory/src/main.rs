fn main() {
    let t1 = Carrier::new_carrier(&ModuleTypes::XModule);
    t1.connect().initialize_connection();
    t1.close().close_connection();
    let t2 = Carrier::new_carrier(&ModuleTypes::YModule);
    t2.connect().initialize_connection();
    t2.close().close_connection();
}

trait DriverFactory {
    fn connect(&self) -> Box<dyn IConnector>;
    fn close(&self) -> Box<dyn ICloser>;
}

enum ModuleTypes {
    XModule,
    YModule,
}

struct XFactory;
struct YFactory;

impl DriverFactory for XFactory {
    fn connect(&self) -> Box<dyn IConnector> {
        Box::new(XConnector)
    }

    fn close(&self) -> Box<dyn ICloser> {
        Box::new(XConnector)
    }
}
impl DriverFactory for YFactory {
    fn connect(&self) -> Box<dyn IConnector> {
        Box::new(YConnector)
    }

    fn close(&self) -> Box<dyn ICloser> {
        Box::new(YConnector)
    }
}

trait IConnector {
    fn initialize_connection(&self);
}
trait ICloser {
    fn close_connection(&self);
}
// X connector
struct XConnector;
impl IConnector for XConnector {
    fn initialize_connection(&self) {
        println!("connection to X connector");
    }
}

impl ICloser for XConnector {
    fn close_connection(&self) {
        println!("closing X connection");
    }
}

// Y connector
struct YConnector;
impl IConnector for YConnector {
    fn initialize_connection(&self) {
        println!("connection to Y connector");
    }
}

impl ICloser for YConnector {
    fn close_connection(&self) {
        println!("closing Y connection");
    }
}

struct Carrier;
impl Carrier {
    fn new_carrier(m: &ModuleTypes) -> Box<dyn DriverFactory> {
        match m {
            ModuleTypes::XModule => Box::new(XFactory),
            ModuleTypes::YModule => Box::new(YFactory),
        }
    }
}
