use std::fs;
use std::num::ParseIntError;


#[derive(Debug)]
enum Op{
    Mult,
    Add,
}

#[derive(Debug)]
struct Accumulator{
    tot : u64,
    op : Op,
}


impl Accumulator{

    fn from(s: String) -> Self{ 
        match s.as_str() {
            "*" => Self {tot: 1, op: Op::Mult},
            "+" => Self {tot: 0, op: Op::Add},
            _ => todo!("passed s {}", s)
        }
    }

    fn acc(&mut self, val: u64){
        match self.op {
            Op::Mult => self.tot *= val,
            Op::Add => self.tot += val,
        }
    }

    fn tot(&self) -> u64 { self.tot }
}

fn main() {
    let mut lines: Vec<Vec<String>> = fs::read_to_string("./real_input")
    .unwrap()
    .lines()
    .map(String::from)
    .collect::<Vec<String>>()
    .into_iter()
    .map(|s| s.split(' ').filter(|s| *s != "").map(String::from).collect::<Vec<String>>())
    .collect();

 
    let mut accs: Vec<Accumulator> = lines.pop().unwrap().into_iter().map(Accumulator::from).collect();
    let lines : Vec<Vec<u64>> = lines.into_iter()
    .map(|v| v.iter().map(|s| s.parse::<u64>().unwrap()).collect())
    .collect();

    for l in &lines{
        println!("{:?}", l);
        for (acc, v) in accs.iter_mut().zip(l.iter().copied())
        {
            acc.acc(v);
        }
    }

    let total: u64 = accs.iter().map(|a| a.tot()).sum();
    println!("{:?}", accs);
    println!("total {}", total);

}
