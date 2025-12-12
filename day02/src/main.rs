use::std::fmt;
use std::ops::RangeInclusive;
use std::str::FromStr;
use std::num::ParseIntError;
use std::collections::HashSet;


#[derive(Debug)]
pub enum Err<E> {
    MissingSeparator,
    Start(E),
    End(E),
}

impl<E: fmt::Display> fmt::Display for Err<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Err::MissingSeparator =>
                write!(f, "missing '-' separator in range"),
            Err::Start(e) =>
                write!(f, "error parsing start of range: {e}"),
            Err::End(e) =>
                write!(f, "error parsing end of range: {e}"),
        }
    }
}

impl<E: std::error::Error + 'static> std::error::Error for Err<E> {}

#[derive(Debug)]
struct RangeIds<T>(pub RangeInclusive<T>);

impl<T> FromStr for RangeIds<T> where T: FromStr{
  type Err = crate::Err<T::Err>;  
  
  fn from_str(s: &str) -> Result<Self, Self::Err> {  
    let (f, l) = s.split_once('-').ok_or(Err::MissingSeparator)?;
    let start: T = f.parse().map_err(Err::Start)?;
    let end: T = l.parse().map_err(Err::End)?;
    Ok(RangeIds(start..=end))
  }
}


fn divs(n: u8) -> Vec<u8> {
    let mut divs = vec![];
    let mut d = 2;
    while d * d <= n {
        if n % d == 0 {
            divs.push(d);
            if d * d != n {
                divs.push(n / d);
            }
        }
        d += 1;
    }
    if divs.is_empty() { divs.push(1); }
    else {
      divs.sort();
      divs.reverse();
    }
    divs
}

#[derive(Debug)]
struct BadIdGenerator{
  start: u64,
  end: u64,
  pattern: u64,
  builder:u64,
  limit: u64,
  sz_divs: Vec<u8>,
}


impl BadIdGenerator{
    
  fn new (start: u64, end: u64) -> Self{
    //handle limit case with bad lower bound but valid upper bound
    let start = if start < 10 && end > 10 { 10 } else { start }; 
    
    let start_sz = start.ilog(10) as u8 + 1;
    let mut sz_divs = divs(start_sz);
    let pattern_len = sz_divs.pop().unwrap();

    let n_pattern = start_sz / pattern_len;
    let mut pattern: u64 = start / u64::pow(10, (start_sz - pattern_len) as u32);
    
    let mut builder: u64 = 1;
    let pow10 = u64::pow(10, (pattern_len) as u32);
    for _ in 1..n_pattern { builder = builder * pow10 + 1; } 

    if (pattern * builder) < start { pattern += 1; } 
    let limit =  u64::pow(10, pattern_len as u32);
    
    Self{
      start: start,
      end: end,
      pattern: pattern,
      builder: builder,
      limit: limit,
      sz_divs: sz_divs,
    }
  }

  //returns the current bad id;
  fn get_bad_id(&self)->u64{
    self.pattern * self.builder
  }

  //returns the next bad id and updates itself;
  fn get_next_bad_id(&mut self) -> u64{
    
    
    if self.pattern == self.limit - 1 {
      *self = BadIdGenerator::new(self.get_bad_id() + 1, self.end);
      // println!("    {:?}", self);
    }
    else{ self.pattern += 1; }
    
    self.get_bad_id()
  }

  // return the set of bad ids for the range 
  fn get_bad_ids(&mut self) -> HashSet<u64>{
    let mut set = HashSet::<u64>::new();
    loop {
      let mut cur_bad_id: u64 = self.get_bad_id();
      
      while self.end >= cur_bad_id {      
        set.insert(cur_bad_id);
        cur_bad_id = self.get_next_bad_id();
      }

      if !self.sz_divs.is_empty(){
        self.pop_next_div();
      }
      else { break; }
    }

    set
  }

  fn pop_next_div(&mut self){   
    let start_sz = self.start.ilog(10) as u8 + 1;
    let pattern_len = self.sz_divs.pop().unwrap();

    let n_pattern = start_sz / pattern_len;
    let mut pattern: u64 = self.start / u64::pow(10, (start_sz - pattern_len) as u32);
    
    let mut builder: u64 = 1;
    let pow10 = u64::pow(10, (pattern_len) as u32);
    for _ in 1..n_pattern { builder = builder * pow10 + 1; } 

    if (pattern * builder) < self.start { pattern += 1; } 
    let limit =  u64::pow(10, pattern_len as u32);
    
      self.pattern = pattern;
      self.builder = builder;
      self.limit = limit;


      // println!("    {:?}", self);
  }
  

}


fn handle_input(input : &str, acc: &mut u64) -> Result<(), Err<ParseIntError>> {
  let RangeIds(r): RangeIds<u64> = input.parse()?;
  
  let mut g = BadIdGenerator::new(*r.start(), *r.end());

  print!("{}, ", input);

  let bad_ids = g.get_bad_ids();

  if bad_ids.is_empty() {
    println!("contains no invalid ids");
  }
  else{
    println!("contains {:?}", bad_ids);
  }

  *acc += bad_ids.into_iter().sum::<u64>();

  Ok(())
}

fn main() {
  let mut acc: u64 = 0;
  for input in _REAL_INPUT.split(','){
    handle_input(input, &mut acc).expect("parsing error");
  }

  println!("sum of bad ids : {}", acc);
}



const _TEST_INPUT : &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";


const _REAL_INPUT : &str = "1090286-1131879,3259566-3404881,138124-175118,266204727-266361099,16765-24272,7657360692-7657593676,88857504-88926597,6869078-6903096,48444999-48532270,61427792-61580535,71-103,8077-10421,1920-2560,2-17,951-1259,34-50,28994-36978,1309-1822,9393918461-9393960770,89479-120899,834641-988077,5389718924-5389797353,34010076-34214499,5063-7100,607034-753348,19098586-19261191,125085556-125188689,39839-51927,3246-5037,174-260,439715-473176,187287-262190,348-535,58956-78301,4388160-4505757,512092-584994,13388753-13534387";


fn _handle_input_part1(input : &str, acc: &mut u64) -> Result<(), Err<ParseIntError>> {
  let RangeIds(r): RangeIds<u64> = input.parse()?;

  //setup
  let start_log = r.start().ilog(10);
  let end_log = r.end().ilog(10);
  
  let is_log_even = start_log % 2 == 0;
  let are_log_diff = end_log - start_log != 0;

  if !are_log_diff && is_log_even { 
    println!("{} contains no invalid ids", input);
    return Ok(()); 
  }
  else {
    print!("{}", input);
  }

  let mut pow10 = u64::pow(10, (start_log + 1) / 2);
  let mut pattern: u64;
  
  if is_log_even {
    pattern = pow10;
    pow10 *= 10;
  } 
  else{
    pattern = r.start() / pow10;
    if pattern < (*r.start() % pow10) { pattern += 1; }
  }
  let mut cur_bad_id = pattern * (pow10 + 1);

  //ok lets go
  let mut bad_ids: Vec<u64> = Vec::new();

  while *r.end() >= cur_bad_id {
    bad_ids.push(cur_bad_id);
    
    pattern += 1;
    if pattern == pow10 { pow10 *= 10; }

    cur_bad_id = pattern * (pow10 + 1);
  }
  if bad_ids.is_empty() {
    println!(" contains no invalid ids");
  }
  else{
    println!(" contains {:?}", bad_ids);
  }

  *acc += bad_ids.into_iter().sum::<u64>();

  Ok(())
}
