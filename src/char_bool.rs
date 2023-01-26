pub(crate) fn _chars() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{} {} {}", c, z, heart_eyed_cat);
}

pub(crate) fn _mem_size() {
    let x = '中';
    println!("字符'中'占用了{}字节的内存大小",std::mem::size_of_val(&x));
}

pub(crate) fn bools() {
    let t = true;
    let f: bool = false; // 使用类型标注,显式指定f的类型
    if f {
        println!("这是段毫无意义的代码");
    }else {
        println!("这是段有意义的代码");
    }
    println!("{} {}", t, f);
}