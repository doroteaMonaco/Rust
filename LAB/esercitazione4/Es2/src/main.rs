use std::io::BufRead;

pub mod simple_even_iter {
    // (1) let start with a simple iterator adapter for just one type, "i32"
    // see the adapter pattern example in the pdf "Adapter Pattern..."
    struct EvenIter <I>
        where
        I: Iterator<Item = i32> { // <== this is the type of the iterator
        inner: I, // hint: it's a generic type... here we don't care about bounds yet 
    }

    impl<I> EvenIter<I>
    where
        I: Iterator<Item = i32>,
    {
        pub fn new(iter: I) -> Self {
            EvenIter { inner: iter }
        }
    }

    impl<I> Iterator for EvenIter<I>
    where
        I: Iterator<Item = i32>,
    {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            while let Some(n) = self.inner.next() {
                if n % 2 == 0 {
                    return Some(n);
                }
            }
            None
        }
    }

    // if EvenIter works the test will compile and pass
    #[test]
    fn test_simple_even_iter() {
        let v = vec![1, 2, 3, 4, 5];
        // why iter() does not work here?
        let it = EvenIter::new(v.into_iter());
        for i in it {
            println!("i: {}", i);
        }
    }


    
    // (2) now let's add the adapter to all Iterator<Item=i32> (adavanced)
    trait AddEvenIter: Iterator<Item = i32>
    where
        Self: Sized
    {
        // add even() to anyone implementing this trait
        // usage: v.into_iter().even() ....
        fn even(self) -> EvenIter<Self>{
            EvenIter::new(self)
        }
    }

    // (3) add here the generic implemention, you can supply it for all the iterators
    // impl .... ? 

    impl<I> AddEvenIter for I 
    where
        I: Iterator<Item = i32>,
    {
        // add the even() method to all iterators
        fn even(self) -> EvenIter<Self> {
            EvenIter::new(self)
        }
    }

    #[test] 
    fn test_adapter() {
        let v = vec![1,2,3,4,5];
        for i in v.into_iter().even() {
            println!("{}", i);
        }
    }
}


pub mod even_iter {
    // (4) more adavanced: implement for all integer types 
    // => install the external crate "num" to have some Traits identifying all number types
    use num;

    // the generic parameters I and U are already defined for you in the struct deinition
    // (5) write in a comment in plain english the meaning of the generic parameters 
    // and their constraints
    struct EvenIter<I, U> 
        where 
        I: Iterator<Item = U> {
        iter: I
    }

    impl<I,U> Iterator for EvenIter<I, U> 
        where 
        U: num::Integer + Copy, 
        I: Iterator<Item = U> {
        type Item = U;

        fn next(&mut self) -> Option<Self::Item> {
            while let Some(n) = self.iter.next() {
                if n.is_even() {
                    return Some(n);
                }
            }
            None
        }
        
    }

    // (6) once implemented, the test will compile and pass
    #[test]
    fn test_even_iter() {
        let v: Vec<u64> = vec![1, 2, 3, 4, 5];
        let it = EvenIter { iter: v.into_iter() };
        for i in it {
            println!("i: {}", i);
        }
    }

}


// finally let's implement the grep command
// (1) install the "walkdir" crate for walking over directories using an iterator
// install also the "regex" crate for regular expressions

use walkdir;

// (2) define the match result
struct Match {
    file: String, 
    line: usize,
    text: String
}

// (3) test walkdir iterator, see how errors are handled
#[test]
fn test_walk_dir() {
    let wdir = walkdir::WalkDir::new("/tmp");
    for entry in wdir.into_iter() {
        // print the name of the file or an error message
        match entry {
            Ok(e) => println!("File: {}", e.path().display()),
            Err(e) => println!("Error: {}", e),
        }
    }
} 


// (3) define the grep adapter for the iterator
// add anything you need implement it
struct GrepIter<I>
where
    I: Iterator<Item = Result<walkdir::DirEntry, walkdir::Error>>,
{
    inner: I,
    pattern: regex::Regex,
}

impl<I> GrepIter<I>
where
    I: Iterator<Item = Result<walkdir::DirEntry, walkdir::Error>>,
{
    fn new(iter: I, pattern: regex::Regex) -> Self {
        GrepIter {
            inner: iter,
            pattern,
        }
    }
}

impl <I> Iterator for GrepIter<I>
    where
    I: Iterator<Item = Result<walkdir::DirEntry, walkdir::Error>> {
    
    type Item = Result<Match, walkdir::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(entry) = self.inner.next() {
            match entry {
                Ok(e) => {
                    if e.file_type().is_file() {
                        let file_path = e.path().display().to_string();
                        let file = std::fs::File::open(&file_path).ok()?;
                        let reader = std::io::BufReader::new(file);
                        for (line_number, line) in reader.lines().enumerate() {
                            let line = line.ok()?;
                            if self.pattern.is_match(&line) {
                                return Some(Ok(Match { 
                                    file: file_path.clone(), 
                                    line: line_number + 1, 
                                    text: line,
                                }));
                            }
                        }
                    }
                }
                Err(e) => return Some(Err(e)),
            }
        }
        None
        
    }
}

#[test]
fn test_grep_iter() {
    let wdir = walkdir::WalkDir::new("/tmp");
    let grep_iter = GrepIter::new(wdir.into_iter(), regex::Regex::new("pattern").unwrap());
    for entry in grep_iter {
        match entry {
            Ok(m) => { println!("File: {}, Line: {}, Text: {}", m.file, m.line, m.text); }
            Err(e) => { println!("Error: {}", e); }
        }
    }
}

 
 
// (5) add grep() to IntoIter  (see the first example in EvenIter for i32)

trait Grep: Iterator<Item = Result<walkdir::DirEntry, walkdir::Error>> {
    fn grep(self, pattern: &str) -> GrepIter<Self>
    where
        Self: Sized,
    {
        let regex = regex::Regex::new(pattern).unwrap();
        GrepIter::new(self, regex)
    }
}

impl Grep for walkdir::IntoIter {
    // This implementation is empty because we are using the generic implementation
    // from the trait above.
}

#[test]
fn test_grep() {
    let walker = walkdir::WalkDir::new("/tmp");
    let grep_iter = walker.into_iter().grep("pattern");
    for entry in grep_iter {
        match entry {
            Ok(m) => {
                println!("{}:{}:{}", m.file, m.line, m.text);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn main() {
    println!("Hello, world!");      
}
