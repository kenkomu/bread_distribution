use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap};

use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen};
// bread whole
// different types
// different shops
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Debug)]
struct Bread {
    name: String,
    date_created: String,
    code: u16,
}
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
struct Shop {
    id: u16,
    name: String,
    location: String,
    inventory_size: u8,        // 20
    minumum_invetory_size: u8, //25
    bread_amount: UnorderedMap<String, u8>,
}

impl Default for Shop {
    fn default() -> Self {
        Shop {
            id: 0,
            name: "".to_string(),
            location: "".to_string(),
            inventory_size: 0,        // 20
            minumum_invetory_size: 0, //25
            bread_amount: UnorderedMap::new(b"s".to_vec()),
        }
    }
}
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct BreadDistributor {
    shops: Vec<Shop>,
    breads: Vec<Bread>,
}

// impl Default for BreadDistributor {
//     fn default() -> Self {
//         BreadDistributor {
//                 shops: Vector::new(b"r".to_vec()),
//                 breads: Vector::new(b"r".to_vec()),
//             }
//     }
// }

#[near_bindgen]
impl BreadDistributor {
    #[init]
    pub fn new() -> Self {
        let shop: Vec<Shop> = vec![];// Vector::new(b"r".to_vec());
        let mut bread: Vec<Bread> =vec![]; // Vector::new(b"r".to_vec());

        let kenblest = Bread {
            name: "kenblest".to_string(),
            date_created: "15/07/2022".to_string(),
            code: 123,
        };
        bread.push(kenblest);

        BreadDistributor {
            shops: shop,
            breads: bread,
        }
    }

    pub fn add_bread(&mut self, name: String, code: u16, date: String) {
        let br = Bread {
            name: name,
            code: code,
            date_created: date,
        };
        self.breads.push(br);
    }

    pub fn supply_bread(&mut self, shop_id: u16, name: String, amount: u8) {
        let mut shop_exits: bool = false;
        self.shops.iter_mut().for_each(|shop| {
            if shop.id == shop_id {
                shop_exits = true;

                shop.inventory_size += amount;

                let bread_current = shop.bread_amount.get(&name);

                match bread_current {
                    Some(n) => {
                        let total = n + amount;
                        shop.bread_amount.insert(&name, &total); //= n+amount;
                    }
                    None => {
                        shop.bread_amount.insert(&name, &amount);
                    }
                }
            }
        });
    }
    
    pub fn shop_sell(&mut self, id_shop: u16, name: String, quantity: u8) {
        self.shops.iter_mut().for_each(|shop: &mut Shop| {
        // for (index, shop) in &mut self.shops.iter().enumerate() {
            if shop.id == id_shop {
                let bread_current = shop.bread_amount.get(&name);

                match bread_current {
                    Some(n) => {
                        // shop
                        // let tmp_shop: Shop = shop;

                        let total = n - quantity;
                        shop.bread_amount.insert(&name, &total); //= n+amount;

                        // let tmp_ind: u64=   index.try_into().expect("msg");
                        // self.shops.replace( tmp_ind, &tmp_shop);
                    }
                    None => {
                        let my_str = format!("There is no bread of type {}", name);
                        env::log(my_str.as_bytes());
                    }
                }
            }

        // }

        });
    }
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */
// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    // TESTS HERE
    #[test]
    fn add_course() {
        let mut bread_dist = BreadDistributor::new();

        bread_dist.add_bread("broadways".to_owned(), 111, "date".to_owned());
        bread_dist.add_bread("supa loaf".to_owned(), 113, "date".to_owned());
        assert_eq!(bread_dist.breads.len(), 3);
       
    }

}