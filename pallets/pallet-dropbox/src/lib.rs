#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{inherent::Vec, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use log;

	type String = Vec<u8>;

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct my_file_struct<T: Config> {
		pub file_url: String,
		pub downloadable: bool,
		pub file_type: my_file_type,
		pub file_cost: u64,
		pub file_size: u64,
		pub owner: T::AccountId,
		pub d_count: u64,
	}

	/*
	impl<T: Config> my_file_struct<T> {
		fn new(file_url: String, downloadable: bool, file_type: my_file_type) -> Self {
			Self { file_url }
		}
	}
	*/

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	pub enum my_file_type {
		Normal,
		Previledged,
	}

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(PhantomData<T>);
	// pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn something)]
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn file_datas)]
	pub(super) type file_datas<T: Config> = StorageMap<_, Blake2_128, String, my_file_struct<T>>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),
		NewFile(T::AccountId, String),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn upload_file(
			origin: OriginFor<T>,
			file_hash: String,
			file_url: String,
			downloadable: bool,
			file_type: my_file_type,
			file_cost: u64,
			file_size: u64,
		) -> DispatchResult {
			// let file_url: String = "file url here".into();
			// let downloadable: bool = true;
			// let file_type: my_file_type = my_file_type::Normal;
			// let file_cost: u64 = 100;
			// let file_size: u64 = 100;

			// let file_url = "fielf url".as_bytes().to_vec();

			let who = ensure_signed(origin)?;

			// Update storage.
			// <Something<T>>::put(something);
			let new_file = my_file_struct {
				file_url,
				downloadable,
				file_type,
				file_cost,
				file_size,
				d_count: 0,
				owner: who.clone(),
			};

			// file_datas::(file_hash, new_file);
			file_datas::<T>::insert(&file_hash, new_file);
			log::info!("file uploaded");

			// Self::deposit_event(Event::SomethingStored(something, who));
			Self::deposit_event(Event::NewFile(who, file_hash));

			Ok(())
		}

		/*
		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => return Err(Error::<T>::NoneValue.into()),
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}
		*/
	}
}
