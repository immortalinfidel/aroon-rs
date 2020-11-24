#![feature(external_doc)]
mod aroon;
mod aroonosc;

pub use aroon::AROON;
pub use aroonosc::AROONOSC;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
