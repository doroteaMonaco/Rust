use std::io;
use std::io::BufRead;
use regex::Regex;

// (1) LineEditor: implement functionality
pub struct LineEditor {
    lines: Vec<String>,
}

impl LineEditor {
    pub fn new(s: String) -> Self {
        let line: Vec<String> = s.lines().map(|l| l.to_string()).collect();
        LineEditor { lines: line }
    }

    // create a new LineEditor from a file
    pub fn from_file(file_name: &str) -> Result<Self, io::Error> {
        let file = std::fs::File::open(file_name)?;
        let reader = io::BufReader::new(file);
        let mut lines = Vec::new();
        for line in reader.lines() {
            lines.push(line?);
        }
        Ok(LineEditor { lines })
    }

    pub fn all_lines(&self) -> Vec<&str> {
        self.lines.iter().map(|l| l.as_str()).collect()
    }

    pub fn replace(&mut self, line: usize, start: usize, end: usize, subst: &str) {
        if let Some(l) = self.lines.get_mut(line) {
            if start < end && end <= l.len() {
                let mut new_line = String::new();
                new_line.push_str(&l[..start]);
                new_line.push_str(subst);
                new_line.push_str(&l[end..]);
                *l = new_line;
            }
        }
    }

    pub fn get_lines(&self) -> &Vec<String> {
        &self.lines
    }

    pub fn set_lines(&mut self, lines: Vec<String>) {
        self.lines = lines;
    }
}

// (2) Match contains the information about the match. Fix the lifetimes
// repl will contain the replacement.
// It is an Option because it may be not set yet or it may be skipped 
struct Match<'a> {
    pub line: usize,
    pub start: usize,
    pub end: usize,
    pub text: &'a str,
    pub repl: Option<String>,
}

// use the crate "regex" to find the pattern and its method find_iter for iterating over the matches
// modify if necessary, this is just an example for using a regex to find a pattern
fn find_example<'a>(lines: &'a Vec<&'a str>, pattern: &'a str) -> Vec<Match<'a>> {
    let mut matches = Vec::new();
    let re = regex::Regex::new(pattern).unwrap();
    for (line_idx, line) in lines.iter().enumerate() {
        for mat in re.find_iter(line) {
            matches.push(Match {
                line: line_idx,
                start: mat.start(),
                end: mat.end(),
                text: &line[mat.start()..mat.end()],
                repl: None,
            });
        }
    }
    matches
}

// (3) Fix the lifetimes of the FindReplace struct
// (4) implement the Finder struct
struct FindReplace<'a> {
    lines: &'a Vec<&'a str>,
    pattern: String,
    matches: Vec<Match<'a>>,
}

impl<'a> FindReplace<'a> {
    pub fn new(lines: &'a Vec<&'a str>, pattern: &'a str) -> Self {
        let matches = find_example(lines, pattern);
        FindReplace {
            lines,
            pattern: pattern.to_string(),
            matches,
        }
    }

    // return all the matches
    pub fn matches(&self) -> &Vec<Match<'a>> {
        &self.matches
    }

    // apply a function to all matches and allow to accept them and set the repl
    // useful for prompting the user for a replacement
    pub fn apply(&mut self, fun: impl Fn(&mut Match) -> bool) {
        for m in &mut self.matches.iter_mut() {
            if fun(m) {
                m.repl = Some("some repl".to_string());
            } else {
                m.repl = None;
            }
        }
    }
}

// (5) how FindReplace should work together with the LineEditor in order
// to replace the matches in the text
#[test]
fn test_find_replace() {
    let s = "Hello World.\nA second line full of text.";
    let mut editor = LineEditor::new(s.to_string());

    let lines = editor.all_lines();
    let mut finder = FindReplace::new(&lines, "ll");

    // find all the matches and accept them 
    finder.apply(|m| {
        println!("{} {} {} {}", m.line, m.start, m.end, m.text);
        m.repl = Some("some repl".to_string());
        true
    });

    let mut subs = Vec::new();
    for m in finder.matches() {
        if let Some(ref repl) = m.repl {
            subs.push((m.line, m.start, m.end, repl.clone()));
        }
    }
    for (line, start, end, subst) in subs {
        editor.replace(line, start, end, &subst);
    }
}


// (6) LazyFinder: implement a lazy iterator
#[derive(Debug, Clone, Copy)]
struct FinderPos {
    pub line: usize,
    pub offset: usize,
}

struct LazyFinder<'a> {
    lines: &'a Vec<&'a str>,
    pattern: String,
    pos: Option<FinderPos>,
}

impl<'a> LazyFinder<'a> {
    pub fn new(lines: &'a Vec<&'a str>, pattern: &'a str) -> Self {
        LazyFinder {
            lines,
            pattern: pattern.to_string(),
            pos: None,
        }
    }

    pub fn next(&mut self) -> Option<Match<'a>> {
        // remember:
        // return None if there are no more matches, return Some(Match) if there is a match
        // each time save the position of the match for the next call
        // find the next match
        if self.pos.is_none() {
            self.pos = Some(FinderPos { line: 0, offset: 0 });
        }
        let mut pos = self.pos.unwrap();
        let re = Regex::new(&self.pattern).unwrap();
        while pos.line < self.lines.len() {
            let line = self.lines[pos.line];
            if let Some(mat) = re.find_at(line, pos.offset) {
                let m = Match {
                    line: pos.line,
                    start: mat.start(),
                    end: mat.end(),
                    text: &line[mat.start()..mat.end()],
                    repl: None,
                };
                // update the position for the next call
                pos.offset = mat.end();
                self.pos = Some(pos);
                return Some(m);
            } else {
                // move to the next line
                pos.line += 1;
                pos.offset = 0;
            }
        }
        // no more matches
        self.pos = None;
        None
    }
}

// (7) example of how to use the LazyFinder
#[test]
fn test_lazy_finder() {
    let s = "Hello World.\nA second line full of text.";
    let mut editor = LineEditor::new(s.to_string());

    let lines = editor.all_lines();
    let mut finder = LazyFinder::new(&lines, "ll");

    // find all the matches and accept them 
    while let Some(m) = finder.next() {
        println!("{} {} {} {}", m.line, m.start, m.end, m.text);
    }
}


// (8) FindIter: implement a real iterator
struct FindIter<'a> {
    lines: Vec<&'a str>,
    pattern: String,
    pos: Option<FinderPos>,

}

impl<'a> FindIter<'a> {
    pub fn new(lines: Vec<&'a str>, pattern: &str) -> Self {
        FindIter {
            lines,
            pattern: pattern.to_string(),
            pos: None,
        }
    }
}

impl<'a> Iterator for FindIter<'a> {
    type Item = Match<'a>; // <== we inform the Iterator that we return a Match

    fn next(&mut self) -> Option<Self::Item> {
        // find the next match
        if self.pos.is_none() {
            self.pos = Some(FinderPos { line: 0, offset: 0 });
        }
        let mut pos = self.pos.unwrap();
        let re = Regex::new(&self.pattern).unwrap();
        while pos.line < self.lines.len() {
            let line = self.lines[pos.line];
            if let Some(mat) = re.find_at(line, pos.offset) {
                let m = Match {
                    line: pos.line,
                    start: mat.start(),
                    end: mat.end(),
                    text: &line[mat.start()..mat.end()],
                    repl: None,
                };
                // update the position for the next call
                pos.offset = mat.end();
                self.pos = Some(pos);
                return Some(m);
            } else {
                // move to the next line
                pos.line += 1;
                pos.offset = 0;
            }
        }
        // no more matches
        self.pos = None;
        None
    }
}

// (9) test the find iterator
#[test]
fn test_find_iter() {
    let s = "Hello World.\nA second line full of text.";
    let mut editor = LineEditor::new(s.to_string());

    let lines = editor.all_lines();
    let mut finder = FindIter::new(lines, "ll");

    // find all the matches and accept them 
    for m in finder {
        println!("{} {} {} {}", m.line, m.start, m.end, m.text);
    }
}

fn main() {}