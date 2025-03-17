// File: src/og_backend/src/registry.rs

use candid::{CandidType, Principal};
use ic_cdk::api::time;
use ic_cdk::call;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::cell::RefCell;

/// Persistent data for a registered graphic.
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct IndexedGraphic {
    pub ogid: u64,
    pub collection_id: u64,
    pub asset: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub asset_hash: Option<String>,
    pub registration_timestamp: String,
    pub canister_id: Principal, // New field added
}

/// Persistent data for a registered collection.
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct IndexedCollection {
    pub collection_id: u64,
    pub title: Option<String>,
    pub description: Option<String>,
    pub artist: Option<String>,
    pub external_link: Option<String>,
    pub graphics: Vec<u64>,
    pub registration_timestamp: String,
}

/// The complete persistent state for the canister.
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Default)] // <--- add Default here
pub struct RegistryState {
    pub registered_collections: Vec<IndexedCollection>,
    pub registered_graphics: Vec<IndexedGraphic>,
}

// Global persistent state stored in stable memory.
thread_local! {
    static STATE: RefCell<RegistryState> = RefCell::new(RegistryState::default());
}

/// Returns a clone of the current persistent state.
pub fn get_state() -> RegistryState {
    STATE.with(|state| state.borrow().clone())
}

/// Replaces the current persistent state.
pub fn set_state(new_state: RegistryState) {
    STATE.with(|state| {
        *state.borrow_mut() = new_state;
    });
}

/// Initializes the persistent state.
pub fn init_state() {
    set_state(RegistryState::default());
}

/// Makes an inter-canister call to the asset canister's "retrieve" method.
/// Expects the asset canister to return a tuple containing a blob (Vec<u8>).
/// Returns the metadata blob as a UTF-8 string.
pub async fn retrieve(canister_id: Principal, asset: &str) -> Result<String, String> {
    let result: Result<(Vec<u8>,), _> = call(canister_id, "retrieve", (asset,)).await;
    match result {
        Ok((metadata_blob,)) => {
            String::from_utf8(metadata_blob).map_err(|e| format!("Invalid UTF-8 data: {}", e))
        }
        Err((code, msg)) => Err(format!(
            "Inter-canister call failed with code {:?}: {:?}",
            code, msg
        )),
    }
}

/// Parses the metadata blob (JSON string) to extract collection information.
pub fn parse_collection(metadata_blob: &str) -> Result<CollectionProposal, String> {
    let v: Value =
        serde_json::from_str(metadata_blob).map_err(|e| format!("JSON parse error: {}", e))?;
    let collection = v.get("collection").ok_or("Missing 'collection' field")?;

    let title = collection
        .get("title")
        .and_then(|t| t.as_str())
        .map(String::from);
    let description = collection
        .get("description")
        .and_then(|d| d.as_str())
        .map(String::from);
    let artist = collection
        .get("artist")
        .and_then(|a| a.as_str())
        .map(String::from);
    let external_link = collection
        .get("external_link")
        .and_then(|e| e.as_str())
        .map(String::from);

    Ok(CollectionProposal {
        title,
        description,
        artist,
        external_link,
    })
}

/// Parses the metadata blob (JSON string) to extract graphic proposals.
pub fn parse_graphics(metadata_blob: &str) -> Result<Vec<GraphicProposal>, String> {
    let v: Value =
        serde_json::from_str(metadata_blob).map_err(|e| format!("JSON parse error: {}", e))?;
    let graphics = v
        .get("graphics")
        .ok_or("Missing 'graphics' field")?
        .as_array()
        .ok_or("'graphics' is not an array")?;

    let mut proposals = Vec::new();
    for graphic in graphics {
        let asset = graphic
            .get("asset")
            .and_then(|a| a.as_str())
            .ok_or("Missing required 'asset' field in graphic")?
            .to_string();
        let title = graphic
            .get("title")
            .and_then(|t| t.as_str())
            .map(String::from);
        let description = graphic
            .get("description")
            .and_then(|d| d.as_str())
            .map(String::from);
        let asset_hash = graphic
            .get("asset_hash")
            .and_then(|h| h.as_str())
            .map(String::from);
        proposals.push(GraphicProposal {
            asset,
            title,
            description,
            asset_hash,
        });
    }
    Ok(proposals)
}

/// Returns the current timestamp in nanoseconds as a string.
pub fn get_current_timestamp() -> String {
    time().to_string()
}

/// Generates a unique on-chain graphic ID (OGID) based on the canister ID, asset path, and current time.
/// This function loops until it produces an OGID that is not already present in the registry.
///
/// Changes: The generated candidate is reduced by 4 digits (via division by 10,000).
pub fn generate_ogid(canister_id: Principal, asset: &str) -> u64 {
    loop {
        let ts = get_current_timestamp();
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}{}", canister_id.to_text(), asset, ts));
        let result = hasher.finalize();
        let bytes = &result[..8];
        let candidate = u64::from_be_bytes(bytes.try_into().unwrap()) / 10000;
        let is_unique = STATE.with(|state| {
            let state = state.borrow();
            !state
                .registered_graphics
                .iter()
                .any(|g| g.ogid == candidate)
        });
        if is_unique {
            return candidate;
        }
        // Otherwise, loop and try again with a new timestamp.
    }
}

/// Generates a unique collection ID based on the canister ID and current time.
/// This function loops until it finds an ID that is not already present in the registry.
///
/// Changes: The generated candidate is reduced by 4 digits (via division by 10,000).
pub fn generate_collection_id(canister_id: Principal) -> u64 {
    loop {
        let ts = get_current_timestamp();
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}{}", canister_id.to_text(), "seed", ts));
        let result = hasher.finalize();
        let bytes = &result[..8];
        let candidate = u64::from_be_bytes(bytes.try_into().unwrap()) / 10000;
        if find_collection(candidate).is_none() {
            return candidate;
        }
        // If a collision is detected, loop and try again.
    }
}

/// Generates indexed graphics from the provided graphic proposals.
/// Each graphic is assigned a unique OGID (generated with time input) and
/// the provided collection_id is stored in each graphic record.
pub fn index_graphics(
    canister_id: Principal,
    proposals: Vec<GraphicProposal>,
    collection_id: u64,
) -> Result<Vec<IndexedGraphic>, String> {
    let mut indexed = Vec::new();
    for proposal in proposals {
        let final_ogid = generate_ogid(canister_id, &proposal.asset);
        let indexed_graphic = IndexedGraphic {
            ogid: final_ogid,
            collection_id,
            asset: proposal.asset,
            title: proposal.title,
            description: proposal.description,
            asset_hash: proposal.asset_hash,
            registration_timestamp: get_current_timestamp(),
            canister_id, // Set the new field with the provided canister_id
        };
        indexed.push(indexed_graphic);
    }
    Ok(indexed)
}

/// Generates an indexed collection from a collection proposal and the list of indexed graphics.
/// The collection stores the generated graphic OGIDs and the generated collection_id.
pub fn index_collection(
    proposal: CollectionProposal,
    indexed_graphics: Vec<IndexedGraphic>,
    collection_id: u64,
) -> Result<IndexedCollection, String> {
    let graphics_ids = indexed_graphics.iter().map(|g| g.ogid).collect();
    Ok(IndexedCollection {
        collection_id,
        title: proposal.title,
        description: proposal.description,
        artist: proposal.artist,
        external_link: proposal.external_link,
        graphics: graphics_ids,
        registration_timestamp: get_current_timestamp(),
    })
}

/// Atomically stores the indexed graphics and collection into persistent state.
pub fn store_state(
    indexed_graphics: Vec<IndexedGraphic>,
    indexed_collection: IndexedCollection,
) -> Result<(), String> {
    STATE.with(|state| {
        let mut s = state.borrow_mut();
        let _backup_graphics = s.registered_graphics.clone();
        let _backup_collections = s.registered_collections.clone();
        s.registered_graphics.extend(indexed_graphics);
        s.registered_collections.push(indexed_collection);
        Ok(())
    })
}

/// Finds and returns a registered collection by collection_id, if it exists.
pub fn find_collection(collection_id: u64) -> Option<IndexedCollection> {
    STATE.with(|state| {
        let state = state.borrow();
        state
            .registered_collections
            .iter()
            .find(|c| c.collection_id == collection_id)
            .cloned()
    })
}

/// Atomically unregisters a collection and its associated graphics from persistent state.
/// It checks if the collection is registered using find_collection, and if so,
/// removes the collection and all graphics associated with it.
pub fn try_unregister_collection(collection_id: u64) -> Result<(), String> {
    if let Some(_collection) = find_collection(collection_id) {
        STATE.with(|state| {
            let mut s = state.borrow_mut();
            s.registered_collections
                .retain(|c| c.collection_id != collection_id);
            s.registered_graphics
                .retain(|graphic| graphic.collection_id != collection_id);
            Ok(())
        })
    } else {
        Err("Collection not registered".to_string())
    }
}

/// The asynchronous function that implements the entire registration process.
/// It retrieves the metadata, parses collection and graphics proposals,
/// indexes them, and updates the persistent state atomically.
pub async fn try_register_collection(canister_id: Principal) -> Result<(), String> {
    // Retrieve the metadata blob from the asset canister.
    let metadata_blob = retrieve(canister_id, "/og_metadata.json")
        .await
        .map_err(|e| format!("Failed to retrieve metadata: {}", e))?;

    // Parse the collection proposal from the metadata.
    let collection_proposal = parse_collection(&metadata_blob)
        .map_err(|e| format!("Failed to parse collection: {}", e))?;

    // Parse the graphic proposals from the metadata.
    let graphic_proposals =
        parse_graphics(&metadata_blob).map_err(|e| format!("Failed to parse graphics: {}", e))?;

    // Generate a unique collection_id using the new dedicated function.
    let collection_id = generate_collection_id(canister_id);

    // Generate indexed graphics (with unique OGIDs) and assign the generated collection_id.
    let indexed_graphics = index_graphics(canister_id, graphic_proposals, collection_id)
        .map_err(|e| format!("Failed to index graphics: {}", e))?;

    // Index the collection proposal with the generated graphic OGIDs and collection_id.
    let indexed_collection =
        index_collection(collection_proposal, indexed_graphics.clone(), collection_id)
            .map_err(|e| format!("Failed to index collection: {}", e))?;

    // Atomically update persistent state for both graphics and the collection.
    store_state(indexed_graphics, indexed_collection)
        .map_err(|e| format!("Failed to store state: {}", e))?;

    Ok(())
}

/// Temporary structure for a collection proposal parsed from metadata.
#[derive(Clone, Debug)]
pub struct CollectionProposal {
    pub title: Option<String>,
    pub description: Option<String>,
    pub artist: Option<String>,
    pub external_link: Option<String>,
}

/// Temporary structure for a graphic proposal parsed from metadata.
#[derive(Clone, Debug)]
pub struct GraphicProposal {
    pub asset: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub asset_hash: Option<String>,
}
