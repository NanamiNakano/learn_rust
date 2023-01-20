pub(crate) fn _chars() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{} {} {}", c, z, heart_eyed_cat);
}

pub(crate) fn mem_size() {
    let x = '中';
    println!("字符'中'占用了{}字节的内存大小",std::mem::size_of_val(&x));
}
