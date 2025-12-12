use std::num::ParseIntError;
use std::{fs::read_to_string, ops::RangeInclusive};
use std::str::FromStr;
use std::collections::VecDeque;

pub enum Err<E> {
    MissingSeparator,
    Start(E),
    End(E),
    ParseIntError(ParseIntError),
}

impl From<ParseIntError> for Err<ParseIntError> {
  fn from(e: ParseIntError) -> Self {
    Err::ParseIntError(e)
  }
}

struct ParsedRangeInclusive<T>(pub RangeInclusive<T>);

impl<T> FromStr for ParsedRangeInclusive<T> where T: FromStr{
  type Err = crate::Err<T::Err>;  
  
  fn from_str(s: &str) -> Result<Self, Self::Err> {  
    let (f, l) = s.split_once('-').ok_or(Err::MissingSeparator)?;
    let start: T = f.parse().map_err(Err::Start)?;
    let end: T = l.parse().map_err(Err::End)?;
    Ok(ParsedRangeInclusive(start..=end))
  }
}

#[derive(Debug)]
struct IdRanges{
    ranges: Vec<RangeInclusive<u64>>,   
}

impl IdRanges{
    fn new() ->Self{
        Self{
            ranges : vec![]
        }
    }

    fn push(&mut self, r: RangeInclusive<u64>) { self.ranges.push(r); }

    fn union(&self, r_in : RangeInclusive<u64>) -> Self{
        let mut new : Self = Self::new();
        let mut mut_r_in = r_in.clone();
        for r in &self.ranges{
            if !r.contains(mut_r_in.start()) && !r.contains(mut_r_in.end()) 
            && !mut_r_in.contains(r.start()) &&  !mut_r_in.contains(r.end())
            { new.push(r.clone()); }
            if r.contains(mut_r_in.start()) { mut_r_in = RangeInclusive::new(*r.start(), *mut_r_in.end()); }
            if r.contains(mut_r_in.end()) { mut_r_in = RangeInclusive::new (*mut_r_in.start(), *r.end()); }
        }
        new.push(mut_r_in);
        new
    }

    fn len(&self) -> usize
    {
        let mut res = 0;
        for r in &self.ranges{
            res += *r.end() - *r.start() + 1;
        }
       
        res as usize
    }

    fn is_ingredient_fresh(&self, id:u64) -> bool{
        for r in &self.ranges {
            if r.contains(&id) {
                return true;
            }
        }
        false
    }
}

fn handle_input(filename: &str) -> Result<usize, Err<ParseIntError>> {
    let mut ranges = IdRanges::new();

    let mut lines: VecDeque<String> = read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect();
    
    let mut s = lines.pop_front();
    while s != Some("".to_string())    
    {
        let ParsedRangeInclusive(r):ParsedRangeInclusive<u64> = s.unwrap().parse()?;
        print!("adding [{:?}] to {:?}", r, ranges);
        ranges = ranges.union(r);
        println!(" => {:?}", ranges);

        s = lines.pop_front();
    }
    let res = ranges.len();

    Ok(res)
}

fn main() {
    let n_fresh = handle_input("./real_input");
    match n_fresh {
        Ok(1) => println!("fresh ingredient"),
        Ok(n) => println!("{} fresh ingredients", n),
        _ => println!("parse error")
    }
}


