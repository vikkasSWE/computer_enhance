#![cfg(test)]

use std::{
    env::temp_dir,
    fs,
    io::{Read, Write},
    process::{Command, Stdio},
};

use crate::dissassemble;

use paste::paste;

pub fn get_file_as_byte_vec(filename: &str) -> Vec<u8> {
    let mut f = fs::File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}

// #[test]
// fn listing_39() {
//     let file = include_bytes!("../listings/listing_39").to_vec();
//     let res = dissassemble(file);

//     assert!(res.is_ok());

//     fs::remove_file("listings/listing_39_out.asm").ok();
//     let mut out = File::create("listings/listing_39_out.asm").unwrap();
//     out.write_all(res.unwrap().as_bytes()).ok();

//     Command::new("nasm")
//         .args(&["listings/listing_39_out.asm", "-o temp.out"])
//         .stdout(Stdio::inherit())
//         .stderr(Stdio::inherit())
//         .output()
//         .unwrap(); // TODO: Change to actual output later...

//     assert_eq!(
//         get_file_as_byte_vec("temp.out"),
//         include_bytes!("../listings/listing_39")
//     );
// }
macro_rules! create_test {
    ($num:expr) => {
        paste! {
            #[test]
            fn [<listing_ $num>]() {
                let file = include_bytes!(concat!("../listings/listing_", stringify!($num))).to_vec();

                let res = dissassemble(file).expect("Failed to disassemble");

                fs::create_dir("out").ok();
                let out_path = format!("out/listing_{}_out.asm", $num);
                fs::remove_file(&out_path).ok();
                let mut out = fs::File::create(&out_path).expect("Failed to create output file");
                out.write_all(res.as_bytes())
                    .expect("Failed to write to output file");

                let temp = temp_dir().join(&format!("temp_{}.out", $num)).to_string_lossy().to_string();

                Command::new("nasm")
                    .args(&[&out_path, &format!("-o {}", &temp)])
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .output()
                    .expect("NASM command failed");

                let binary = get_file_as_byte_vec(&temp);
                fs::remove_file(&temp).ok();

                assert_eq!(
                    binary,
                    include_bytes!(concat!("../listings/listing_", stringify!($num)))
                );
            }
        }
    };
}

create_test!(37);
create_test!(38);
create_test!(39);
