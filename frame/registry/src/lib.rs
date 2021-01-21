// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of Nomo.
//
// Copyright (c) 2019-2020 Wei Tang.
//
// Nomo is free software: you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Nomo is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Nomo. If not, see <http://www.gnu.org/licenses/>.

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	dispatch::DispatchResult, decl_module, decl_storage, decl_event,
};
use sp_std::prelude::*;
use frame_system::ensure_signed;

pub trait Config: pallet_balances::Config {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

decl_storage! {
	trait Store for Module<T: Config> as Example {
		Dummy get(fn dummy) config(): Option<T::Balance>;
		Bar get(fn bar) config(): map hasher(blake2_128_concat) T::AccountId => T::Balance;
		Foo get(fn foo) config(): T::Balance;
	}
}

decl_event!(
	pub enum Event<T> where B = <T as pallet_balances::Config>::Balance {
		Dummy(B),
	}
);

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		#[weight = 0]
		fn accumulate_dummy(origin, increase_by: T::Balance) -> DispatchResult {
			// This is a public call, so we ensure that the origin is some signed account.
			let _sender = ensure_signed(origin)?;

			// Here's the new one of read and then modify the value.
			<Dummy<T>>::mutate(|dummy| {
				let new_dummy = dummy.map_or(increase_by, |dummy| dummy + increase_by);
				*dummy = Some(new_dummy);
			});

			// Let's deposit an event to let the outside world know this happened.
			Self::deposit_event(RawEvent::Dummy(increase_by));

			// All good.
			Ok(())
		}
	}
}

impl<T: Config> Module<T> {
	// Add public immutables and private mutables.
	#[allow(dead_code)]
	fn accumulate_foo(origin: T::Origin, increase_by: T::Balance) -> DispatchResult {
		let _sender = ensure_signed(origin)?;

		let prev = <Foo<T>>::get();
		// Because Foo has 'default', the type of 'foo' in closure is the raw type instead of an Option<> type.
		let result = <Foo<T>>::mutate(|foo| {
			*foo = *foo + increase_by;
			*foo
		});
		assert!(prev + increase_by == result);

		Ok(())
	}
}
