#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_string_new(b: &mut Bencher) {
        b.iter(|| String::new());
    }

    #[bench]
    fn bench_to_string(b: &mut Bencher) {
        b.iter(|| "".to_string());
    }
}
