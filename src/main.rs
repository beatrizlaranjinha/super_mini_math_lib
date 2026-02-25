use std::error::Error;

use super_mini_math_lib::{Number, bit_add, eea, mdv, power, zk_guillou_quisquater::State};

fn main() -> Result<(), Box<dyn Error>> {
    /* let mut num = Number { value: [false; 32] };
    num.value[0] = true; //1
    let mut num2 = Number { value: [false; 32] };
    num2.value[1] = true; //2
    let rest = bit_add(num, num2); // 1 + 2
    println!("{:?}", rest); */
    let power = power(3, 13);
    println!("{:?}", power);
    let mdv = mdv(48, 18);
    println!("{:?}", mdv);
    let eea = eea(7, 26);
    println!("{:?}", eea);

    let state = State {
        p: 4,
        q: 59,
        v: 7,
        identity: 13,
        nonce: 59,
        challange: 5,
    };

    state.guillou_quisquater()?;

    Ok(())
}
