use std::io::{stdin, Read};
//use std::{char, process};
//use rayon::prelude::*;

type Umi = u32;

#[derive(Debug)]
pub struct Field{
    width: u32,
    lsb: u32,
}

//////////////////// registers \\\\\\\\\\\\\\\\\\\\
#[derive(Clone)]
pub struct Registers {
  pub registers: [u32; 8]
}
impl Registers {
  //register methods

  fn new() -> Registers{
    Registers { 
      registers: [0 as u32;8]
    }
  }

  //get and set
  fn get(&mut self,register:usize) -> &u32{
    &self.registers[register]
  }

  fn set(&mut self, register:usize, value:u32){
    self.registers[register] = value;
  }
}




//////////////////// Memory \\\\\\\\\\\\\\\\\\\\
//Will store and manage all memory segments
pub struct Memory {

    pub memory_segments:Vec<Vec<u32>>,
    //will store indexes of cleared memory locations
    pub cleared_memory_locations:Vec<u32>
}
impl Memory {

  fn new(instructions:Vec<u32>)  -> Memory{

    Memory { memory_segments: vec![instructions], cleared_memory_locations: Vec::new()}

  }

  // fn get_inst(&mut self,counter:usize) -> &u32{

  //   &self.memory_segments[0][counter]

  // }

}


//////////////////// UM Stuct \\\\\\\\\\\\\\\\\\\\


pub struct UM {
  pub register:Registers,
  pub memory:Memory,
  pub counter:usize
}

impl UM {

  pub fn new(instructions: Vec<u32>) -> UM{

    UM {register: Registers::new(), memory: Memory::new(instructions),counter:0}

  }

  // //print pseudo code for functions
  // pub fn run_skel(&mut self){
  //   for instruction in self.memory.memory_segments[0].clone() {
  //     println!("{}",disassemble_skel(instruction))
  //   }
  // }

  pub fn run(&mut self){
    //let i = 0;
    //for instruction in self.memory.memory_segments[0].clone() {

    //let mut inst_arr:[u32; self.memory.memory_segments[0].len()] = self.memory.memory_segments[0];
    
    loop {

      //let offset = self.counter;
      
      //let inst = *self.memory.get_inst(self.counter);
      let inst = self.memory.memory_segments[0][self.counter];

        //println!("{}",i);
        //print register data
        //println!("About to execute instruction {}", self.counter);
        //println!("Registers Before: {:?}", self.register.registers);
        //println!("Memory Segments Before: {:?}", self.memory.memory_segments);
        // dbg!(&self.memory.memory_segments);
        Self::disassemble(self,inst);
        // println!("Registers After:{:?}", self.register.registers);
        // println!("Memory Segments After: {:?}", self.memory.memory_segments);
        // println!("_____________________________________________________");
        //println!();
    
      
    }
  }

  fn disassemble(&mut self,inst: Umi) {

    match get(&OP, &inst) {

    //Conditional Move OP 0
    o if o == Opcode::CMov as u32 => {
      
      let c = *self.register.get(get(&RC, &inst) as usize);
      

      if c != 0{
      //register a
      let a_register = get(&RA, &inst) as usize;

      //register values
      let b = *self.register.get(get(&RB, &inst) as usize);
      

      
        self.register.set(a_register,b);
      }

      self.counter += 1;

    },

    //Segmented Load OP 1
    o if o == Opcode::Load as u32 => {

      //println!("r{} := m[r{}][r{}];", get(&RA, inst), get(&RB, inst), get(&RC, inst));

      //register a
      let a_register = get(&RA, &inst) as usize;

      //register values
      //let a = *self.register.get(get(&RA, inst) as usize);
      let b = *self.register.get(get(&RB, &inst) as usize);
      let c = *self.register.get(get(&RC, &inst) as usize);
      //dbg!(a,b,c);

      let value = self.memory.memory_segments[b as usize][c as usize];

      self.register.set(a_register, value);
      self.counter += 1;
    },

    //Segmented Store OP 2
    o if o == Opcode::Store as u32 => {

      //println!("m[r{}][r{}] := r{};", get(&RA, inst), get(&RB, inst), get(&RC, inst));

      //register c
      //let c_register = get(&RC, inst) as usize;

      //register values
      let a = *self.register.get(get(&RA, &inst) as usize);
      let b = *self.register.get(get(&RB, &inst) as usize);
      let c = *self.register.get(get(&RC, &inst) as usize);

      self.memory.memory_segments[a as usize][b as usize] = c;

      //self.register.set(c_register, value);

      // self.memory.memory_segments[get(&RA, inst) as usize]
      // [get(&RB, inst) as usize] 
      // = get(&RC, inst);

      self.counter += 1;

    },

    //Addition OP 3
    o if o == Opcode::Add as u32 => {


      //register values
      let b = *self.register.get(get(&RB, &inst) as usize);
      let c = *self.register.get(get(&RC, &inst) as usize);

      //register a
      let a_register = get(&RA, &inst) as usize;

      //wrapping does the mod 2^32
      let value = add(&b,&c);

      self.register.set(a_register,value);
      self.counter += 1;
    },

    //Multiplication OP 4
    o if o == Opcode::Mul as u32 => {


      //register values
      let b = *self.register.get(get(&RB, &inst) as usize);
      let c = *self.register.get(get(&RC, &inst) as usize);

      //register a
      let a_register = get(&RA, &inst) as usize;

      let value = mul(&b,&c);

      self.register.set(a_register, value);
      self.counter += 1;
    },

    //Division OP 5
    o if o == Opcode::Div as u32 => {

      //register values
      let b = *self.register.get(get(&RB, &inst) as usize);
      let c = *self.register.get(get(&RC, &inst) as usize);

      //register a
      let a_register = get(&RA, &inst) as usize;

      // let value = b / c;
      let value = div(&b,&c);

      self.register.set(a_register, value);
      self.counter += 1;

    },

    //Bitwise NAND OP 6
    o if o == Opcode::Nand as u32 => {

      let a_register = get(&RA, &inst) as usize;

      //values
      let b = *self.register.get(get(&RB, &inst) as usize);
      let c = *self.register.get(get(&RC, &inst) as usize);

      //nand values
      let value = bitwise_nand(b,c);


      self.register.set(a_register,value);
      self.counter += 1;

    },

    //Halt Exit Code 7
    o if o == Opcode::Halt as u32 => {

      std::process::exit(0);

    },

    //Map Segment OP 8
    o if o == Opcode::MapSegment as u32 => {
      let b_reg = get(&RB, &inst) as usize;

      let c_value = *self.register.get(get(&RC, &inst) as usize);

      let new_seg = vec![0;c_value as usize];
      
  
      if !self.memory.cleared_memory_locations.is_empty(){
        self.register.set(b_reg, self.memory.cleared_memory_locations.pop().unwrap());
        let b_value = *self.register.get(get(&RB, &inst) as usize);
        self.memory.memory_segments[b_value as usize] = new_seg;
      }

      else {
        
        self.memory.memory_segments.push(new_seg);
        self.register.set(b_reg, (self.memory.memory_segments.len() - 1) as u32);

      }
      

      self.counter += 1;
    },

    //Unmap Segment OP 9
    o if o == Opcode::UnmapSegment as u32 => {
      
      let c_value = *self.register.get(get(&RC, &inst) as usize);

      self.memory.cleared_memory_locations.push(c_value.try_into().unwrap());

      self.counter += 1;

    },

    //Output OP 10
    o if o == Opcode::Output as u32 => {

      let c = *self.register.get(get(&RC, &inst) as usize);


      print!("{}",c as u8 as char);
      self.counter += 1;
    },
  

    //Input OP 11

    o if o == Opcode::Input as u32 =>  {
      
      let mut buffer: [u8;1] = [0];
      let c = get(&RC, &inst) as usize;


      let character = stdin().read(&mut buffer);

      self.register.registers[c] = match character {
          Ok(1) => buffer[0] as u32,
          Ok(0) => u32::MAX,
          _ => panic!()
      };

      self.counter += 1;

    },

    //Load Program OP 12
    o if o == Opcode::LoadProgram as u32 => {

      let b_val = *self.register.get(get(&RB, &inst) as usize);

      let c_val = *self.register.get(get(&RC, &inst) as usize);


      //new stuff
      //do not clone mem seg 0
      
        if b_val != 0  {
          
          let value = self.memory.memory_segments[b_val as usize].clone();
          self.memory.memory_segments[0] = value;

        }
        
        let counter_value = c_val as usize;

        self.counter =  counter_value;

    },

    //Load Value OP 13
    o if o == Opcode::LoadValue as u32 => {
      
      
      self.register.set(get(&RL, &inst) as usize,
      get(&VL, &inst));
      self.counter += 1;
    },
  
    _ => {
    }
  }
  }

}


//////////////////// Instruction Parsing \\\\\\\\\\\\\\\\\\\\

static RA: Field = Field{width: 3, lsb: 6};
static RB: Field = Field{width: 3, lsb: 3};
static RC: Field = Field{width: 3, lsb: 0};
static RL: Field = Field{width: 3, lsb: 25};
static VL: Field = Field{width: 25, lsb: 0};
static OP: Field = Field{width: 4, lsb: 28};

#[inline]
fn mask(bits: u32) -> u32{ ( 1 << bits) - 1 }

#[inline]
pub fn get(field: &Field, instruction: &Umi) -> u32 {
  (instruction >> field.lsb) & mask(field.width)
}

pub fn op(instruction: Umi) -> u32 {
(instruction >> OP.lsb) & mask(OP.width)
} 


//////////////////// Helper Functions \\\\\\\\\\\\\\\\\\\\


//6
fn bitwise_nand(a:u32,b:u32) -> u32{
  !(a & b)
}

fn add(a:&u32,b:&u32) -> u32{
  a.wrapping_add(*b)
}

fn mul(a:&u32,b:&u32) -> u32{
  a.wrapping_mul(*b)
}

fn div(a:&u32,b:&u32) -> u32{
  a/b
}


#[derive(Debug, PartialEq, Copy, Clone)]
enum Opcode{
    CMov, //0
    Load, //1
    Store, //2
    Add, //3
    Mul, //4
    Div, //5
    Nand, //6 
    Halt, //7 
    MapSegment, //8 
    UnmapSegment, //9 
    Output, //10
    Input, //11
    LoadProgram, //12
    LoadValue, //13
}


// #[cfg(test)]
// mod tests {
//     use super::*;

//     //test registers
//     #[test]
//     fn test_registers_get_set(){
//       let mut register = Registers::new();

//       let value = 2;
//       let reg = 7;
//       register.set(reg, value);
//       println!("Set Register {} to: {}",reg,value);


//       let test = register.get(reg);
//       println!("Attempt to get Register {}'s value returns: {}",reg,test);

//       assert_eq!(register.get(reg),test);
//     }

//     #[test]
//     fn test_add(){

//       let one:u32 = 1;
//       let two:u32 = 2;

//       assert_eq!(add(one,two),3);

//     }

//     #[test]
//     fn test_mul(){

//       let two:u32 = 2;
//       let three:u32 = 3;

//       assert_eq!(mul(two,three),6);
      
//     }

//     #[test]
//     fn test_div(){

//       let two:u32 = 2;
//       let six:u32 = 6;

//       assert_eq!(div(six,two),3);
      
//     }

//     #[test]
//     fn test_nand() {
        
//       let a: u32 = 0b11111111000000001111111100000000;
//       let b: u32 = 0b11111111111111110000000000000000;
//       let e: u32 = 0b00000000111111111111111111111111;

//       assert_eq!(bitwise_nand(a, b), e);

//     }

// }
