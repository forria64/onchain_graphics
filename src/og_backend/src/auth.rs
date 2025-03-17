use candid::Principal;
use ic_cdk::api::management_canister::main::{canister_status, CanisterIdRecord};
use ic_cdk::id;

/// Checks if the caller is among this canister's controllers.
pub async fn authenticate_caller(caller_id: Principal) -> Result<(), String> {
    // Prepare a record containing this canister's own principal.
    let canister_id_record = CanisterIdRecord {
        canister_id: id(), // `id()` is the principal of *this* canister
    };

    // Call the management canister's canister_status function.
    // It returns a tuple: (CanisterStatusResponse,)
    let (status,) = canister_status(canister_id_record)
        .await
        .map_err(|e| format!("Failed to call canister_status: {}", e.1))?;

    // Make sure the caller is in the canister's controllers list.
    if !status.settings.controllers.contains(&caller_id) {
        return Err(format!(
            "Caller {} not in controllers list: {:?}",
            caller_id, status.settings.controllers
        ));
    }

    Ok(())
}
