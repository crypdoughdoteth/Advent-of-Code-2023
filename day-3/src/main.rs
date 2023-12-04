// 0 -> 139 line idx
// 0 -> 139 line num

use anyhow::Result;
use std::fs;

#[derive(Clone, Copy, Debug)]
struct Point(usize, usize);

#[derive(Clone, Copy, Debug)]
struct Number {
    start: Point,
    len: usize,
    val: u16,
}

impl Number {
    pub fn new(start: Point, len: usize, val: u16) -> Self {
        Self { start, len, val }
    }

    // search space
    // y(start), y(start) + 1, y(start) - 1
    // x - 1 -> x.len() + 1
}

#[derive(Clone)]
struct SearchArea<'a> {
    s1: Option<&'a str>,
    s2: &'a str,
    s3: Option<&'a str>,
}

impl <'a> SearchArea<'a> {
    pub fn new(lines: &'a Vec<&'a str>, num_data: Number) -> Self {
        let start_point = num_data.start;
        // 0 is the y coordinate, 1 is the x coordinate
        // L -> R bound
        // println!("{:#?}", num_data);
        let mut rstart = start_point.1;
        let mut rend = start_point.1 + num_data.len - 1;
        if rstart > 0 {
            rstart -= 1;
        }
        if rend < 139 {
            rend += 1;
        }

        // Upper -> Lower bound
        let mid = start_point.0;
    
        if mid == 0 {
            SearchArea {
                s1: None,
                s2: &lines[mid][rstart..rend],
                s3: Some(&lines[mid + 1][rstart..rend]),
            }

        } else if mid == 139 {
            SearchArea {
                s1: Some(&lines[mid - 1][rstart..rend]),
                s2: &lines[mid][rstart..rend],
                s3: None,
            }
        } else {
            SearchArea {
                s1: Some(&lines[mid - 1][rstart..rend]),
                s2: &lines[mid][rstart..rend],
                s3: Some(&lines[mid + 1][rstart..rend]),
            }
        }

    }
    pub fn detect_symbols(&self) -> bool {
        let mut found: bool = false;
        if let Some(line) = self.s1 {
            line.chars().for_each(|chr| {
                if chr == '#'
                    || chr == '*'
                    || chr == '+'
                    || chr == '%'
                    || chr == '@'
                    || chr == '='
                    || chr == '/'
                    || chr == '$'
                    || chr == '&'
                    || chr == '-'
                {
                    found = true;
                }
            });
        };
        
        self.s2.chars().for_each(|chr| {
                if chr == '#'
                    || chr == '*'
                    || chr == '+'
                    || chr == '%'
                    || chr == '@'
                    || chr == '='
                    || chr == '/'
                    || chr == '$'
                    || chr == '&'
                    || chr == '-'
                {
                    found = true;
                }
            });

        if let Some(line) = self.s3 {
            line.chars().for_each(|chr| {
                if chr == '#'
                    || chr == '*'
                    || chr == '+'
                    || chr == '%'
                    || chr == '@'
                    || chr == '='
                    || chr == '/'
                    || chr == '$'
                    || chr == '&'
                    || chr == '-'
                {
                    found = true;
                }
            });
        };
        found
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("day-3/input.txt")?;
    let lines: Vec<&str> = input.lines().collect::<Vec<_>>();
    let num_data = lines
        .iter()
        .enumerate()
        .flat_map(|(i, l)| {
            let mut cached_num_str: String = String::new();
            let mut num_str_start: usize = 0;
            let mut prev_num = false;
            let mut numbers: Vec<Number> = Vec::new();
            l.chars().enumerate().for_each(|(idx, ch)| {
                if ch.is_numeric() {
                    if prev_num == false {
                        num_str_start = idx;
                    }
                    cached_num_str.push(ch);
                    prev_num = true;
                } else {
                    if prev_num == true {
                        let number = Number {
                            start: Point(i, idx - cached_num_str.len()),
                            len: cached_num_str.len(),
                            val: cached_num_str.parse::<u16>().unwrap(),
                        };
                        numbers.push(number);
                    }
                    prev_num = false;
                    cached_num_str.clear();
                }
               if idx == 139 {
                    if prev_num == true {
                        let number = Number {
                            start: Point(i, idx - cached_num_str.len()),
                            len: cached_num_str.len(),
                            val: cached_num_str.parse::<u16>().unwrap(),
                        };
                        numbers.push(number);
                    }
                    prev_num = false;
                    cached_num_str.clear();
               } 
            });
            numbers
        })
        .collect::<Vec<Number>>();
        // println!("{:#?}", num_data); 
        


    let pn =  num_data.iter().fold(0, |acc, n| {
        let sa = SearchArea::new(&lines, *n);
        if sa.detect_symbols() == true {
            n.val + acc
        } else {
            0
        }
    });

   println!("{pn}");
    
    Ok(())
}
