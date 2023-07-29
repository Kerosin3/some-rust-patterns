fn main() {
    let factory = ProviderFactory::new_provider(&Providers::Provider1);
    factory.provide();
    let factory = ProviderFactory::new_provider(&Providers::Provider2);
    factory.provide();
}

trait Provider {
    fn provide(&self);
}

// supply to factory
enum Providers {
    Provider1,
    Provider2,
    Provider3,
}

struct Provider1 {}

impl Provider for Provider1 {
    fn provide(&self) {
        println!("provider 1 was called!");
    }
}
struct Provider2 {}

impl Provider for Provider2 {
    fn provide(&self) {
        println!("provider 2 was called!");
    }
}
struct Provider3 {}

impl Provider for Provider3 {
    fn provide(&self) {
        println!("provider 3 was called!");
    }
}

struct ProviderFactory;
impl ProviderFactory {
    fn new_provider(x: &Providers) -> Box<dyn Provider> {
        match x {
            Providers::Provider1 => Box::new(Provider1 {}),
            Providers::Provider2 => Box::new(Provider2 {}),
            Providers::Provider3 => Box::new(Provider3 {}),
        }
    }
}
