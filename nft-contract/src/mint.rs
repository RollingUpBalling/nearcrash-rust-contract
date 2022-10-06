use crate::*;
use near_sdk::{ONE_NEAR};
use serde::Deserialize;

pub(crate) fn hash_account_id(account_id: &AccountId) -> CryptoHash {
    //get the default hash
    let mut hash = CryptoHash::default();
    //we hash the account ID and return it
    hash.copy_from_slice(&env::sha256(account_id.as_bytes()));
    hash
}

// #[derive(Debug, Deserialize)]
// struct MyStruct {
//     city: String,
//     name: String,
// }

// #[derive(Serialize, Deserialize)]
// #[serde(crate = "near_sdk::serde")]
pub struct Attribute {
    pub trait_type:Option<String>,
    pub value: Option<String>,
}


#[near_bindgen]
impl Contract {
    #[payable]
    pub fn nft_mint(
        &mut self,
        receiver_id: AccountId,
    ) {
        //measure the initial storage being used on the contract
        let initial_storage_usage = env::storage_usage();

        assert!(999 - self.tokens_count > 0, "No NFTs to buy left");


        assert_eq!(self.public_sales || self.white_list_sales, true, "mint is blocked");
        assert_eq!(self.public_sales || (self.white_list_sales && self.white_list.contains(&receiver_id)), true, "Whitelist mint is blocked for your address");


        //get the set of tokens for the given account
        let mut tokens_set_len = self.tokens_per_owner.get(&receiver_id).unwrap_or_else(|| {
            //if the account doesn't have any tokens, we create a new unordered set
            UnorderedSet::new(
                StorageKey::TokenPerOwnerInner {
                    //we get a new unique prefix for the collection
                    account_id_hash: hash_account_id(&receiver_id),
                }
                    .try_to_vec()
                    .unwrap(),
            )
        }).len();

        assert_eq!(
                self.team_list.contains(&receiver_id)
                || (self.og_list.contains(&receiver_id) && tokens_set_len < 5)
                || (!self.og_list.contains(&receiver_id) && tokens_set_len < 1),
            true,
            "You already got your nfts"
        );

        let attached_deposit = env::attached_deposit();

        assert!(ONE_NEAR * 10 <= attached_deposit, "Not enough deposit");

        // create a royalty map to store in the token
        let mut royalty = HashMap::new();

        royalty.insert(env::predecessor_account_id(), 1000);

        let token = Token {
            //set the owner ID equal to the receiver ID passed into the function
            owner_id: receiver_id,
            //we set the approved account IDs to the default value (an empty map)
            approved_account_ids: Default::default(),
            //the next approval ID is set to 0
            next_approval_id: 0,
            royalty,
        };

        self.tokens_by_id.insert(&self.tokens_count.to_string(), &token);
        //specify the token struct that contains the owner ID

        //call the internal method for adding the token to the owner
        self.internal_add_token_to_owner(&token.owner_id, &self.tokens_count.to_string());

        // Construct the mint log as per the events standard.
        let nft_mint_log: EventLog = EventLog {
            // Standard name ("nep171").
            standard: NFT_STANDARD_NAME.to_string(),
            // Version of the standard ("nft-1.0.0").
            version: NFT_METADATA_SPEC.to_string(),
            // The data related with the event stored in a vector.
            event: EventLogVariant::NftMint(vec![NftMintLog {
                // Owner of the token.
                owner_id: token.owner_id.to_string(),
                // Vector of token IDs that were minted.
                token_ids: vec![self.tokens_count.to_string()],
                // An optional memo to include.
                memo: None,
            }]),
        };

        self.tokens_count += 1;

        // Log the serialized json.
        env::log_str(&nft_mint_log.to_string());
    }

    // pub fn rofl(&mut self, id: AccountId) {
    //     let nft_mint_log: EventLog = EventLog {
    //         // Standard name ("nep171").
    //         standard: NFT_STANDARD_NAME.to_string(),
    //         // Version of the standard ("nft-1.0.0").
    //         version: NFT_METADATA_SPEC.to_string(),
    //         // The data related with the event stored in a vector.
    //         event: EventLogVariant::NftMint(vec![NftMintLog {
    //             // Owner of the token.
    //             owner_id: id.to_string(),
    //             // Vector of token IDs that were minted.
    //             token_ids: vec![0.to_string()],
    //             // An optional memo to include.
    //             memo: None,
    //         }]),
    //     };
    //     env::log_str(&nft_mint_log.to_string());
    // }

    pub fn white_list_add(
        &mut self,
        whitelist_id: AccountId,
    ) -> Vec<near_sdk::AccountId> {
        assert_eq!(
            &env::predecessor_account_id(),
            &self.owner_id,
            "Predecessor must be the token owner."
        );

        assert_eq!(self.white_list.to_vec().contains(&whitelist_id), false, "Already in whitelist");
        &self.white_list.push(whitelist_id);
        return self.white_list.to_vec();
    }

    pub fn white_list_sales(&mut self, status: bool) -> bool {
        assert_eq!(
            &env::predecessor_account_id(),
            &self.owner_id,
            "Predecessor must be the token owner."
        );

        self.white_list_sales = status;
        return self.white_list_sales;
    }

    pub fn public_sales(&mut self, status: bool) -> bool {
        assert_eq!(
            &env::predecessor_account_id(),
            &self.owner_id,
            "Predecessor must be the token owner."
        );

        self.public_sales = status;
        return self.public_sales;

    }

    pub fn white_list_get(&mut self) -> Vec<near_sdk::AccountId> {
        return self.white_list.to_vec();
    }

    pub fn og_list_add(
        &mut self,
        whitelist_id: AccountId,
    ) -> Vec<near_sdk::AccountId> {
        assert_eq!(
            &env::predecessor_account_id(),
            &self.owner_id,
            "Predecessor must be the token owner."
        );

        assert_eq!(self.og_list.contains(&whitelist_id), false, "Already in og_list");
        &self.og_list.push(whitelist_id);
        return self.og_list.to_vec();
    }

    pub fn og_list(&mut self) ->Vec<near_sdk::AccountId>  {
        return self.og_list.to_vec();
    }

    pub fn team_get(&mut self) -> Vec<near_sdk::AccountId>{
        return self.team_list.to_vec();
    }

    pub fn team_list_add(&mut self, whitelist_id: AccountId) -> Vec<near_sdk::AccountId> {
        assert_eq!(
            &env::predecessor_account_id(),
            &self.owner_id,
            "Predecessor must be the token owner."
        );

        assert_eq!(self.team_list.contains(&whitelist_id), false, "Already in team_list");
        &self.team_list.push(whitelist_id);
        return self.team_list.to_vec();
    }


    pub fn metadata_create(
        &mut self,
        token_id: String,
        title: String,
        description: String,
        extra: String,
        media: String,
    ) {
        assert_eq!(
            &env::predecessor_account_id(),
            &self.owner_id,
            "Predecessor must be the token owner."
        );

        let subj = TokenMetadata {
            title: serde::export::Some(title.to_string()),
            description: serde::export::Some(description.to_string()),
            media: serde::export::Some(media.to_string()),
            extra: serde::export::Some(extra.to_string()),
        };

        //insert the token ID and metadata
        self.token_metadata_by_id.insert(&token_id, &subj);
    }
}