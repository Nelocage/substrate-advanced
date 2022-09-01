#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_std::prelude::*;
	use sp_std::vec::Vec;

	/* Placeholder for defining custom types. */

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		#[pallet::constant]
		type MaxBytesInHash: Get<u32>;
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		// For constraining the maximum bytes of a hash used for any proof
		// type MaxBytesInHash: Get<u32>;
	}

	// The struct on which we build all of our Pallet logic.
	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(_);

	// Pallets use events to inform users when important changes are made.
	// Event documentation should end with an array that provides descriptive names for parameters.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event emitted when a proof has been claimed. [who, claim]
		// ClaimCreated(T::AccountId, BoundedVec<u8, T::MaxBytesInHash>),
		ClaimCreated(T::AccountId, Vec<u8>),
		/// Event emitted when a claim is revoked by the owner. [who, claim]
		// ClaimRevoked(T::AccountId, BoundedVec<u8, T::MaxBytesInHash>),
		ClaimRevoked(T::AccountId, Vec<u8>),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The proof has already been claimed.
		ProofAlreadyClaimed,
		/// The proof does not exist, so it cannot be revoked.
		NoSuchProof,
		/// The proof is claimed by another account, so caller can't revoke it.
		NotProofOwner,
		ClaimTooLong
	}

	#[pallet::hooks]
	impl <T:Config>Hooks<BlockNumberFor<T>> for Pallet<T> {

	}

	#[pallet::storage]
	/// Maps each proof to its owner and block number when the proof was made
	pub(super) type Proofs<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxBytesInHash>,
		// Vec<u8>,
		(T::AccountId, T::BlockNumber),
		OptionQuery,
	>;

	// Dispatchable functions allow users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn create_claim(
			origin: OriginFor<T>,
			// proof: BoundedVec<u8, T::MaxBytesInHash>,
			proof: Vec<u8>,
		) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let sender = ensure_signed(origin)?;
			let bounded_claim=BoundedVec::<u8,T::MaxBytesInHash>::try_from(proof.clone())
				.map_err(|_| Error::<T>::ClaimTooLong)?;
			// Verify that the specified proof has not already been claimed.
			ensure!(
                !Proofs::<T>::contains_key(&bounded_claim),
                Error::<T>::ProofAlreadyClaimed
            );
			// Get the block number from the FRAME System pallet.
			let current_block = <frame_system::Pallet<T>>::block_number();
			// Store the proof with the sender and block number.
			Proofs::<T>::insert(&bounded_claim, (&sender, current_block));
			// Emit an event that the claim was created.
			Self::deposit_event(Event::ClaimCreated(sender, proof));
			Ok(())
		}

		#[pallet::weight(0)]
		pub fn transfer(
			origin: OriginFor<T>,
			claim: Vec<u8>,
			to: T::AccountId,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			//把vector<u8> 转换成bounded_vector type
			let bounded_claim=BoundedVec::<u8,T::MaxBytesInHash>::try_from(claim.clone())
				.map_err(|_| Error::<T>::ClaimTooLong)?;
			let (owner, _block_number) = Proofs::<T>::get(&bounded_claim).ok_or(Error::<T>::NoSuchProof)?;
			ensure!(owner == sender, Error::<T>::NotProofOwner);

			Proofs::<T>::insert(&bounded_claim,(to,frame_system::Pallet::<T>::block_number()));
			// Proofs::<T>::remove(&claim);
			// Self::deposit_event(Event::ClaimRevoked(sender, claim.clone()));
			// ensure!(
            //     !Proofs::<T>::contains_key(claim.clone()),
            //     Error::<T>::ProofAlreadyClaimed
            // );
			// Proofs::<T>::insert(
			// 	&bounded_claim,
			// 	(to.clone(), frame_system::Pallet::<T>::block_number()),
			// );
			// Self::deposit_event(Event::ClaimCreated(to, claim));
			Ok(().into())
		}

		#[pallet::weight(0)]
		pub fn revoke_claim(
			origin: OriginFor<T>,
			// proof: BoundedVec<u8, T::MaxBytesInHash>,
			proof: Vec<u8>,
		) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let sender = ensure_signed(origin)?;
			let bounded_claim=BoundedVec::<u8,T::MaxBytesInHash>::try_from(proof.clone())
				.map_err(|_| Error::<T>::ClaimTooLong)?;
			//只有已经存储的存证才可以被吊销
			let (owner, _) = Proofs::<T>::get(&bounded_claim).ok_or(Error::<T>::NoSuchProof)?;
			// Verify that the specified proof has been claimed.
			// ensure!(Proofs::<T>::contains_key(&bounded_claim), Error::<T>::NotProofOwner);
			// Get owner of the claim.
			// Panic condition: there is no way to set a `None` owner, so this must always unwrap.
			// Verify that sender of the current call is the claim owner.
			ensure!(sender == owner, Error::<T>::NotProofOwner);
			// Remove claim from storage.
			Proofs::<T>::remove(&bounded_claim);
			// Emit an event that the claim was erased.
			Self::deposit_event(Event::ClaimRevoked(sender, proof));
			Ok(().into())
		}
	}
}
