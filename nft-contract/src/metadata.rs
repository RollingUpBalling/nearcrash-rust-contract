use crate::*;
pub type TokenId = String;
//defines the payout type we'll be returning as a part of the royalty standards.
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Payout {
    pub payout: HashMap<AccountId, U128>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct NFTContractMetadata {
        pub spec: String,              // required, essentially a version like "nft-1.0.0"
        pub name: String,              // required, ex. "Mosaics"
        pub symbol: String,            // required, ex. "MOSAIC"
        pub icon: Option<String>,      // Data URL
        pub base_uri: Option<String>, // Centralized gateway known to have reliable access to decentralized storage assets referenced by `reference` or `media` URLs
        pub reference: Option<String>, // URL to a JSON file with more info
        pub reference_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenMetadata {
        pub title: Option<String>, // ex. "Arch Nemesis: Mail Carrier" or "Parcel #5055"
        pub description: Option<String>, // free-form description
        pub extra: Option<String>,
        pub media: Option<String>, // URL to associated media, preferably to decentralized, content-addressed storage
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Token {
   //owner of the token
       pub owner_id: AccountId,
   //list of approved account IDs that have access to transfer the token. This maps an account ID to an approval ID
       pub approved_account_ids: HashMap<AccountId, u64>,
   //the next approval ID to give out.
       pub next_approval_id: u64,
   //keep track of the royalty percentages for the token in a hash map
       pub royalty: HashMap<AccountId, u32>,
}

//The Json token is what will be returned from view calls. 
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonToken {
    //token ID
       pub token_id: TokenId,
       //owner of the token
       pub owner_id: AccountId,
       //token metadata
       pub metadata: TokenMetadata,
       //list of approved account IDs that have access to transfer the token. This maps an account ID to an approval ID
       pub approved_account_ids: HashMap<AccountId, u64>,
       //keep track of the royalty percentages for the token in a hash map
       pub royalty: HashMap<AccountId, u32>,
}

pub trait NonFungibleTokenMetadata {
    //view call for returning the contract metadata
    fn nft_metadata(&self) -> NFTContractMetadata;
}

#[near_bindgen]
impl NonFungibleTokenMetadata for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
            self.metadata.get().unwrap()
        }
}