// Simple tryout to build a NFT that shows a text

// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

/// Use an efficient WASM allocator.
/// Commented out because seems rustmate already does this
//#[global_allocator]
//static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

use alloy_primitives::Address;
/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{alloy_primitives::U256, prelude::*};

use rustmate::tokens::erc721::{ERC721Error, ERC721Params, ERC721};

pub struct NftParams;

impl ERC721Params for NftParams {
    const NAME: &'static str = "TextNft";
    const SYMBOL: &'static str = "TXT";

    fn token_uri(id: U256) -> String {
        format!("https://example.com/nft/{}", id)
    } 
}

sol_storage! {
    #[entrypoint]
    pub struct TextNft {
        #[borrow]
        ERC721<NftParams> erc721;
        uint256 total_supply;

        mapping(uint256 => string) content;
    }
}
#[external]
#[inherit(ERC721<NftParams>)]
impl TextNft {
    pub fn get_content(&self, id: U256) -> Result<String, ERC721Error> {
        Ok(self.content.get(id).get_string())
    }

    pub fn total_supply(&self) -> U256 {
        self.total_supply.get()
    }

    pub fn mint_new(&mut self, to: Address, msg: String) -> Result<(), ERC721Error> {
        let id = self.total_supply();
        self.erc721.mint(to, id)?;
        self.content.setter(id).set_str(msg);
        self.total_supply.set(id + U256::from(1));

        Ok(())
    }
}