const SUBS_I : &str =
"àáâäæãåāăąçćčđďèéêëēėęěğǵḧîïíīįìıİłḿñńǹňôöòóœøōõőṕŕřßśšşșťțûüùúūǘůűųẃẍÿýžźż";
const SUBS_O: &str =
"aaaaaaaaaacccddeeeeeeeegghiiiiiiiilmnnnnoooooooooprrsssssttuuuuuuuuuwxyyzz
z";
trait MySlug{
    fn is_slug(&self) -> bool;
    fn to_slug(&self) -> String;
}

impl<T> MySlug for T where T: AsRef<str>{
    fn is_slug(&self) -> bool {
        let s = self.as_ref();
        for c in s.chars() {
            if c.is_uppercase() {
                return false; // Non sono permesse lettere maiuscole
            }
            if !(c.is_alphanumeric() || c == '-') {
                return false; // Solo caratteri alfanumerici o '-' sono permessi
            }
        }
        if !s.starts_with('-') || !s.ends_with('-') {
            return false; // Non può iniziare o finire con '-'
        }
        return true;
    }
    fn to_slug(&self) -> String {
        let s = self.as_ref();
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

}



fn conv(c: char) -> char {
    let vettI : Vec<char> = SUBS_I.chars().collect();
    let vettO : Vec<char> = SUBS_O.chars().collect();

    let mut index : u32 = 0;

    if c.is_ascii_alphabetic(){
        return c;
    }
    else if c.is_digit(10){
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

fn main() {
    let s1 = String::from("Hello-String");
    let s2 = "hello-slice";
    println!("{}", s1.is_slug()); // false
    let s3: String = s1.to_slug();
    println!("{}", s2.is_slug()); // true
    let s4: String = s2.to_slug();
    println!("s3:{} s4:{}", s3, s4); 
}
