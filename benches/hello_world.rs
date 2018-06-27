#![feature(test)]
extern crate brainfuck;
extern crate test;

use brainfuck::prelude::*;
use std::io;
use test::Bencher;

struct BlackHole;

impl io::Write for BlackHole {
    fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
        Ok(0)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl io::Read for BlackHole {
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
        Ok(0)
    }
}

static PROGRAM: &str = "++++++++++[>+>+++>+++++++>++++++++++<<<<-]>>>++.>+.+++++++..+++.<<++.>+++++++++++++++.>.+++.------.--------.<<+.<.";

#[bench]
fn hello_world_compile(bencher: &mut Bencher) {
    bencher.iter(|| {
        let _ = test::black_box(PROGRAM).into_program();
    });
}

#[bench]
fn hello_world_execute(bencher: &mut Bencher) {
    bencher.iter(|| {
        let mut process = Process::new(PROGRAM, 1024, BlackHole, BlackHole);
        process.execute().unwrap();
    });
}


#[bench]
fn u32_xor_u32_x1000(bencher: &mut Bencher) {
    let a: u32 = test::black_box(666666666);
    let b: u32 = test::black_box(233333333);
    bencher.iter(|| {
        for _ in 1..1000 {
            let _ = test::black_box(a ^ b);
        }
    });
}
