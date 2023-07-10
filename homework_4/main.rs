struct MyVector<T> {
    data: [Option<T>; 16],
    length: usize,
    capacity: usize,
}

impl<T> MyVector<T> {
    fn new() -> Self {
        Self {
            data: [None; 16],
            length: 0,
            capacity: 16,
        }
    }

    fn with_capacity(capacity: usize) -> Self {
        Self {
            data: [None; capacity],
            length: 0,
            capacity,
        }
    }

    fn push(&mut self, element: T) {
        if self.length == self.capacity {
            self.resize(self.capacity * 2);
        }
        self.data[self.length] = Some(element);
        self.length += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if self.length == 0 {
            return None;
        }
        let index = self.length - 1;
        let element = self.data[index].take();
        self.length -= 1;
        if self.length < self.capacity / 4 {
            self.resize(self.capacity / 2);
        }
        element
    }

    fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.length {
            return None;
        }
        let element = self.data[index].take();
        for i in index..self.length - 1 {
            self.data[i] = self.data[i + 1].take();
        }
        self.length -= 1;
        if self.length < self.capacity / 4 {
            self.resize(self.capacity / 2);
        }
        element
    }

    fn get(&self, index: usize) -> Option<&T> {
        if index >= self.length {
            return None;
        }
        self.data[index].as_ref()
    }

    fn resize(&mut self, new_capacity: usize) {
        let mut new_data = [None; 16];
        for i in 0..self.length {
            new_data[i] = self.data[i].take();
        }
        self.data = new_data;
        self.capacity = new_capacity;
    }
}