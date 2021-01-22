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

use codec::{Encode, EncodeLike, Decode};
use sp_std::{prelude::*, fmt::Debug};
use frame_support::{
	decl_module, decl_storage, decl_event, decl_error, ensure
};
use frame_system::ensure_signed;
use np_domain::Name;

pub trait Ownership<T: Config>: Encode + Decode + EncodeLike + Default + Eq + Debug + Clone {
	fn none() -> Self;
	fn account(account: T::AccountId) -> Self;
}

pub trait Config: pallet_balances::Config {
	type Ownership: Ownership<Self>;
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

decl_storage! {
	trait Store for Module<T: Config> as Registry {
		Ownerships: map hasher(blake2_128_concat) Name => T::Ownership;
	}
}

decl_event! {
	pub enum Event<T> where Ownership = <T as crate::Config>::Ownership {
		OwnershipSet(Name, Ownership),
	}
}

decl_error! {
	pub enum Error for Module<T: Config> {
		OwnershipMismatch,
		RootOwnershipNotAllowed,
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		type Error = Error<T>;

		fn deposit_event() = default;

		#[weight = 0]
		fn set_ownership(origin, name: Name, ownership: T::Ownership) {
			let owner = ensure_signed(origin)?;
			let ownership = Ownership::<T>::account(owner);

			ensure!(!name.is_root(), Error::<T>::RootOwnershipNotAllowed);

			let parent = name.parent().ok_or(Error::<T>::OwnershipMismatch)?;
			let parent_ownership = Ownerships::<T>::get(&parent);

			ensure!(parent_ownership == ownership, Error::<T>::OwnershipMismatch);

			if ownership == T::Ownership::none() {
				Ownerships::<T>::remove(name.clone());
			} else {
				Ownerships::<T>::insert(name.clone(), ownership.clone());
			}

			Self::deposit_event(Event::<T>::OwnershipSet(name, ownership));
		}
	}
}

impl<T: Config> Module<T> {
}
