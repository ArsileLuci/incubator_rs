use money::{Coin, Money};
use product::Product;
use std::any::TypeId;
use trade::HasValue;
mod trade {
    pub trait HasValue {
        fn value(&self) -> u64;
    }
}

#[derive(Debug)]
enum InsertionError {
    TooMuchItems,
    NoEmptySpace,
}

#[derive(Debug)]
enum PurchaseError {
    NotEnoughProduct,
    CantGiveRest,
    NoItemOfThisType,
    NotEnoughMoney,
}

mod money {
    use crate::trade::HasValue;
    pub struct Amount(u32);
    impl Into<Amount> for u32 {
        fn into(self) -> Amount {
            Amount(self)
        }
    }
    impl Into<u32> for Amount {
        fn into(self) -> u32 {
            self.0
        }
    }
    pub enum Coin {
        One,
        Two,
        Five,
        Ten,
        Twenty,
        Fifty,
    }

    impl HasValue for Coin {
        fn value(&self) -> u64 {
            match self {
                One => 1,
                Two => 2,
                Five => 5,
                Ten => 10,
                Twenty => 20,
                Fifty => 50,
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct Money {
        one_count: u64,
        two_count: u64,
        five_count: u64,
        ten_count: u64,
        twenty_count: u64,
        fifty_count: u64,
    }
    impl HasValue for Money {
        fn value(&self) -> u64 {
            let mut value: u64 = 0;
            value += self.one_count * Coin::One.value();
            value += self.two_count * Coin::Two.value();
            value += self.five_count * Coin::Five.value();
            value += self.ten_count * Coin::Ten.value();
            value += self.twenty_count * Coin::Twenty.value();
            value += self.fifty_count * Coin::Fifty.value();
            value
        }
    }

    impl Money {
        pub fn new() -> Self {
            Money {
                one_count: 0,
                two_count: 0,
                five_count: 0,
                ten_count: 0,
                twenty_count: 0,
                fifty_count: 0,
            }
        }

        pub fn add(&mut self, coin: Coin) {
            match coin {
                Coin::One => self.one_count += 1,
                Coin::Two => self.two_count += 1,
                Coin::Five => self.five_count += 1,
                Coin::Ten => self.ten_count += 1,
                Coin::Twenty => self.twenty_count += 1,
                Coin::Fifty => self.fifty_count += 1,
            }
        }

        pub fn add_vec<T: IntoIterator<Item = Coin>>(&mut self, coins: T) {
            for coin in coins {
                self.add(coin)
            }
        }

        pub fn merge(&mut self, new: Money) {
            self.one_count += new.one_count;
            self.two_count += new.two_count;
            self.five_count += new.five_count;
            self.ten_count += new.ten_count;
            self.twenty_count += new.twenty_count;
            self.fifty_count += new.fifty_count;
        }

        pub fn try_give_rest(&self, user_input: &Self, value: u64) -> Option<(Money, Money)> {
            let mut _money = self.clone();
            let u_input = user_input.clone();
            let mut user_money = Money::new();
            _money.merge(u_input);
            let mut lock_level = 0;
            let mut split = Money::split_money(value, lock_level);
            while lock_level <= 5 {
                Money::try_substract(&mut _money, &mut split, &mut user_money);
                lock_level += 1;
                if split.value() == 0 {
                    return Some((_money, user_money));
                }
                split = Money::split_money(split.value(), lock_level);
            }
            None
        }

        fn try_substract(left: &mut Money, right: &mut Money, store: &mut Money) {
            let mut sub = std::cmp::min(left.fifty_count, right.fifty_count);
            left.fifty_count -= sub;
            right.fifty_count -= sub;
            store.fifty_count += sub;
            sub = std::cmp::min(left.twenty_count, right.twenty_count);
            left.twenty_count -= sub;
            right.twenty_count -= sub;
            store.twenty_count += sub;
            sub = std::cmp::min(left.ten_count, right.ten_count);
            left.ten_count -= sub;
            right.ten_count -= sub;
            store.ten_count += sub;
            sub = std::cmp::min(left.five_count, right.five_count);
            left.five_count -= sub;
            right.five_count -= sub;
            store.five_count += sub;
            sub = std::cmp::min(left.two_count, right.two_count);
            left.two_count -= sub;
            right.two_count -= sub;
            store.two_count += sub;
            sub = std::cmp::min(left.one_count, right.one_count);
            left.one_count -= sub;
            right.one_count -= sub;
            store.one_count += sub;
        }

        fn split_money(value: u64, lock_level: u8) -> Money {
            let mut rest = value;
            let fifty_count;
            let twenty_count;
            let ten_count;
            let five_count;
            let two_count;
            if lock_level <= 1 {
                fifty_count = rest / Coin::Fifty.value();
                rest = rest % Coin::Fifty.value();
            } else {
                fifty_count = 0;
            }
            if lock_level <= 2 {
                twenty_count = rest / Coin::Twenty.value();
                rest = rest % Coin::Twenty.value();
            } else {
                twenty_count = 0;
            }
            if lock_level <= 3 {
                ten_count = rest / Coin::Ten.value();
                rest = rest % Coin::Ten.value();
            } else {
                ten_count = 0;
            }
            if lock_level <= 4 {
                five_count = rest / Coin::Five.value();
                rest = rest % Coin::Five.value();
            } else {
                five_count = 0;
            }
            if lock_level < 5 {
                two_count = rest / Coin::Two.value();
                rest = rest % Coin::Two.value();
            } else {
                two_count = 0;
            }
            let one_count = rest;
            Money {
                fifty_count,
                twenty_count,
                ten_count,
                five_count,
                two_count,
                one_count,
            }
        }
    }
}

mod product {
    use crate::trade;
    use std::any::Any;
    pub trait Product: trade::HasValue + Any {
        fn name(&self) -> &'static str;
    }
    #[derive(Hash, Debug)]
    //Amount of items remaining
    pub struct Amount(pub u32);
    impl From<u32> for Amount {
        fn from(val: u32) -> Self {
            Amount(val)
        }
    }
}

struct Stock {
    stock_capacity: RowCount,
    in_stock_capacity: InRowCount,
    product_storage: std::collections::HashMap<TypeId, Vec<Box<dyn Product>>>,
}

impl Stock {
    pub fn new<T1: Into<RowCount>, T2: Into<InRowCount>>(
        stock_capacity: T1,
        in_stock_capacity: T2,
    ) -> Self {
        Stock {
            stock_capacity: stock_capacity.into(),
            in_stock_capacity: in_stock_capacity.into(),
            product_storage: std::collections::HashMap::new(),
        }
    }

    pub fn display_stock(&self) {
        self.product_storage.iter().for_each(|kvp| {
            let product = kvp.1;
            println!(
                "product:{}, price:{}, in stock:{:?};",
                product[0].name(),
                product[0].value(),
                product.len()
            )
        })
    }

    pub fn insert<T: Product>(&mut self, items: Vec<Box<T>>) -> Result<(), InsertionError> {
        let tid = TypeId::of::<T>();
        match self.product_storage.get_mut(&tid) {
            Some(product) => {
                if product.len() + items.len() > self.in_stock_capacity.0 as usize {
                    return Err(InsertionError::TooMuchItems);
                }
                for item in items {
                    product.push(item)
                }
                Ok(())
            }
            None => {
                if self.product_storage.len() == self.stock_capacity.0 as usize {
                    return Err(InsertionError::NoEmptySpace);
                }
                if items.len() > self.in_stock_capacity.0 as usize {
                    return Err(InsertionError::TooMuchItems);
                }
                let mut new_vec: Vec<Box<dyn Product>> = Vec::new();
                for item in items {
                    new_vec.push(item);
                }
                self.product_storage.insert(TypeId::of::<T>(), new_vec);
                return Ok(());
            }
        }
    }

    pub fn get_price_and_count<T: Product>(&self) -> Option<(u64, usize)> {
        let tid = TypeId::of::<T>();
        self.product_storage
            .get(&tid)
            .map(|product| (product[0].value(), product.len()))
    }

    pub fn purchase<T: Product, TAmmount: Into<product::Amount>>(
        &mut self,
        amount: TAmmount,
    ) -> Vec<Box<dyn Product>> {
        let tid = TypeId::of::<T>();
        let count = amount.into().0 as usize;
        let items = self.product_storage.get_mut(&tid).unwrap();

        let vec = items.drain(0..count).collect::<Vec<_>>();
        if items.is_empty() {
            self.product_storage.remove(&tid);
        }
        vec
    }
}
struct VendingMachine {
    stock: Stock,
    rest: Money,
}
#[derive(Clone, Copy)]
struct RowCount(u8);
#[derive(Clone, Copy)]
struct InRowCount(u8);
impl VendingMachine {
    pub fn display_stock(&self) {
        self.stock.display_stock()
    }
    pub fn new(stock_capacity: RowCount, in_stock_capacity: InRowCount) -> Self {
        VendingMachine {
            stock: Stock::new(stock_capacity, in_stock_capacity),
            rest: Money::new(),
        }
    }

    pub fn insert<T: Product, TCollection: IntoIterator<Item = T>>(
        &mut self,
        items: TCollection,
    ) -> Result<(), InsertionError> {
        let collection = items
            .into_iter()
            .map(|item| Box::new(item))
            .collect::<Vec<_>>();
        self.stock.insert(collection)
    }

    pub fn add_money(&mut self, money: money::Money) {
        self.rest.merge(money);
    }

    pub fn purchase<T: Product, TAmmount: Into<product::Amount>>(
        &mut self,
        money: money::Money,
        amount: TAmmount,
    ) -> Result<(Vec<Box<dyn Product>>, Money), (Money, PurchaseError)> {
        let items_amount = amount.into();
        let res = self.stock.get_price_and_count::<T>().map_or(
            Err((money.clone(), PurchaseError::NoItemOfThisType)),
            |price_and_count| {
                let price = price_and_count.0;
                let count = price_and_count.1;
                if count < items_amount.0 as usize {
                    return Err((money, PurchaseError::NotEnoughProduct));
                }
                if items_amount.0 as u64 * price > money.value() {
                    return Err((money, PurchaseError::NotEnoughMoney));
                }
                let required_rest = &money.value() - items_amount.0 as u64 * price;
                match self.rest.try_give_rest(&money, required_rest) {
                    Some((machine_rest, user_rest)) => {
                        self.rest = machine_rest;
                        return Ok((self.stock.purchase::<T, _>(items_amount), user_rest));
                    }
                    None => return Err((money, PurchaseError::CantGiveRest)),
                }
            },
        );
        res
    }
}

mod implementation {
    use crate::product::Product;
    use crate::trade::HasValue;
    pub struct Snickers {}
    impl Product for Snickers {
        fn name(&self) -> &'static str {
            "Snickers"
        }
    }
    impl HasValue for Snickers {
        fn value(&self) -> u64 {
            23
        }
    }

    pub struct Cola {}
    impl HasValue for Cola {
        fn value(&self) -> u64 {
            47
        }
    }
    impl Product for Cola {
        fn name(&self) -> &'static str {
            "Cola"
        }
    }
}

fn main() {
    let mut vm = VendingMachine::new(RowCount(1), InRowCount(12));
    println!(
        "{:?}",
        vm.insert(vec![
            implementation::Snickers {},
            implementation::Snickers {},
            implementation::Snickers {},
            implementation::Snickers {},
        ])
    );
    println!(
        "{:?}",
        vm.insert(vec![
            implementation::Cola {},
            implementation::Cola {},
            implementation::Cola {},
            implementation::Cola {},
        ])
    );
    vm.display_stock();
    let mut money = Money::new();
    let vec = vec![
        Coin::Twenty,
        Coin::Twenty,
        Coin::Twenty,
        Coin::Twenty,
        Coin::Ten,
        Coin::Ten,
        Coin::Ten,
        Coin::Ten,
        Coin::Five,
        Coin::Five,
        Coin::Five,
        Coin::Five,
        Coin::Five,
        Coin::Fifty,
        Coin::Fifty,
        Coin::Fifty,
        Coin::Fifty,
        Coin::Fifty,
        Coin::One,
        Coin::One,
        Coin::One,
        Coin::One,
    ];
    let mut machine_money = Money::new();
    machine_money.add_vec(vec);
    vm.add_money(machine_money);
    money.add_vec(vec![Coin::Fifty, Coin::Fifty]);
    println!("{}", money.value());
    match vm.purchase::<implementation::Cola, _>(money, 2) {
        Ok(_) => println!("ok"),
        Err(err) => println!("{:?}", err),
    }
}
