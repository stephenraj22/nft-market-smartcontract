// Author: Stephen Raj D

use near_sdk::collections::{LookupMap, UnorderedSet};
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    log,
    serde::{Deserialize, Serialize},
    AccountId, PanicOnDefault, Promise,
};
use near_sdk::{env, near_bindgen};
use std::collections::LinkedList;


#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Nft {
    uri: String,
    category: String,
    desc:String,
    price:u64,
    forSale:bool
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Creations{
    nfts:Vec<Nft>,
}


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner_id: AccountId,
    creators: LookupMap<AccountId, Creations>,
    creators_earnings: LookupMap<AccountId, u64>,
    creatorIds:UnorderedSet<AccountId>,
}

#[near_bindgen]
impl Contract {
    // ADD CONTRACT METHODS HERE
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        Self {
            owner_id,
            creators: LookupMap::new(b"c"),
            creators_earnings:LookupMap::new(b"e"),
            creatorIds: UnorderedSet::new(b"i"),
        }
    }

    #[payable]
    pub fn make_creator(
        &mut self,
        index:u16,
        owner:AccountId,
        buyer:AccountId
    ){
        if self.creators.get(&buyer).is_none(){
            let account_copy = buyer.clone();
            self.creators.insert(
                &buyer,
                &Creations {
                    nfts:Vec::new(),
                }
            );
            self.creatorIds.insert(&account_copy);
        }
        let owner_clone = owner.clone();
        let mut creator_earnings = self.creators_earnings.get(&owner).expect("NOT_FOUND");
        let mut owner_nfts = self.creators.get(&owner).expect("NOT_FOUND");
        let mut buyer_nfts = self.creators.get(&buyer).expect("NOT_FOUND");
        let mut nft = owner_nfts.nfts.remove(index.into());
        creator_earnings += nft.price;
        nft.forSale = false;
        buyer_nfts.nfts.push(nft);
        self.creators_earnings.insert(&owner_clone, &creator_earnings);
        self.creators.insert(&owner, &owner_nfts);
        self.creators.insert(&buyer, &buyer_nfts);
    }

    #[payable]
    pub fn mint(
        &mut self,
        account:AccountId,
        uri: String,
        category: String,
        desc:String,
        price:u64,
    )
    {
        let account_copy = account.clone();
        let account_copy1 = account.clone();
        if self.creators.get(&account).is_none(){
            self.creators.insert(
                &account,
                &Creations {
                    nfts:Vec::new(),
                }
            );
            self.creatorIds.insert(&account_copy);
            self.creators_earnings.insert(&account_copy1, &0);
        }
        let mut creations = self.creators.get(&account).expect("NOT_FOUND");       
        creations.nfts.push(Nft {
            uri: uri,
            category: category,
            desc: desc, 
            price: price,
            forSale:true
        });
        self.creators.insert(&account, &creations);
    }

    #[payable]
    pub fn put_in_sale(
        &mut self,
        account:AccountId,
        index:u16,
        price:u64,
    )
    {
        let mut creations = self.creators.get(&account).expect("NOT_FOUND");
        let mut nft = creations.nfts.remove(index.into());
        nft.forSale = true;
        nft.price = price;
        creations.nfts.push(nft);
        self.creators.insert(&account, &creations);
    }

    #[payable]
    pub fn withdraw(
        &mut self,
        account_id:AccountId
    ){
        let mut earnings = self
            .creators_earnings
            .get(&account_id)
            .expect("NOT_FOUND");
        if earnings != 0 {
            self.creators_earnings.insert(&account_id, &0);
            let base:u128 = 10;
            let amt:u128 = earnings.into();
            Promise::new(env::predecessor_account_id()).transfer(amt*base.pow(24));
        }
    }

    pub fn get_nfts_by_account(
        &self,
        account:AccountId
    ) -> Creations{
        let creations = self.creators.get(&account).expect("NOT_FOUND");
        return creations;
    }

    pub fn get_earnings_by_account(
        &self,
        account:AccountId
    ) -> u64{
        let earnings = self.creators_earnings.get(&account).expect("NOT_FOUND");
        return earnings;
    }

    pub fn get_creators(&self) -> Vec<AccountId> {
        return self.creatorIds.to_vec();
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
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    // TESTS HERE
    #[test]
    fn check_flow(){
        let alice = AccountId::new_unchecked("alice.testnet".to_string());
        let context = get_context(alice.clone());
        testing_env!(context.build());

        let mut contract = Contract::new(alice);
        let account_id =  AccountId::new_unchecked("alice.testnet".to_string());
        contract.mint(
            AccountId::new_unchecked("alice11.testnet".to_string()),
            "ffff".to_string(),
            "fdsfsd".to_string(),
            "ffff".to_string(),
            1
        );
        contract.make_creator(0,AccountId::new_unchecked("alice11.testnet".to_string()),AccountId::new_unchecked("alice.testnet".to_string()));
        println!("Let's debug -0: {:?}", contract.get_nfts_by_account(AccountId::new_unchecked("alice.testnet".to_string())));
        println!("Let's debug -0: {:?}", contract.get_creators());
        println!("Let's debug -0: {:?}", contract.get_earnings_by_account(AccountId::new_unchecked("alice11.testnet".to_string())));
        contract.withdraw(AccountId::new_unchecked("alice11.testnet".to_string()));
        println!("Let's debug -0: {:?}", contract.get_earnings_by_account(AccountId::new_unchecked("alice11.testnet".to_string())));
        contract.put_in_sale(AccountId::new_unchecked("alice.testnet".to_string()), 0, 2);
        println!("Let's debug -0: {:?}", contract.get_nfts_by_account(AccountId::new_unchecked("alice.testnet".to_string())));
    }
    
}
