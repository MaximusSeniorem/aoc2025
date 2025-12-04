use::std::fmt;
use std::ops::RangeInclusive;
use std::str::FromStr;
use std::num::ParseIntError;


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


fn handle_input(input : &str, acc: &mut u64) -> Result<(), Err<ParseIntError>> {
  let RangeIds(r): RangeIds<u64> = input.parse()?;

  //setup
  let start_log = r.start().ilog(10);
  let end_log = r.end().ilog(10);
  
  let is_log_pair = start_log % 2 == 0;
  let is_log_diff = end_log - start_log != 0;

  if !is_log_diff && is_log_pair { 
    println!("{} contains no invalid ids", input);
    return Ok(()); 
  }
  else {
    print!("{}", input);
  }

  let n = if is_log_pair { (start_log + 2) / 2 } 
      else { (start_log + 1) / 2 };
  let mut cur_pow = u64::pow(10, n);
  let mut pattern = if is_log_pair { cur_pow / 10 } else { r.start() / cur_pow };
  let mut cur_bad_id = pattern * cur_pow + pattern;

  while !r.contains(&cur_bad_id) && cur_bad_id < *r.end() {
    pattern += 1;
    if pattern == cur_pow { cur_pow *= 10; }

    cur_bad_id = pattern * cur_pow + pattern;
  }
  if !r.contains(&&cur_bad_id)  { 
    println!(" contains no invalid ids");
    return Ok(()); 
  }
  
  //ok lets go
  let mut bad_ids: Vec<u64> = Vec::new();

  while r.contains(&cur_bad_id) {
    bad_ids.push(cur_bad_id);
    
    pattern += 1;
    if pattern == cur_pow { cur_pow *= 10; }

    cur_bad_id = pattern * cur_pow + pattern;
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

fn main() {
  let mut acc: u64 = 0;
  for input in REAL_INPUT.split(','){
    handle_input(input, &mut acc).expect("parsing error");
  }

  println!("sum of bad ids : {}", acc);
}


const TEST_INPUT : &str = "1-9,456-999,11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";


const REAL_INPUT : &str = "1090286-1131879,3259566-3404881,138124-175118,266204727-266361099,16765-24272,7657360692-7657593676,88857504-88926597,6869078-6903096,48444999-48532270,61427792-61580535,71-103,8077-10421,1920-2560,2-17,951-1259,34-50,28994-36978,1309-1822,9393918461-9393960770,89479-120899,834641-988077,5389718924-5389797353,34010076-34214499,5063-7100,607034-753348,19098586-19261191,125085556-125188689,39839-51927,3246-5037,174-260,439715-473176,187287-262190,348-535,58956-78301,4388160-4505757,512092-584994,13388753-13534387";
