#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{ decl_module, dispatch::DispatchResult, debug, sp_runtime };
use frame_system::{ self as system, ensure_signed };
use sp_runtime::print;

pub trait Trait: system::Trait {}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        /// A function that says hello to the user by printing messages to the node log
        #[weight = 10_000]
        pub fn say_hello(origin) -> DispatchResult {
            let caller = ensure_signed(origin)?;

            // Print a message
            print("Hello World");
            // Inspecting variables
            debug::info!("Request sent by: {:?}", caller);

            // Indicate that this call succeeded
            Ok(())
        }

        // More dispatchable calls could go here
    }
}

