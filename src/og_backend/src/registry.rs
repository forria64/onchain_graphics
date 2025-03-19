// File: src/og_backend/src/registry.rs

use candid::{CandidType, Principal};
use ic_cdk::api::time;
use ic_cdk::call;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::cell::RefCell;
use time::OffsetDateTime;
use time::macros::format_description;

/// Persistent data for a registered graphic, with mandatory title.
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct IndexedGraphic {
    pub ogid: u64,
    pub collection_id: u64,
    pub asset: String,
    pub title: String,                    // Now mandatory
    pub description: Option<String>,
    pub registration_timestamp: String,
    pub update_timestamp: Option<String>, // Update timestamp field
    pub canister_id: Principal,
}

/// Persistent data for a registered collection, with mandatory title.
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct IndexedCollection {
    pub collection_id: u64,
    pub title: String,                    // Now mandatory
    pub description: Option<String>,
    pub artist: Option<String>,
    pub external_link: Option<String>,
    pub graphics: Vec<u64>,
    pub registration_timestamp: String,
    pub update_timestamp: Option<String>, // Update timestamp field
}

/// The complete persistent state for the canister.
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Default)]
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

/// Parses the metadata blob (JSON string) to extract collection info.
/// The title field is mandatory. Other fields remain optional.
pub fn parse_collection(metadata_blob: &str) -> Result<CollectionProposal, String> {
    let v: Value = serde_json::from_str(metadata_blob)
        .map_err(|e| format!("JSON parse error: {}", e))?;
    let collection = v.get("collection").ok_or("Missing 'collection' field")?;

    // Title is mandatory (string).
    let title = collection
        .get("title")
        .and_then(|t| t.as_str())
        .ok_or("Missing required 'title' field in collection")?
        .to_string();

    let description = collection.get("description").and_then(|d| d.as_str()).map(String::from);
    let artist = collection.get("artist").and_then(|a| a.as_str()).map(String::from);
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
/// The title field is mandatory for each graphic. Other fields remain optional.
pub fn parse_graphics(metadata_blob: &str) -> Result<Vec<GraphicProposal>, String> {
    let v: Value = serde_json::from_str(metadata_blob)
        .map_err(|e| format!("JSON parse error: {}", e))?;
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

        // Title is mandatory (string).
        let title = graphic
            .get("title")
            .and_then(|t| t.as_str())
            .ok_or("Missing required 'title' field in graphic")?
            .to_string();

        let description = graphic
            .get("description")
            .and_then(|d| d.as_str())
            .map(String::from);

        proposals.push(GraphicProposal {
            asset,
            title,
            description,
        });
    }
    Ok(proposals)
}

/// Returns the current timestamp as a human-readable string.
pub fn get_current_timestamp() -> String {
    let nanos = time();
    let secs = (nanos / 1_000_000_000) as i64;
    let nsec = (nanos % 1_000_000_000) as u32;
    let datetime = OffsetDateTime::from_unix_timestamp(secs)
        .and_then(|dt| dt.replace_nanosecond(nsec))
        .unwrap_or_else(|_| OffsetDateTime::now_utc());
    let format = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
    datetime.format(&format).expect("Failed to format datetime")
}

/// Generates a unique on-chain graphic ID (OGID) based on the canister ID, asset, and current time.
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
            !state.registered_graphics.iter().any(|g| g.ogid == candidate)
        });
        if is_unique {
            return candidate;
        }
    }
}

/// Generates a unique collection ID based on the canister ID and current time.
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
    }
}

/// Generates indexed graphics from the provided graphic proposals using a differential update.
/// For each new graphic proposal, if an old record with the same asset exists, its registration timestamp is preserved.
/// The function compares modifiable fields (title and description) and only updates the record (with a new update timestamp)
/// if differences are found. It returns both the new vector and a boolean indicating whether any graphic was changed.
pub fn index_graphics_differential(
    canister_id: Principal,
    proposals: Vec<GraphicProposal>,
    collection_id: u64,
    old_graphics: Option<Vec<IndexedGraphic>>,
) -> Result<(Vec<IndexedGraphic>, bool), String> {
    let mut changed = false;
    let mut indexed = Vec::new();
    for proposal in proposals {
        let final_ogid = generate_ogid(canister_id, &proposal.asset);
        // If an old graphic exists with the same asset, preserve its registration timestamp.
        let reg_ts = if let Some(ref old) = old_graphics {
            if let Some(old_g) = old.iter().find(|g| g.asset == proposal.asset) {
                old_g.registration_timestamp.clone()
            } else {
                get_current_timestamp()
            }
        } else {
            get_current_timestamp()
        };
        // Check for differences in modifiable fields.
        let (update_ts, is_different) = if let Some(ref old) = old_graphics {
            if let Some(old_g) = old.iter().find(|g| g.asset == proposal.asset) {
                if old_g.title == proposal.title && old_g.description == proposal.description {
                    // No changes; keep old update timestamp.
                    (old_g.update_timestamp.clone(), false)
                } else {
                    // Changed fields; update timestamp now.
                    (Some(get_current_timestamp()), true)
                }
            } else {
                // New graphic.
                (Some(get_current_timestamp()), true)
            }
        } else {
            (Some(get_current_timestamp()), true)
        };
        if is_different {
            changed = true;
        }
        let indexed_graphic = IndexedGraphic {
            ogid: final_ogid,
            collection_id,
            asset: proposal.asset,
            title: proposal.title,
            description: proposal.description,
            registration_timestamp: reg_ts,
            update_timestamp: update_ts,
            canister_id,
        };
        indexed.push(indexed_graphic);
    }
    Ok((indexed, changed))
}

/// Generates an indexed collection record from the collection proposal and indexed graphics.
/// The old registration timestamp is preserved if available; a new update timestamp is generated.
pub fn index_collection(
    proposal: CollectionProposal,
    indexed_graphics: Vec<IndexedGraphic>,
    collection_id: u64,
    old_registration: Option<String>,
) -> Result<IndexedCollection, String> {
    let reg_ts = old_registration.unwrap_or_else(|| get_current_timestamp());
    let update_ts = Some(get_current_timestamp());
    Ok(IndexedCollection {
        collection_id,
        title: proposal.title,
        description: proposal.description,
        artist: proposal.artist,
        external_link: proposal.external_link,
        graphics: indexed_graphics.iter().map(|g| g.ogid).collect(),
        registration_timestamp: reg_ts,
        update_timestamp: update_ts,
    })
}

/// Atomically stores the indexed graphics and collection into persistent state.
pub fn store_state(
    indexed_graphics: Vec<IndexedGraphic>,
    indexed_collection: IndexedCollection,
) -> Result<(), String> {
    STATE.with(|state| {
        let mut s = state.borrow_mut();
        s.registered_graphics.extend(indexed_graphics);
        s.registered_collections.push(indexed_collection);
        Ok(())
    })
}

/// Finds and returns a registered collection by collection_id, if it exists.
pub fn find_collection(collection_id: u64) -> Option<IndexedCollection> {
    STATE.with(|state| {
        let state = state.borrow();
        state.registered_collections.iter().find(|c| c.collection_id == collection_id).cloned()
    })
}

/// Atomically unregisters a collection and its associated graphics from persistent state.
pub fn try_unregister_collection(collection_id: u64) -> Result<(), String> {
    if find_collection(collection_id).is_some() {
        STATE.with(|state| {
            let mut s = state.borrow_mut();
            s.registered_collections.retain(|c| c.collection_id != collection_id);
            s.registered_graphics.retain(|g| g.collection_id != collection_id);
            Ok(())
        })
    } else {
        Err("Collection not registered".to_string())
    }
}

/// The asynchronous function that implements the entire registration process.
/// It retrieves the metadata, parses proposals, generates new records, and updates the persistent state atomically.
pub async fn try_register_collection(canister_id: Principal) -> Result<(), String> {
    let metadata_blob = retrieve(canister_id, "/og_metadata.json")
        .await
        .map_err(|e| format!("Failed to retrieve metadata: {}", e))?;
    let collection_proposal = parse_collection(&metadata_blob)
        .map_err(|e| format!("Failed to parse collection: {}", e))?;
    let graphic_proposals = parse_graphics(&metadata_blob)
        .map_err(|e| format!("Failed to parse graphics: {}", e))?;
    let collection_id = generate_collection_id(canister_id);
    let indexed_graphics = index_graphics_differential(canister_id, graphic_proposals, collection_id, None)
        .map_err(|e| format!("Failed to index graphics: {}", e))?
        .0;
    let indexed_collection = index_collection(
        collection_proposal,
        indexed_graphics.clone(),
        collection_id,
        None,
    )
    .map_err(|e| format!("Failed to index collection: {}", e))?;
    store_state(indexed_graphics, indexed_collection)
        .map_err(|e| format!("Failed to store state: {}", e))?;
    Ok(())
}

/// The asynchronous function that implements the update process for an existing collection.
/// It retrieves updated metadata and proposals, preserves the original registration timestamp,
/// performs a differential update (both on the collection and on its graphics), and if any difference is detected,
/// updates the persistent state atomically. If no changes are found, the update call fails.
pub async fn try_update_collection(collection_id: u64, canister_id: Principal) -> Result<(), String> {
    let metadata_blob = retrieve(canister_id, "/og_metadata.json")
        .await
        .map_err(|e| format!("Failed to retrieve metadata: {}", e))?;
    let new_collection_proposal = parse_collection(&metadata_blob)
        .map_err(|e| format!("Failed to parse collection: {}", e))?;
    let new_graphic_proposals = parse_graphics(&metadata_blob)
        .map_err(|e| format!("Failed to parse graphics: {}", e))?;
    
    let old_collection = find_collection(collection_id)
        .ok_or("Collection not registered".to_string())?;
    
    // Check if the collection fields have changed.
    let collection_changed = old_collection.title != new_collection_proposal.title
        || old_collection.description != new_collection_proposal.description
        || old_collection.artist != new_collection_proposal.artist
        || old_collection.external_link != new_collection_proposal.external_link;
    
    // Get old graphics belonging to this collection.
    let old_graphics = STATE.with(|state| {
        state.borrow().registered_graphics
            .iter()
            .filter(|g| g.collection_id == collection_id)
            .cloned()
            .collect::<Vec<_>>()
    });
    
    // Generate new graphics with differential update.
    let (new_indexed_graphics, graphics_changed) = index_graphics_differential(
        canister_id,
        new_graphic_proposals,
        collection_id,
        Some(old_graphics),
    )?;
    
    // If neither the collection fields nor any graphic changed, abort the update.
    if !collection_changed && !graphics_changed {
        return Err("No differences detected. Update aborted.".to_string());
    }
    
    // Remove the old records for this collection.
    STATE.with(|state| {
        let mut s = state.borrow_mut();
        s.registered_graphics.retain(|g| g.collection_id != collection_id);
        s.registered_collections.retain(|c| c.collection_id != collection_id);
    });
    
    // Build the updated collection record (preserving old registration timestamp).
    let updated_collection = index_collection(
        new_collection_proposal,
        new_indexed_graphics.clone(),
        collection_id,
        Some(old_collection.registration_timestamp),
    )
    .map_err(|e| format!("Failed to index collection: {}", e))?;
    
    // Atomically update the state.
    STATE.with(|state| {
        let mut s = state.borrow_mut();
        s.registered_graphics.extend(new_indexed_graphics);
        s.registered_collections.push(updated_collection);
        Ok(())
    })
}

/// Temporary structure for a collection proposal, with a mandatory title.
#[derive(Clone, Debug)]
pub struct CollectionProposal {
    pub title: String,
    pub description: Option<String>,
    pub artist: Option<String>,
    pub external_link: Option<String>,
}

/// Temporary structure for a graphic proposal, with a mandatory title.
#[derive(Clone, Debug)]
pub struct GraphicProposal {
    pub asset: String,
    pub title: String,
    pub description: Option<String>,
}
