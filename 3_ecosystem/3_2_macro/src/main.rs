use proc_btmap::btmap_proc;

#[macro_export]
macro_rules! btmap {
    ($ ($key:expr,$value:expr),*) => {
        {
            let mut tmp = std::collections::BTreeMap::new();
            $(
                tmp.insert($key, $value);
            )*
            tmp
        }
    };
}

fn main() {
    let map = btmap!(1, "10", 2, "20", 3, "30");
    println!("{:?}", map.get(&1));
    println!("{:?}", map.get(&2));
    println!("{:?}", map.get(&3));

    let x = btmap_proc!(1, "10", 2, "20", 3, "30");
    println!("{:?}", x.get(&1));
    println!("{:?}", x.get(&2));
    println!("{:?}", x.get(&3));
}
