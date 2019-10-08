// Copyright (c) BohuTANG
// Code is licensed with BSD

#[cfg(test)]
pub mod tests {
    use crate::field;

    #[test]
    fn fields_fp37_test() {
        let fp = field::Field::new(12);

        for i in 0..13 {
            let add = fp.add(5, i);
            println!("add.element:{}", add);
        }

        for i in 0..13 {
            let mul = fp.mul(5, i);
            println!("mul.element:{}", mul);
        }
    }
}
