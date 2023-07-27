fn main() {
    let mut subject = Subject::new();
    let obs1 = ConcreteObserver { id: 1 };
    let obs2 = ConcreteObserver { id: 2 };
    let obs3 = ConcreteObserver { id: 3 };
    subject.attach(&obs1);
    subject.attach(&obs2);
    subject.attach(&obs3);
    subject.notify_observer();
    subject.detach(&obs2);
    subject.notify_observer();
}

trait IObserver {
    fn update(&self);
}

trait ISubject<'a, T: IObserver> {
    fn attach(&mut self, obs: &'a T);
    fn detach(&mut self, obs: &'a T);
    fn notify_observer(&self);
}

struct Subject<'a, T: IObserver> {
    observers: Vec<&'a T>,
}
impl<'a, T: IObserver + PartialEq> Subject<'a, T> {
    fn new() -> Self {
        Self {
            observers: Vec::new(),
        }
    }
}

impl<'a, T: IObserver + PartialEq> ISubject<'a, T> for Subject<'a, T> {
    fn attach(&mut self, obs: &'a T) {
        // push obj implements IObserver
        self.observers.push(obs);
    }

    fn detach(&mut self, obs: &'a T) {
        if let Some(idx) = self.observers.iter().position(|x| *x == obs) {
            self.observers.remove(idx);
        }
    }

    fn notify_observer(&self) {
        for item in self.observers.iter() {
            item.update();
        }
    }
}
#[derive(PartialEq)]
struct ConcreteObserver {
    id: i32,
}
impl IObserver for ConcreteObserver {
    fn update(&self) {
        println!("Observer id:{} received event!", self.id);
    }
}
