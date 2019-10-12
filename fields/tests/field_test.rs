// Copyright (c) BohuTANG
// Code is licensed with BSD

#[cfg(test)]
pub mod tests {
    use fields::field;

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

        // Exp test.
        {
            let x = fp37.exp(18, 11);
            assert_eq!(x, 17);
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
        /*
        f12.add.element:
        [4, 5, 6, 7, 8, 9, 10, 11, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 0, 1, 2, 3]
        */
        vec.clear();

        for i in 0..2 * p {
            let product = fp.mul(4, i);
            vec.push(product);
        }
        println!("f12.mul.element:\n{:?}", vec);
        /*
        f12.mul.element:
        [0, 4, 8, 0, 4, 8, 0, 4, 8, 0, 4, 8, 0, 4, 8, 0, 4, 8, 0, 4, 8, 0, 4, 8]
        */
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
        /*
        f37.add.element:
        [4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 0, 1, 2, 3]
        */
        vec.clear();

        for i in 0..2 * p {
            let product = fp.mul(4, i);
            vec.push(product);
        }
        println!("f37.mul.element:\n{:?}", vec);
        /*
        f37.mul.element:
        [0, 4, 8, 12, 16, 20, 24, 28, 32, 36, 3, 7, 11, 15, 19, 23, 27, 31, 35, 2, 6, 10, 14, 18, 22, 26, 30, 34, 1, 5, 9, 13, 17, 21, 25, 29, 33, 0, 4, 8, 12, 16, 20, 24, 28, 32, 36, 3, 7, 11, 15, 19, 23, 27, 31, 35, 2, 6, 10, 14, 18, 22, 26, 30, 34, 1, 5, 9, 13, 17, 21, 25, 29, 33]
        */
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
        /*
        legendre.check,i:1,result:1
        legendre.check,i:2,result:1
        legendre.check,i:3,result:-1
        legendre.check,i:4,result:1
        legendre.check,i:5,result:1
        legendre.check,i:6,result:-1
        legendre.check,i:7,result:1
        legendre.check,i:8,result:1
        legendre.check,i:9,result:1
        legendre.check,i:10,result:1
        legendre.check,i:11,result:-1
        legendre.check,i:12,result:-1
        legendre.check,i:13,result:-1
        legendre.check,i:14,result:1
        legendre.check,i:15,result:-1
        legendre.check,i:16,result:1
        legendre.check,i:17,result:-1
        legendre.check,i:18,result:1
        legendre.check,i:19,result:1
        legendre.check,i:20,result:1
        legendre.check,i:21,result:-1
        legendre.check,i:22,result:-1
        legendre.check,i:23,result:-1
        legendre.check,i:24,result:-1
        legendre.check,i:25,result:1
        legendre.check,i:26,result:-1
        legendre.check,i:27,result:-1
        legendre.check,i:28,result:1
        legendre.check,i:29,result:-1
        legendre.check,i:30,result:-1
        */
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
            /*
            0,sqrt:0
            1,sqrt:1
            2,sqrt:8
            4,sqrt:2
            5,sqrt:25
            7,sqrt:10
            8,sqrt:16
            9,sqrt:28
            10,sqrt:14
            14,sqrt:18
            16,sqrt:4
            18,sqrt:7
            19,sqrt:9
            20,sqrt:19
            25,sqrt:5
            28,sqrt:20
            31,sqrt:0
            32,sqrt:1
            33,sqrt:8
            35,sqrt:2
            36,sqrt:25
            38,sqrt:10
            39,sqrt:16
            40,sqrt:28
            41,sqrt:14
            45,sqrt:18
            47,sqrt:4
            49,sqrt:7
            50,sqrt:9
            51,sqrt:19
            56,sqrt:5
            59,sqrt:20
            */
            assert_eq!(fp.sqrt(0), Some(0));
            assert_eq!(fp.sqrt(2), Some(8));
            assert_eq!(fp.sqrt(3), None);
            assert_eq!(fp.sqrt(33), Some(8));
        }
    }
}
