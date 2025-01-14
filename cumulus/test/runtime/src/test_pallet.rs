// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

/// A special pallet that exposes dispatchables that are only useful for testing.
pub use pallet::*;

/// Some key that we set in genesis and only read in [`TestOnRuntimeUpgrade`] to ensure that
/// [`OnRuntimeUpgrade`] works as expected.
pub const TEST_RUNTIME_UPGRADE_KEY: &[u8] = b"+test_runtime_upgrade_key+";

#[frame_support::pallet(dev_mode)]
pub mod pallet {
	use crate::test_pallet::TEST_RUNTIME_UPGRADE_KEY;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + cumulus_pallet_parachain_system::Config {}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// A test dispatchable for setting a custom head data in `validate_block`.
		#[pallet::weight(0)]
		pub fn set_custom_validation_head_data(
			_: OriginFor<T>,
			custom_header: sp_std::vec::Vec<u8>,
		) -> DispatchResult {
			cumulus_pallet_parachain_system::Pallet::<T>::set_custom_validation_head_data(
				custom_header,
			);
			Ok(())
		}
	}

	#[derive(frame_support::DefaultNoBound)]
	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub self_para_id: Option<cumulus_primitives_core::ParaId>,
		#[serde(skip)]
		pub _config: sp_std::marker::PhantomData<T>,
	}

	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		fn build(&self) {
			sp_io::storage::set(TEST_RUNTIME_UPGRADE_KEY, &[1, 2, 3, 4]);
			self.self_para_id.map(|para_id| {
				crate::ParachainId::set(&para_id);
			});
		}
	}
}
