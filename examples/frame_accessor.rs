mod examples_common;

use examples_common::*;

fn main() {
    let mut frame: Frame = [' ' as u8; 20 * 4];
    let mut accessor = Accessor::new(&mut frame, 20, 4);

    accessor[(1, 2)] = 'x' as u8;

    let mut accessor = FixedAccessor::new(&mut frame);

    accessor[(4, 3)] = 'y' as u8;

    println!("{}", display_frame(&accessor));

    // this should panic
    //accessor[(20, 0)] = '1' as u8;
}
