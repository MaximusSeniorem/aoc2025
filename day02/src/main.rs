use std::ops::{ RangeInclusive };
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
enum Err {
  ParseIntError(ParseIntError),
  ParseError,
}

impl From<ParseIntError> for Err {
  fn from(e: ParseIntError) -> Self {
    Err::ParseIntError(e)
  }
}

#[derive(Debug)]
struct RangeIds {
  first: String,
  last: String,
  range : RangeInclusive<u32>,
}

impl RangeIds {
  fn new (s: &str) -> Result<Self, Err> {
    s.parse()
  } 
}

impl FromStr for RangeIds {
      type Err = crate::Err;  
  
  fn from_str(s: &str) -> Result<Self, Self::Err> {  
    match s.split_once('-'){
      Some((f, l)) => Ok(
        Self{
          first: f.to_string(), 
          last: l.to_string(),
          range: RangeInclusive::new(f.parse::<u32>()?, l.parse::<u32>()?)
        }),
      None => Err(Err::ParseError),
    } 
  }
}

fn handle_input(input : &str, inv_ids : &mut Vec<u32>) -> Result<(), Err> {
  let r = RangeIds::new(input)?;
  //println!("{:?}, contains? : {}, {}", r, r.range.contains(&r.range.start()), r.range.contains(&r.range.end()));

  let len_diff = r.last.len() - r.first.len();
  if len_diff == 0 && (r.first.len() % 2) != 0 { return Ok(()); }
  

  
  Ok(())
}

fn main() {
  let mut inv_ids = Vec::<u32>::new();
  for input in TEST_INPUT.split(','){
    handle_input(input, &mut inv_ids).expect("parsing error");
  }
}


const TEST_INPUT : &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";


const REAL_INPUT : &str = "1090286-1131879,3259566-3404881,138124-175118,266204727-266361099,16765-24272,7657360692-7657593676,88857504-88926597,6869078-6903096,48444999-48532270,61427792-61580535,71-103,8077-10421,1920-2560,2-17,951-1259,34-50,28994-36978,1309-1822,9393918461-9393960770,89479-120899,834641-988077,5389718924-5389797353,34010076-34214499,5063-7100,607034-753348,19098586-19261191,125085556-125188689,39839-51927,3246-5037,174-260,439715-473176,187287-262190,348-535,58956-78301,4388160-4505757,512092-584994,13388753-13534387";
