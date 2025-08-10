use std::{convert, env};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "slugify")]
#[command(version = "0.1.0")]
#[command(about = "Convert a string into a slug")]
struct Args{
    #[arg(short, long)]
    slug_in : String, 
    #[arg(short, long)]
    repeat : u32, 
    #[arg(short, long)]
    verbose: bool,
}

//tabella di conversione
const SUBS_I : &str =
"àáâäæãåāăąçćčđďèéêëēėęěğǵḧîïíīįìıİłḿñńǹňôöòóœøōõőṕŕřßśšşșťțûüùúūǘůűųẃẍÿýžźż";
const SUBS_O: &str =
"aaaaaaaaaacccddeeeeeeeegghiiiiiiiilmnnnnoooooooooprrsssssttuuuuuuuuuwxyyzz
z";

fn slugify (s: &str) -> String {
    let mut converted : String = String::new();
    let mut str : Vec<char> = s.chars().collect();
    let mut fin : char;

    for c in str{
        let mut res_lower  = c.to_lowercase();
        for car in res_lower{
            fin = conv(c);
            if(converted.ends_with('-') && fin != '-'){
                converted.push( conv(car));
            }
            else if(!converted.ends_with('-')){
                converted.push( conv(car));
            }
        }
    }

    if(converted.ends_with('-')){
        converted.pop();
    }

    return converted;
}

fn conv(c: char) -> char {
    let vettI : Vec<char> = SUBS_I.chars().collect();
    let vettO : Vec<char> = SUBS_O.chars().collect();

    let mut index : u32 = 0;

    if c.is_ascii_alphabetic(){
        return c;
    }
    else if c.is_digit(){
        return c;
    }
    else if c.is_alphabetic(){
        for car in vettI {
            if car == c {
                return vettO[index as usize];
            }
            index += 1;
        }
        return '-';
    }
    else{
        return '-';
    }
}

fn run_slugify(){
    //let args: Vec<String> = std::env::args().collect();
    
    let args = Args::parse();

    let string = args.slug_in;
    let mut string_conv = slugify(&string);

    println!("The converted string is: {}", string_conv);

    println!("Variabile repeat: {}", args.repeat);
    println!("Variabile verbose: {}", args.verbose);
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn conv_accentata(){
        let c : char = 'à';
        assert_eq!(conv(c), 'a');
    }

    #[test]
    fn conv_non_accentata(){
        let c : char = 'a';
        assert_eq!(conv(c), 'a');
    }

    #[test]
    fn conv_sconosciuta(){
        let c : char = '!';
        assert_eq!(conv(c), '-');
    }

    #[test]
    fn conv_accentata_non_lista(){
        let c : char = 'ῶ';
        assert_eq!(conv(c), '-');
    }

    #[test]
    fn stringa_con_spazio(){
        let string = String::from("Scrivi una stringa");
        assert_eq!(slugify(&string), "scrivi-una-stringa");
    }

    #[test]
    fn stringa_con_accentati(){
        let string = String::from("Scrivi una stringa olè");
        assert_eq!(slugify(&string), "scrivi-una-stringa-ole");
    }

    #[test]
    fn stringa_vuota(){
        let string = String::from("");
        assert_eq!(slugify(&string), "");
    }

    #[test]
    fn stringa_spazi_consecutivi(){
        let string = String::from("Ciao   Rust");
        assert_eq!(slugify(&string), "ciao-rust");
    }

    #[test]
    fn stringa_non_validi_consecutivi(){
        let string = String::from("Ciao ῶῶ Rust");
        assert_eq!(slugify(&string), "ciao-rust");
    }

    #[test]
    fn stringa_solo_non_validi(){
        let string = String::from("ῶῶ");
        assert_eq!(slugify(&string), "");
    }

    #[test]
    fn stringa_spazio_finale(){
        let string = String::from("Ciao Rust ");
        assert_eq!(slugify(&string), "ciao-rust");
    }

    #[test]
    fn stringa_non_validi_consecutivi_finali(){
        let string = String::from("Ciao Rust ῶῶ");
        assert_eq!(slugify(&string), "ciao-rust");
    }
    
}

fn main(){
    run_slugify();
}

