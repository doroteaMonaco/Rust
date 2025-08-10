
//PROGRAMMA BATTAGLIA NAVALE

/*Un programma deve gestire la costruzione di uno schema di battaglia navale 20x20 salvato
su file.
La costruzione dello schema è a passi. Alla prima invocazione si costruisce una board
vuota, poi ad ogni successiva invocazione si aggiunge una nave nella posizione indicata e si
salva lo schema aggiornato.
Il formato del file è il seguente (21 righe):
● LINEA 1: N1 N2 N3 N4, 4 interi separati da spazio che indicano il numero di navi
rispettivamente di lunghezza 1, 2 , 3 e 4, che si possono ancora aggiungere alla
board
● LINEE 2..21, 20 righe di 20 caratteri con “ “ (spazio) per le caselle vuote e “B” per
quelle con navi*/


use std::env;
use std::fs::{self, OpenOptions};
use std::fs::File;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};
use clap::Parser;

#[derive(Parser, Debug)]
struct Args{
    #[arg(short, long)]
    filename: String,
    #[arg(short, long)]
    command : String,
    #[arg(short, long)]
    ships : String
}

const bsize: usize = 20;

#[derive(Clone)]
pub struct Board {
    boats: [u8; 4],
    data: [[u8; bsize]; bsize],
}

pub enum Result<T, E>{
    Ok(T),
    Err(E),
}
pub enum Error {
    Overlap,
    OutOfBounds,
    BoatCount,
}
#[derive(Debug)]
pub enum Boat {
    Vertical(usize),
    Horizontal(usize)
}
impl Board {
    /** crea una board vuota con una disponibilità di navi */
    pub fn new(boats: &[u8; 4]) -> Board {
        let mut mat : [[u8; bsize]; bsize] = [[0; bsize]; bsize];
        Board{
            boats: *boats,
            data: mat,
        }
    }
    /* crea una board a partire da una stringa che rappresenta tutto
    il contenuto del file board.txt */
    pub fn from(s: String)->Board {
        let mut mat: [[u8; bsize]; bsize] = [[0;bsize];bsize];
        let mut boats: [u8; 4] = [0;4];
        let mut i = 0;
        let mut j = 0;
        let mut k = 0;
        let zero = '0' as u8;
        let mut firstrow = true;
        for c in s.chars() {
            if c == '\n' {
                if(firstrow){
                    firstrow = false;
                    j = 0;
                }
                else{
                    i += 1;
                    j = 0;
                }
            }
            else if c == ' '{
                if !firstrow {
                    mat[i][j] = 0;
                    j += 1;
                }
            }
            else{
                if(firstrow){
                    boats[k] = c as u8 - zero;
                    k += 1;
                }
                else{
                    mat[i][j] = 1;
                    j += 1;
                }
            }

        }
        Board {
            boats: boats,
            data: mat
        }
      }
    /* aggiunge la nave alla board, restituendo la nuova board se
    possibile */
    /* bonus: provare a *non copiare* data quando si crea e restituisce
    una nuova board con la barca, come si può fare? */
    pub fn add_boat(self, boat: Boat, pos: (usize, usize)) 
   -> Result<Board, Error> {
    let mut new_board = self.clone();
    let mut i = pos.0;
    let mut j = pos.1;
    let mut len = 0;
    
    match boat {
      Boat::Vertical(l) => {
        len = l;
        if new_board.boats[len-1]==0 {
          return Result::Err(Error::BoatCount);
        }
        if i+l>bsize {
          return Result::Err(Error::OutOfBounds);
        }
        for k in 0..l {
          if self.data[i+k][j]==1 {
            return Result::Err(Error::Overlap);
          }
        }
        for k in 0..l {
          new_board.data[i+k][j] = 1;
        }
      },
      Boat::Horizontal(l) => {
        len = l;
        if new_board.boats[len-1]==0 {
          return Result::Err(Error::BoatCount);
        }
        if j+l>bsize {
          return Result::Err(Error::OutOfBounds);
        }
        for k in 0..l {
          if self.data[i][j+k]==1 {
            return Result::Err(Error::Overlap);
          }
        }
        for k in 0..l {
          new_board.data[i][j+k] = 1;
        }
      }
    }
    new_board.boats[len-1]-=1;
    
    let content = new_board.to_string();
    let mut file = OpenOptions::new().write(true).open("board.txt").expect("Impossible to write");
    write!(file, "{}", content).expect("Error writing file");
    return Result::Ok(new_board);

   }
    /* converte la board in una stringa salvabile su file */
    pub fn to_string(&self) -> String {
        let mut s = String::new();
        for i in 0..self.boats.len() {
          s.push_str(&self.boats[i].to_string());
          s.push(' ');
        }
        s.push('\n');
        for i in 0..bsize {
          for j in 0..bsize {
            if self.data[i][j]==0 {
              s.push(' ');
            }
            else {
              s.push('B');
    
            }
            
          }
          s.push('\n');
        }
        return s;
      }
}

pub fn new_board(filename: &String, ships: [u8; 4]) -> Board {
    let mut fileopen = OpenOptions::new()
        .write(true)
        .open(filename)
        .expect("Error opening file");
    let mut firstRow : Vec<String> = Vec::new();
    for i in 0..4{
        firstRow.push(ships[i].to_string());
    }
    
    fileopen = OpenOptions::new()
        .write(true)
        .append(true)
        .open(filename)
        .expect("Error opening file");

    for str in firstRow{
        write!(fileopen, "{} ", str).expect("Error writing to file");
    }   
    print!("\n");

    for i in 0..20{
        for j in 0..20{
            write!(fileopen, " ").expect("Error writing file");
        }
        write!(fileopen, "\n");
    }

    let mut b = Board::new(&ships);
    return b; 
}

pub fn run_board(){
    let args = Args::parse();
    let filename = &args.filename;
    let command = &args.command;
    let mut ships : [u8; 4] = [0; 4];

    let mut board : Board = Board::new(&ships);

    if command == "new" {
        let file = File::create(filename).expect("Error creating file");
        let ship: Vec<&str> = args.ships.split(",").collect();
        for i in 0..4 {
            ships[i] = match ship[i].parse::<u8>(){
                Ok(n) => n,
                Err(e) => {
                    eprintln!("Conversion failed: {}", e);
                    return;
                }
            };        
        }
        board = new_board(filename, ships);
    }
    else if command == "add_boat"{
        let com: Vec<&str>= args.ships.split(",").collect();
        let orientation = com[0];
        let length = match com[1].parse::<usize>(){
            Ok(n) => n,
            Err(e) => {
                eprintln!("Conversion failed: {}", e);
                return;
            }
        };
        let row = match com[2].parse::<usize>(){
            Ok(n) => n,
            Err(e) => {
                eprintln!("Conversion failed: {}", e);
                return;
            }
        };
        let col = match com[3].parse::<usize>(){
            Ok(n) => n,
            Err(e) => {
                eprintln!("Conversion failed: {}", e);
                return;
            }
        };

        let pos = (row, col);
        let mut boat : Boat = Boat::Horizontal(0);

        if orientation == "V" {
            boat = Boat::Vertical(length);
        }
        else if orientation == "H" {
            boat = Boat::Horizontal(length);
        }
        else{
            eprintln!("Invalid orientation");
            return;
        }

        let content = match fs::read_to_string(filename) {
            Ok(data) => data,
            Err(err) => {
              eprintln!("Error in reading file {}: {}", filename, err);
              return;
            }
        };
        board = Board::from(content);
        println!{"{} {} {} {}", board.boats[0], board.boats[1], board.boats[2], board.boats[3]};

        board = match board.add_boat(boat, pos) { //riassegno a board il valore restituito dalla funzione add_boat
            Result::Err(e) => { //se la funzione add_boat restituisce un errore
              match e {
                Error::BoatCount => { //se l'errore è BoatCount
                  println!("Impossible to place ship, no ships available!");
                  let content = match fs::read_to_string(filename) {
                    Ok(data) => data,
                    Err(err) => {
                      eprintln!("Error in reading file {}: {}", filename, err);
                      return;
                    }
                  };
                  Board::from(content) //ritorno la board creata a partire dal contenuto del file
                },
                Error::OutOfBounds => { //se l'errore è OutOfBounds
                  println!("Impossible to place ship, ship out of bounds!");
                  let content = match fs::read_to_string(filename) {
                    Ok(data) => data,
                    Err(err) => {
                      eprintln!("Error in reading file {}: {}", filename, err);
                      return;
                    }
                  };
                  Board::from(content) //ritorno la board creata a partire dal contenuto del file
                },
                Error::Overlap => {//se l'errore è Overlap
                  println!("Impossible to place ship, ship overlap error!");
                  let content = match fs::read_to_string(filename) {
                    Ok(data) => data,
                    Err(err) => {
                      eprintln!("Error in reading file {}: {}", filename, err);
                      return;
                    }
                  };
                  Board::from(content) //ritorno la board creata a partire dal contenuto del file
                }
    
              }
            },
            Result::Ok(s) => { //se la funzione add_boat restituisce una board
              s //ritorno la board
            }
        };
    }
}

fn main() {
    run_board();
}
 