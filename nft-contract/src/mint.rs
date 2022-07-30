use crate::*;
use near_sdk::{ONE_NEAR};
use serde::Deserialize;


// #[derive(Debug, Deserialize)]
// struct MyStruct {
//     city: String,
//     name: String,
// }


#[near_bindgen]
impl Contract {
    #[payable]
    pub fn nft_mint(
        &mut self,
        receiver_id: AccountId,
        mut count: u128
    ) {
         //measure the initial storage being used on the contract
                let initial_storage_usage = env::storage_usage();


                assert!(count > 0, "Count of nft should be bigger than 1");

                   //get the attached deposit
                let attached_deposit = env::attached_deposit();

                assert!(ONE_NEAR * 10 * count <= attached_deposit, "Not enough deposit");

                             let token = Token {
                                                                       //set the owner ID equal to the receiver ID passed into the function
                                                         owner_id: receiver_id,
                                                                        //we set the approved account IDs to the default value (an empty map)
                                                         approved_account_ids: Default::default(),
                                                                           //the next approval ID is set to 0
                                                         next_approval_id: 0,
                                                     };

                loop {
                    count = count - 1;



                        self.tokens_by_id.insert(&self.tokens_count.to_string(), &token);
                        //specify the token struct that contains the owner ID

                          //call the internal method for adding the token to the owner
                          self.internal_add_token_to_owner(&token.owner_id, &self.tokens_count.to_string());

                      self.tokens_count += 1;

                      if count == 0 {
                        break;
                      }
                }

//                 //refund any excess storage if the user attached too much. Panic if they didn't attach enough to cover the required.
//                 refund_deposit(required_storage_in_bytes);
    }


    pub fn metadata_create(
        &mut self,
        token_id: String,
           title: String,
         description: String,
         hat: String,
         background: String,
         pet: String,
        flag: String,
          media: String,
        body: String,
        face: String,
    ) {


        assert_eq!(
                    &env::predecessor_account_id(),
                    &self.owner_id,
                    "Predecessor must be the token owner."
                );


        let subj = TokenMetadata {
                           title: serde::export::Some(title.to_string()),
                           description: serde::export::Some(description.to_string()),
                           hat: serde::export::Some(hat.to_string()),
                           background: serde::export::Some(background.to_string()),
                           pet: serde::export::Some(pet.to_string()),
                           flag: serde::export::Some(flag.to_string()),
                           media: serde::export::Some(media.to_string()),
                           body: serde::export::Some(body.to_string()),
                           face: serde::export::Some(face.to_string()),
                       };

        //insert the token ID and metadata
        self.token_metadata_by_id.insert(&token_id, &subj);
    }
}