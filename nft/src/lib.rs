// Simple tryout to build a NFT that shows a text

// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

/// Use an efficient WASM allocator.
#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

use alloy_primitives::Address;
/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{alloy_primitives::U256, prelude::*};

mod erc721;
use crate::erc721::{ERC721, ERC721Error, ERC721Params};

pub struct NftParams;

impl ERC721Params for NftParams {
    const NAME: &'static str = "TextNft";
    const SYMBOL: &'static str = "TXT";
}

sol_storage! {
    #[entrypoint]
    pub struct TextNft {
        #[borrow]
        ERC721<NftParams> erc721;

        mapping(uint256 => string) content;
    }
}

const SVG_DATA_URL_START: &str = r#"data:image/svg+xml;charset=UTF-8,<svg xmlns="http://www.w3.org/2000/svg" width="640" height="480" viewBox="0 0 169.33 127"><path fill="white" stroke="%232479e3" stroke-width=".952" d="M.476.476h168.38v126.05H.476z"/><text xml:space="preserve" x="84.742" y="63.473" stroke-width=".352" font-family="Roboto" font-size="14.075" style="line-height:1.25" text-anchor="middle"><tspan x="84.742" y="63.473">"#;
const SVG_END: &str = r#"</tspan></text></svg>"#;

#[external]
#[inherit(ERC721<NftParams>)]
impl TextNft {
    #[selector(name = "tokenURI")]
    pub fn token_uri(&self, id: U256) -> Result<String, ERC721Error> {
        self.erc721.owner_of(id)?; // require NFT exist
        Ok(format!("{}{}{}", SVG_DATA_URL_START, self.get_content(id)?, SVG_END))
    } 

    pub fn get_content(&self, id: U256) -> Result<String, ERC721Error> {
        Ok(self.content.get(id).get_string())
    }

    pub fn total_supply(&self) -> U256 {
        self.erc721.total_supply.get()
    }

    pub fn mint_new(&mut self, to: Address, msg: String) -> Result<(), ERC721Error> {
        let id = self.erc721.total_supply.get();
        self.erc721.mint(to)?;
        self.content.setter(id).set_str(msg);

        Ok(())
    }
}