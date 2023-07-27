use lib_example::machinery::*;
use lib_example::mashinaries::*;
use lib_example::*;
fn main() {
    let mut mach_1 = MachineX::new(Box::new(Action1));
    mach_1.act();
    mach_1.set_beh(Box::new(Action2));
    mach_1.act();
}
