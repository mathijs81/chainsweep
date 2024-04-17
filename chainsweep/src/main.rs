#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
#![cfg_attr(not(any(feature = "export-abi", test)), no_std)]

#[cfg(feature = "export-abi")]
fn main() {
  chainsweep::main();
}

