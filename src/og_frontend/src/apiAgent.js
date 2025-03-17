// File: src/og_frontend/src/apiAgent.js

import { HttpAgent, Actor } from '@dfinity/agent';
import { idlFactory as ogBackendIdlFactory } from '../../declarations/og_backend/og_backend.did.js';

// Read the canister ID from the environment variable (ensure it's defined in your .env file)
const canisterId = process.env.CANISTER_ID_OG_BACKEND;
console.log("CANISTER_ID_OG_BACKEND:", canisterId);

// Set the host for your local replica (port 4943) or use the production host if on ic.
const localHost = 'http://localhost:4943';
const host = process.env.DFX_NETWORK === 'ic' ? 'https://ic0.app' : localHost;

// Create the HttpAgent
const agent = new HttpAgent({ host });

// For local development, fetch the root key so the agent validates the certificates.
// Ensure your local replica is running (run `dfx start`).
if (process.env.DFX_NETWORK !== 'ic') {
  agent.fetchRootKey().catch(err => {
    console.warn('Unable to fetch root key. Check that you are running a local replica.');
  });
}

// Automatically create and cache the actor using the generated candid interface.
const ogBackendActor = Actor.createActor(ogBackendIdlFactory, {
  agent,
  canisterId,
});

/*----------------------------------------------------------
  Exposed Fetch API Calls
-----------------------------------------------------------*/

/**
 * Fetch all registered collection IDs.
 */
export async function fetchCollections() {
  try {
    const result = await ogBackendActor.fetch_collections();
    return JSON.parse(result);
  } catch (err) {
    console.error('fetchCollections error:', err);
    throw err;
  }
}

/**
 * Fetch the details of a single collection by its ID.
 * @param {number} collectionId
 */
export async function fetchCollection(collectionId) {
  try {
    const result = await ogBackendActor.fetch_collection(collectionId);
    return JSON.parse(result);
  } catch (err) {
    console.error('fetchCollection error:', err);
    throw err;
  }
}

/**
 * Fetch all graphic OGIDs for a given collection.
 * @param {number} collectionId
 */
export async function fetchGraphics(collectionId) {
  try {
    const result = await ogBackendActor.fetch_graphics(collectionId);
    return JSON.parse(result);
  } catch (err) {
    console.error('fetchGraphics error:', err);
    throw err;
  }
}

/**
 * Fetch full details for a specific graphic given its OGID.
 * @param {number} ogid
 */
export async function fetchGraphic(ogid) {
  try {
    const result = await ogBackendActor.fetch_graphic(ogid);
    return JSON.parse(result);
  } catch (err) {
    console.error('fetchGraphic error:', err);
    throw err;
  }
}

/*----------------------------------------------------------
  New Asset Retrieval Wrapper
-----------------------------------------------------------*/

/**
 * Minimal IDL for an asset canister exposing a "retrieve" method.
 */
const assetIdlFactory = ({ IDL }) => {
  return IDL.Service({
    'retrieve': IDL.Func([IDL.Text], [IDL.Vec(IDL.Nat8)], ['query']),
  });
};

/**
 * Retrieves an asset image using its graphic OGID.
 * It first fetches the graphic details (including asset path and canister id),
 * then creates an actor for the asset canister and calls its "retrieve" method.
 * The returned blob is converted to a base64 data URL.
 *
 * @param {number} ogid - The on-chain graphic ID.
 * @returns {Promise<string>} - A data URL for the retrieved image.
 */
export async function retrieveAsset(ogid) {
  try {
    // Get graphic details (includes asset path and asset canister id)
    const graphic = await fetchGraphic(ogid);
    const asset = graphic.asset;
    const assetCanisterId = graphic.canister_id;
    
    // Create an actor for the asset canister using the minimal IDL
    const assetActor = Actor.createActor(assetIdlFactory, {
      agent,
      canisterId: assetCanisterId,
    });
    
    // Call the "retrieve" method to get the asset blob (Uint8Array)
    const blob = await assetActor.retrieve(asset);
    
    // Convert Uint8Array to a base64 string for use in an image data URL.
    const uint8Array = new Uint8Array(blob);
    const binaryString = Array.from(uint8Array, byte => String.fromCharCode(byte)).join('');
    const base64String = btoa(binaryString);
    
    // Return as a data URL (assuming PNG format; adjust if needed)
    return `data:image/png;base64,${base64String}`;
  } catch (err) {
    console.error('retrieveAsset error:', err);
    throw err;
  }
}

