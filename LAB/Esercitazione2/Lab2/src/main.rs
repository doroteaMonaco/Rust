// paste this file into main.rs
use std::{env, fs, process};

fn stats(text: &str) -> [u32; 26] {
    let mut counts : [u32; 26] = [0; 26]; //inizializzato il vettore a 0
    let mut letter : char = 'a'; //dichiaro un char
    let mut num : u32 = letter as u32; //converto in numero
    
    for c in text.chars(){
        if c.is_alphabetic(){
            let c_lower : u32 = c.to_ascii_lowercase() as u32; //trasformo in minuscolo e faccio cast
            let l : usize = (c_lower as usize - num as usize); //ottengo offset per inserire nel vettore
            counts[l] += 1;
        }
    }
    return counts;
}

fn is_pangram(counts: &[u32]) -> bool {
    if counts.len() != 26{
        return false;
    }
    for c in counts.iter() {
        if *c == 0 {
            return false;
        }
    }
    return true;
}

// call this function from main
// load here the contents of the file
pub fn run_pangram() {
    let args : Vec<String> = env::args().collect();
    if args.len() < 2{
        println!("Number of arguments not valid");
    }

    let filename = &args[1];
    let strings = match fs::read_to_string(filename) {
        Ok(data) => data,
        Err(err) => {
            eprint!("Error reading file");
            return;
        }
    };

    let mut countsVector = stats(&strings);
    let mut letter : char = 'a';
    let mut num : u32 = letter as u32;
    let mut i : u32 = 0;

    if is_pangram(&countsVector){
        println!("The string is a pangram!: {}", strings);
        for c in countsVector{
            let l : char = (num as u8 + i as u8) as char;
            println!("{}: {}", l, c);
            i += 1;
        }
    }
    else{
        println!("The string is not a pangram!");
    }
}


// please note, code has been splittend in simple functions in order to make testing easier

#[cfg(test)] // this is a test module
mod tests
{   
    // tests are separated modules, yuou must import the code you are testing
    use super::*;
    
    #[test]
    fn test_all_ones() {
        let counts = [1; 26];
        assert!(is_pangram(&counts));
    }

    #[test]
    fn test_some_zeros() {
        let mut counts = [0; 26];
        counts[0] = 0;
        counts[1] = 0;
        assert!(!is_pangram(&counts));
    }
    
    #[test]
    fn test_increasing_counts() {
        let mut counts = [0; 26];
        for i in 0..26 {
            counts[i] = i as u32 + 1;
        }
        assert!(is_pangram(&counts));
    }

    #[test]
    fn test_wrong_size()  {
        let counts = [1; 25];
        assert!(!is_pangram(&counts));
    }    
    
    #[test]
    fn test_stats_on_full_alphabet() {
        let counts = stats("abcdefghijklmnopqrstuvwxyz");
        for c in counts {
            assert!(c == 1);
        }
    }

    #[test]
    fn test_stats_on_empty_string() {
        let counts = stats("");
        for c in counts {
            assert!(c == 0);
        }
    }

    #[test]
    fn test_stats_missing_char() {
        let counts = stats("abcdefghijklmnopqrstuvwxy");
        for c in counts.iter().take(25) {
            assert!(*c == 1);
        }
        assert!(counts[25] == 0);

    }

    #[test]
    fn test_stats_on_full_tring() {
        let contents = "The quick brown fox jumps over the lazy dog";
        let counts = stats(contents);
        for c in counts {
            assert!(c > 0);
        }
    }

    #[test]
    fn test_stats_with_punctuation() {
        let contents = "The quick brown fox jumps over the lazy dog!";
        let counts = stats(contents);
        for c in counts {
            assert!(c > 0);
        }
    }

    #[test] 
    fn test_missing_char_on_full_string() {
        let contents = "The quick brown fox jumps over the laz* dog";
        let counts = stats(contents);
        println!("{:?}", counts);
        for (i, c) in counts.iter().enumerate() {
            if i == 24 {
                assert!(*c == 0);
            } else {
                assert!(*c > 0);
            }
            
        }
    }

    #[test]
    fn test_is_pangram() {
        let counts = stats("The quick brown fox jumps over the lazy dog");
        assert!(is_pangram(&counts));
    }
}

fn main() {
    run_pangram();
}

