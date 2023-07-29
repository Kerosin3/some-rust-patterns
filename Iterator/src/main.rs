fn main() {
    let mut c = Container::new();
    c.add_item(1);
    c.add_item(2);
    c.add_item(3);
    c.add_item(4);

    let mut iter = c.iter();
    while iter.has_next() {
        println!("item is {}", iter.next().unwrap());
    }
}

trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
    fn current(&self) -> Option<T>;
    fn has_next(&self) -> bool;
    fn reset(&mut self);
}

struct Container<T> {
    data: Vec<T>,
}

impl<T: Clone> Container<T> {
    fn new() -> Container<T> {
        Self { data: Vec::new() }
    }
    fn add_item(&mut self, item: T) {
        self.data.push(item);
    }
    fn iter(&self) -> impl Iterator<T> + '_ {
        IteratorItself::new(self)
    }
}

struct IteratorItself<'a, T> {
    index: usize,
    container: &'a Container<T>,
}

impl<'a, T: Clone> IteratorItself<'a, T> {
    fn new(container: &'a Container<T>) -> IteratorItself<T> {
        Self {
            index: 0,
            container: container,
        }
    }
}

impl<'a, T: Clone> Iterator<T> for IteratorItself<'a, T> {
    fn next(&mut self) -> Option<T> {
        let next_item = self.container.data.get(self.index).cloned(); //get current
        self.index += 1; // increment
        next_item
    }

    fn current(&self) -> Option<T> {
        self.container.data.get(self.index).cloned()
    }
    // if has next item to iterate
    fn has_next(&self) -> bool {
        self.container.data.len() > self.index
    }

    fn reset(&mut self) {
        self.index = 0;
    }
}
