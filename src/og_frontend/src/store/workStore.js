// File: src/og_frontend/src/store/workStore.js
import { defineStore } from 'pinia';
import { fetchCollections, fetchCollection, fetchGraphics, retrieveAsset } from '@/apiAgent.js';

export const useWorkStore = defineStore('work', {
  state: () => ({
    collections: [],
    loading: false,
  }),
  actions: {
    async loadCollections() {
      // If collections are already loaded, do nothing.
      if (this.collections.length > 0) return;
      this.loading = true;
      try {
        const collectionIds = await fetchCollections();
        const promises = collectionIds.map(async (id) => {
          try {
            const details = await fetchCollection(id);
            const graphics = await fetchGraphics(id);
            return {
              id,
              title: details.title || `Collection ${id}`,
              description: details.description || '',
              artist: details.artist || '',
              external_link: details.external_link || '',
              registration_timestamp: details.registration_timestamp || '',
              update_timestamp: details.update_timestamp || '', // new field
              graphics,
              imageUrl: '',
            };
          } catch (err) {
            console.error("Error loading collection", id, err);
            return null;
          }
        });
        const results = await Promise.all(promises);
        this.collections = results.filter(c => c !== null);
      } catch (err) {
        console.error("Failed to load collections", err);
      } finally {
        this.loading = false;
      }
    },
    async updateArtwork() {
      await Promise.all(
        this.collections.map(async (collection, index) => {
          if (collection.graphics && collection.graphics.length > 0) {
            const randomIndex = Math.floor(Math.random() * collection.graphics.length);
            const graphicId = collection.graphics[randomIndex];
            try {
              const imageUrl = await retrieveAsset(graphicId);
              this.collections[index].imageUrl = imageUrl;
            } catch (err) {
              console.error("Error retrieving asset for collection", collection.id, err);
            }
          }
        })
      );
    },
  },
});

