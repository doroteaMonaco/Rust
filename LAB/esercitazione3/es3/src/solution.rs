#[derive(Debug, Clone, PartialEq)]
pub struct CircularBuffer<T> where T : Default {
    pub buffer: Vec<T>,
    pub flags: Vec<bool>,
    pub head: usize,
    pub tail: usize,
    pub size: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CircularBufferError{
    BufferFull,
}

impl <T : Default + Copy> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let mut f = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            f.push(false); // Use push to initialize the flags
        }
        let mut b = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            b.push(T::default()); // Use push to initialize the buffer
        }
        CircularBuffer {
            buffer: b,
            flags: f,
            head: 0,
            tail: 0,
            size: capacity,
        }
    }

    pub fn write(&mut self, item: T) -> Result<(), CircularBufferError> {
        if self.tail == self.head && self.flags[self.tail] == true { // buffer pieno
            return Err(CircularBufferError::BufferFull);
        } else {
            self.buffer[self.tail] = item;
            self.flags[self.tail] = true;
            self.tail = (self.tail + 1) % self.size;
            return Ok(());
        }
    }

    pub fn read(&mut self) -> Option<T> { //pop
        if self.flags[self.head] == false {
            return None;
        }
        else{
            let item = self.buffer[self.head];
            self.flags[self.head] = false;
            self.head = (self.head + 1) % self.size;
            return Some(item);
        }
    }

    pub fn clear(&mut self) {
        for i in 0..self.size {
            self.flags[i] = false;
        }
        self.head = 0;
        self.tail = 0;
    }

    pub fn size(&self) -> usize {
        let mut countElements = 0;
        for i in 0..self.size {
            if self.flags[i] == true {
                countElements += 1;
            }
        }
        return countElements;
    }

    // può essere usata quando il buffer è pieno per forzare una
    // scrittura riscrivendo l’elemento più vecchio
    pub fn overwrite(&mut self, item: T) {
        if self.tail == self.head && self.size() == 0 { // buffer vuoto
            self.write(item);
        }
        else { // buffer pieno
            self.buffer[self.head] = item;
            self.flags[self.head] = true;
            self.head = (self.head + 1) % self.size; 
        }
    }

    // vedi sotto*
    pub fn make_contiguous(&mut self) {
        let mut new_buffer = Vec::with_capacity(self.size);
        let mut new_flags = Vec::with_capacity(self.size);

        for _ in 0..self.size {
            new_buffer.push(T::default());
            new_flags.push(false);
        }

        let mut j = 0;
        for i in 0..self.size {
            if self.flags[i] == true {
                new_buffer[j] = self.buffer[i]; // Replace the element at index j
                new_flags[j] = true;           // Mark the flag as true
                j += 1;
            }
        }

        self.buffer = new_buffer;
        self.flags = new_flags;
        self.head = 0;
        self.tail = j;
    }
}

impl<T: Default + Copy> Index<usize> for CircularBuffer<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.size() {
            panic!("Index out of bounds!");
        }
        let real_index = (self.head + index) % self.size; // Calcola l'indice reale
        &self.buffer[real_index]
    }
}

impl<T: Default + Copy> IndexMut<usize> for CircularBuffer<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.size() {
            panic!("Index out of bounds!");
        }
        let real_index = (self.head + index) % self.size; // Calcola l'indice reale
        &mut self.buffer[real_index]
    }
}

impl <T: Default + Copy> Deref for CircularBuffer<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        if self.head == self.tail && self.size() == 0 { // buffer vuoto
            return &self.buffer;
        } else if self.head == self.tail && self.size() > 0 { // buffer pieno
            panic!("Buffer is not contiguous.");
        } else {
            let start = self.head;
            let end = (self.tail + self.size) % self.size;
            return &self.buffer[start..end];
        }
    }
}

impl <T: Default + Copy> TryDeref for CircularBuffer<T> {
    type Error = CircularBufferError;

    fn try_deref(&self) -> Result<&Self::Target, Self::Error> {
        if self.head == self.tail && self.size() == 0 { // buffer vuoto
            return Ok(&self.buffer);
        } else if self.head == self.tail && self.size() > 0 { // buffer pieno
            return Err(CircularBufferError::BufferFull);
        } else {
            let start = self.head;
            let end = (self.tail + self.size) % self.size;
            return Ok(&self.buffer[start..end]);
        }
    }
}

impl <T: Default + Copy> DerefMut for CircularBuffer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.head == self.tail && self.size() == 0 { // buffer vuoto
            return &mut self.buffer;
        } else if self.head == self.tail && self.size() > 0 { // buffer pieno
            panic!("Buffer is not contiguous.");
        } else {
            let start = self.head;
            let end = (self.tail + self.size) % self.size;
            return &mut self.buffer[start..end];
        }
    }
}