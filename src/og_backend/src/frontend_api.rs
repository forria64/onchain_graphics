// File: src/og_backend/src/frontend_api.rs

use crate::registry::{get_state, IndexedGraphic};
use serde::Serialize;

/// A helper struct to return collection information without the graphics vector.
/// Now that `title` is mandatory, we store it as a `String`.
#[derive(Serialize)]
pub struct CollectionInfo {
    pub collection_id: u64,
    pub title: String,
    pub description: Option<String>,
    pub artist: Option<String>,
    pub external_link: Option<String>,
    pub registration_timestamp: String,
    pub update_timestamp: Option<String>,
}

/// Returns a vector of all registered collection IDs.
pub fn try_fetch_collections() -> Result<Vec<u64>, String> {
    let state = get_state();
    let ids = state
        .registered_collections
        .into_iter()
        .map(|c| c.collection_id)
        .collect();
    Ok(ids)
}

/// Returns the collection details (excluding the graphics vector) for the given collection_id.
/// Now that `title` is non-optional, we assign it directly from the `IndexedCollection` record.
pub fn try_fetch_collection(collection_id: u64) -> Result<CollectionInfo, String> {
    let state = get_state();
    state
        .registered_collections
        .into_iter()
        .find(|c| c.collection_id == collection_id)
        .map(|collection| CollectionInfo {
            collection_id: collection.collection_id,
            title: collection.title, // no longer Option
            description: collection.description,
            artist: collection.artist,
            external_link: collection.external_link,
            registration_timestamp: collection.registration_timestamp,
            update_timestamp: collection.update_timestamp,
        })
        .ok_or_else(|| "Collection not found".to_string())
}

/// Returns a vector of graphic OGIDs for the collection with the given collection_id.
pub fn try_fetch_graphics(collection_id: u64) -> Result<Vec<u64>, String> {
    let state = get_state();
    state
        .registered_collections
        .into_iter()
        .find(|c| c.collection_id == collection_id)
        .map(|collection| collection.graphics)
        .ok_or_else(|| "Collection not found".to_string())
}

/// Returns all the fields of a registered graphic for the given OGID.
/// The `IndexedGraphic` struct itself has been updated to require `title: String`.
pub fn try_fetch_graphic(ogid: u64) -> Result<IndexedGraphic, String> {
    let state = get_state();
    state
        .registered_graphics
        .into_iter()
        .find(|g| g.ogid == ogid)
        .ok_or_else(|| "Graphic not found".to_string())
}

