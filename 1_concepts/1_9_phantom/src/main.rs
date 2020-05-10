use std::marker::PhantomData;
trait HasFacts {
    fn name() -> &'static str;
    fn random_fact() -> String;
}
#[derive(Debug)]
struct Fact<T>(PhantomData<T>)
where
    T: HasFacts;
impl<T> Fact<T>
where
    T: HasFacts,
{
    pub fn new() -> Self {
        Fact(PhantomData)
    }
    pub fn fact(&self) -> String {
        format!("Fact about {}: {}", T::name(), T::random_fact())
    }
}

impl<T> HasFacts for Fact<T>
where
    T: HasFacts,
{
    fn name() -> &'static str {
        "Recursive Fact!!!"
    }
    fn random_fact() -> String {
        Fact::<T>(PhantomData).fact()
    }
}

impl<T> HasFacts for Vec<T> {
    fn name() -> &'static str {
        "Vec"
    }
    fn random_fact() -> String {
        let rnd: u8 = rand::random();
        match rnd % 2 {
            0 => "Vec is heap-allocated.".to_owned(),
            1 => "Vec may re-allocate on growing.".to_owned(),
            _ => panic!(),
        }
    }
}

fn main() {
    let f = Fact::<Fact<Fact<Vec<i32>>>>::new();
    println!("{}", f.fact());
    println!("{}", f.fact());
    println!("{}", f.fact());
    println!("{}", f.fact());
    println!("{}", f.fact());
    println!("{}", f.fact());
}
