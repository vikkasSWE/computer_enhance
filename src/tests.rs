#![cfg(test)]

use crate::dissassemble;

use paste::paste;

// #[test]
// fn listing_XX() {
//     let file = include_bytes!("../listings/listing_XX").to_vec();
//     let res = dissassemble(file);

//     assert!(res.is_ok());
//     assert_eq!(res.unwrap(), include_str!("../listings/listing_XX.asm"));
// }
macro_rules! create_test {
    ($num:expr) => {
        paste! {
            #[test]
            fn [<listing_ $num>]() {
                let file = include_bytes!(concat!("../listings/listing_", stringify!($num))).to_vec();
                let res = dissassemble(file);

                assert!(res.is_ok());
                assert_eq!(res.unwrap(), include_str!(concat!("../listings/listing_", stringify!($num), ".asm")));
            }
        }
    };
}

create_test!(37);
create_test!(38);
