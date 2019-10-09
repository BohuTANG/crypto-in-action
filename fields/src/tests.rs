// Copyright (c) BohuTANG
// Code is licensed with BSD

#[cfg(test)]
pub mod tests {
    use crate::field;

    #[test]
    fn fields_field_test() {
        let fp37 = field::Field::new(37);
        // Add test.
        {
            let x = fp37.add(18, 20);
            assert_eq!(x, 1);
        }
        {
            let x = fp37.add(18, 19);
            assert_eq!(x, 0);
        }

        // Sub test.
        {
            let x = fp37.sub(14, 19);
            assert_eq!(x, 32);
        }

        // Mul test.
        {
            let x = fp37.mul(18, 19);
            assert_eq!(x, 9);
        }
    }

    #[test]
    fn fields_f12_test() {
        let p = 12;
        let fp = field::Field::new(p);

        let mut vec = Vec::new();
        for i in 0..2 * p {
            let sum = fp.add(4, i);
            vec.push(sum);
        }
        println!("f12.add.element:\n{:?}", vec);
        vec.clear();

        for i in 0..2 * p {
            let product = fp.mul(4, i);
            vec.push(product);
        }
        println!("f12.mul.element:\n{:?}", vec);
        vec.clear();
    }

    #[test]
    fn fields_f37_test() {
        let p = 37;
        let fp = field::Field::new(p);

        let mut vec = Vec::new();
        for i in 0..2 * p {
            let sum = fp.add(4, i);
            vec.push(sum);
        }
        println!("f37.add.element:\n{:?}", vec);
        vec.clear();

        for i in 0..2 * p {
            let product = fp.mul(4, i);
            vec.push(product);
        }
        println!("f37.mul.element:\n{:?}", vec);
        vec.clear();
    }

    #[test]
    fn fields_legendre_symbol_test() {
        let p = 31;
        let fp = field::Field::new(p);
        for i in 1..31 {
            let x = fp.legendre_symbol(i);
            println!("legendre.check,i:{},result:{:?}", i, x);
        }
    }

    #[test]
    fn fields_sqrt_test() {
        {
            let p = 31;
            let fp = field::Field::new(p);
            for i in 0..60 {
                let sqrt = fp.sqrt(i);
                if sqrt != None {
                    println!("{},sqrt:{:?}", i, sqrt.unwrap());
                }
            }
            assert_eq!(fp.sqrt(0), Some(0));
            assert_eq!(fp.sqrt(2), Some(8));
            assert_eq!(fp.sqrt(3), None);
            assert_eq!(fp.sqrt(33), Some(8));
        }
    }
}
