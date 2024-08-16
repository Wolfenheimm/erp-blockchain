use frame::prelude::{DispatchError, ensure};

/// Create a bucket for an owner (user) under a given MSP account.
pub(crate) fn do_create_bucket(
    sender: u32,

    private: bool,
) -> Result<(), DispatchError> {
    // TODO: Hold user funds for the bucket creation.

    // Check if the MSP is indeed an MSP.
    // ensure!(
    //         <T::Providers as ReadProvidersInterface>::is_msp(&msp_id),
    //         Error::<T>::NotAMsp
    //     );

    // // Create collection only if bucket is private
    // let maybe_collection_id = if private {
    //     // The `owner` of the collection is also the admin of the collection since most operations require the sender to be the admin.
    //     Some(Self::create_collection(sender.clone())?)
    // } else {
    //     None
    // };
    //
    // let bucket_id = <T as crate::Config>::Providers::derive_bucket_id(&sender, name);

    // <T::Providers as MutateProvidersInterface>::add_bucket(
    //     msp_id,
    //     sender,
    //     bucket_id,
    //     private,
    //     maybe_collection_id.clone(),
    // )?;

    Ok(())
}