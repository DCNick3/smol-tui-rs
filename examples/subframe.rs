mod examples_common;

use examples_common::*;

use smol_tui_rs::FrameAccessorTrait;

fn main() {
    let mut frame: Frame = [' ' as u8; 20 * 4];
    let mut accessor = FixedAccessor::new(&mut frame);

    let mut subaccessor = accessor.subframe::<10, 0, 10, 2>();
    subaccessor.fill('X' as u8);

    let mut subaccessor = accessor.subframe::<0, 2, 10, 2>();
    subaccessor.fill('Y' as u8);

    println!("{}", display_frame(&accessor));
}
