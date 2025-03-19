// File: src/og_backend/src/lib.rs

mod auth;
mod frontend_api;
mod registry; // Now includes both auth and registry functions

use candid::Principal;
use ic_cdk::storage;
use ic_cdk_macros::{init, post_upgrade, pre_upgrade, query, update};
use serde_json::json;

#[init]
fn init() {
    registry::init_state();
}

#[pre_upgrade]
fn pre_upgrade() {
    let state = registry::get_state();
    storage::stable_save((state,)).expect("failed to save state");
}

#[post_upgrade]
fn post_upgrade() {
    let (state,): (registry::RegistryState,) =
        storage::stable_restore().expect("failed to restore state");
    registry::set_state(state);
}

/// Controller-only update call to register a new collection.
#[update]
async fn register_collection(canister_id: Principal) -> String {
    let caller_id = ic_cdk::api::caller();
    if let Err(err) = auth::authenticate_caller(caller_id).await {
        return json!({ "error": { "code": "UNAUTHORIZED", "message": err } }).to_string();
    }
    match registry::try_register_collection(canister_id).await {
        Ok(_) => json!({ "ok": "Collection registered successfully." }).to_string(),
        Err(err_msg) => json!({ "error": { "code": "REGISTER_COLLECTION_FAILED", "message": err_msg } }).to_string(),
    }
}

/// Controller-only update call to update an existing collection.
/// This API first checks the caller's principal via the auth function,
/// then retains the original registration timestamp and creates a new update timestamp.
#[update]
async fn update_collection(collection_id: u64, canister_id: Principal) -> String {
    let caller_id = ic_cdk::api::caller();
    if let Err(err) = auth::authenticate_caller(caller_id).await {
        return json!({ "error": { "code": "UNAUTHORIZED", "message": err } }).to_string();
    }
    match registry::try_update_collection(collection_id, canister_id).await {
        Ok(_) => json!({ "ok": "Collection updated successfully." }).to_string(),
        Err(err_msg) => json!({ "error": { "code": "UPDATE_COLLECTION_FAILED", "message": err_msg } }).to_string(),
    }
}

/// Controller-only update call to unregister an existing collection.
#[update]
async fn unregister_collection(collection_id: u64) -> String {
    let caller_id = ic_cdk::api::caller();
    if let Err(err) = auth::authenticate_caller(caller_id).await {
        return json!({ "error": { "code": "UNAUTHORIZED", "message": err } }).to_string();
    }
    match registry::try_unregister_collection(collection_id) {
        Ok(_) => json!({ "ok": "Collection unregistered successfully." }).to_string(),
        Err(err_msg) => json!({ "error": { "code": "UNREGISTER_COLLECTION_FAILED", "message": err_msg } }).to_string(),
    }
}

/// PUBLIC API CALL: Returns all registered collection IDs.
#[query]
fn fetch_collections() -> String {
    match frontend_api::try_fetch_collections() {
        Ok(ids) => serde_json::to_string_pretty(&ids).unwrap_or_else(|e| {
            json!({ "error": { "code": "SERIALIZATION_ERROR", "message": e.to_string() } }).to_string()
        }),
        Err(e) => json!({ "error": { "code": "FETCH_COLLECTIONS_FAILED", "message": e } }).to_string(),
    }
}

/// PUBLIC API CALL: Returns collection details (excluding the graphics vector).
#[query]
fn fetch_collection(collection_id: u64) -> String {
    match frontend_api::try_fetch_collection(collection_id) {
        Ok(collection) => serde_json::to_string_pretty(&collection).unwrap_or_else(|e| {
            json!({ "error": { "code": "SERIALIZATION_ERROR", "message": e.to_string() } }).to_string()
        }),
        Err(e) => json!({ "error": { "code": "FETCH_COLLECTION_FAILED", "message": e } }).to_string(),
    }
}

/// PUBLIC API CALL: Returns all graphic OGIDs for the given collection.
#[query]
fn fetch_graphics(collection_id: u64) -> String {
    match frontend_api::try_fetch_graphics(collection_id) {
        Ok(graphics) => serde_json::to_string_pretty(&graphics).unwrap_or_else(|e| {
            json!({ "error": { "code": "SERIALIZATION_ERROR", "message": e.to_string() } }).to_string()
        }),
        Err(e) => json!({ "error": { "code": "FETCH_GRAPHICS_FAILED", "message": e } }).to_string(),
    }
}

/// PUBLIC API CALL: Returns the full details of a registered graphic given its OGID.
#[query]
fn fetch_graphic(ogid: u64) -> String {
    match frontend_api::try_fetch_graphic(ogid) {
        Ok(graphic) => serde_json::to_string_pretty(&graphic).unwrap_or_else(|e| {
            json!({ "error": { "code": "SERIALIZATION_ERROR", "message": e.to_string() } }).to_string()
        }),
        Err(e) => json!({ "error": { "code": "FETCH_GRAPHIC_FAILED", "message": e } }).to_string(),
    }
}

