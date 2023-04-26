use std::env;
//use std::mem::size_of;
//use std::num::IntErrorKind;


use rum::rumdis::UM;
use rum::rumload;

fn main() {
    let input = env::args().nth(1);
    let instructions = rumload::load(input.as_deref());
    let mut u_machine:UM = UM::new(instructions);

    // TODO TEMP
    // let my_val: u32 = 100; // lowercase 'd'
    // let nand: u32 = !(my_val & my_val);
    // println!("{my_val} {nand}");
    // TODO END TEMP



    //run
    u_machine.run();

    //run skeleton (RUMDump)
    //description code from the lab
    // u_machine.run_skel();

}
  
