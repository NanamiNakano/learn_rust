// wrong code
// fn main() {
//     let v = {
//         let mut x = 1;
//         x += 2
//     };
//
//     assert_eq!(v, 3);
// }

// correct code
fn _main0() {
    let v = {
        let mut x = 1;
        x += 2;
        x
    };

    assert_eq!(v, 3);
}

fn _main1() {
    let v = {
        let x = 1;
        x + 2
    };

    assert_eq!(v, 3);
}

fn _main2() {
    let v = {
        let mut _x = 1;
        _x += 2
    };

    assert_eq!(v, ());
}

// wrong code
// fn main() {
//     let v = (let x = 3);
//
//     assert!(v == 3);
// }

// correct code
fn _main3() {
    let v = {
        let x = 3;
        x
    };

    assert_eq!(v, 3);
}