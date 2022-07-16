use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};
// // add the following attributes to prepare your code for serialization and invocation on the blockchain
// // More built-in Rust attributes here: https://doc.rust-lang.org/reference/attributes.html#built-in-attributes-index

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Counter {
    val: i8,
}

#[near_bindgen]
impl Counter {
    pub fn get_num(&self) -> i8 {
        // 8bit符号付き整数を返す
        // ```
        // view関数として実行
        //counterはこれでいいの？
        // near view test_runacy.testnet get_num --accountId test_runacy.testnet
        // ```
        return self.val;
    }

    pub fn increment(&mut self) {
        // 増加する
        // ```
        // near call test_runacy.testnet increment --accountId test_runacy.testnet
        // ```
        self.val += 1;
        let log_message = format!("Incressed number to {}", self.val);
        env::log_str(&log_message);
        after_counter_change();
    }

    pub fn decrement(&mut self) {
        // ```
        // near call test_runacy.testnet decrement --accountId test_runacy.testnet
        // ```
        self.val -= 1;
        let log_message = format!("Decressed number to {}", self.val);
        env::log_str(&log_message);
        after_counter_change();
    }

    pub fn reset(&mut self) {
        self.val = 0;
        env::log_str("The number is ZERO.");
        after_counter_change();
    }

    pub fn bit_brute_force_search(&self) -> String {
        let n = self.val;
        if n < 1 || n > 20 {
            env::log_str("Please select value is over 1 to 19");
            return "".to_string();
        }

        let mut message = String::from("");
        for bit in 0..(1 << n) {
            let mut v: Vec<i8> = Vec::new();
            for i in 0..n {
                if (bit & (1 << i)) != 0 {
                    v.push(i);
                }
            }
            message += &String::from(format!("{} :", &bit));
            message += "{ ";
            for j in 0..v.len() {
                let val = v[j].to_string();
                message = message + &val + " ";
            }
            message += "}\n";
        }

        env::log_str(&message);
        after_counter_change();
        return message;
    }
}

fn after_counter_change() {
    env::log_str("Make sure you don't overflow, my friend.");
}

// *
//  * the rest of this file sets up unit tests
//  * to run these, the command will be:
//  * cargo test --package rust-counter-tutorial -- --nocapture
//  * Note: 'rust-counter-tutorial' comes from cargo.toml's 'name' key
//  */
#[cfg(test)]
mod test {
    use std::task::Context;

    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    #[test]
    fn get_num_test() {
        let user: AccountId = AccountId::new_unchecked("alice.test.net".to_string());
        let _context: VMContextBuilder = get_context(user.clone());
        let counter = Counter { val: 1 };
        let get_num: i8 = counter.get_num();
        assert_eq!(get_num, 1)
    }

    #[test]
    fn bit_brute_force_search_error_tst() {
        let user: AccountId = AccountId::new_unchecked("bob.test.net".to_string());
        let _context: VMContextBuilder = get_context(user.clone());
        let counter = Counter { val: 35 };
        let error: String = counter.bit_brute_force_search();
        assert_eq!(error, "".to_string());
    }
}
