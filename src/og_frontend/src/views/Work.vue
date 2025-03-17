<template>
  <div class="work">
    <!-- Global header always visible -->
    <div class="work-header">
      <div class="work-title">Work</div>
      <div class="sort-buttons">
        <button :class="{ active: sortType === 'alphabetical' }" @click="sortAlphabetically">
          Sort A–Z
        </button>
        <button :class="{ active: sortType === 'chronological' }" @click="sortChronologically">
          Sort Recent
        </button>
      </div>
    </div>

    <!-- Fixed gradient divider -->
    <div class="fixed-divider"></div>

    <!-- Scrollable grid container -->
    <div class="grid-container">
      <!-- LOADING SCREEN -->
      <div v-if="loading" class="loading-screen">
        LOADING...
      </div>

      <!-- COLLECTION GRID -->
      <div v-else class="collections-grid">
        <div
          v-for="collection in collections"
          :key="collection.id"
          class="collection-card"
        >
          <!-- Inner container to hold card content with equal padding -->
          <div class="card-inner">
            <div class="artwork-container">
              <div class="artwork-inner">
                <img
                  v-if="collection.imageUrl"
                  :src="collection.imageUrl"
                  alt="Artwork"
                  class="artwork-image"
                />
                <div v-else class="spinner"></div>
              </div>
            </div>
            <div class="collection-details">
              <div class="collection-title">{{ collection.title }}</div>
              <div class="collection-description" v-if="collection.description">
                {{ collection.description }}
              </div>
              <div class="collection-artist" v-if="collection.artist">
                Artist: {{ collection.artist }}
              </div>
              <div class="collection-external-link" v-if="collection.external_link">
                <a :href="collection.external_link" target="_blank">VISIT EXTERNAL LINK</a>
              </div>
              <div class="collection-timestamp">
                Registered @ {{ collection.registration_timestamp }}
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { ref, onMounted, onUnmounted } from 'vue';
import { fetchCollections, fetchCollection, fetchGraphics, retrieveAsset } from '@/apiAgent.js';

export default {
  name: 'Work',
  setup() {
    const collections = ref([]);
    const sortType = ref('chronological');
    const loading = ref(true);  // <-- NEW
    let timer = null;

    async function loadCollections() {
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
              graphics,
              imageUrl: '',
            };
          } catch (err) {
            console.error("Error loading collection", id, err);
            return null;
          }
        });
        const results = await Promise.all(promises);
        collections.value = results.filter(c => c !== null);
      } catch (err) {
        console.error("Failed to load collections", err);
      }
    }

    async function updateArtwork() {
      await Promise.all(
        collections.value.map(async (collection) => {
          if (collection.graphics && collection.graphics.length > 0) {
            const randomIndex = Math.floor(Math.random() * collection.graphics.length);
            const graphicId = collection.graphics[randomIndex];
            try {
              const imageUrl = await retrieveAsset(graphicId);
              collection.imageUrl = imageUrl;
            } catch (err) {
              console.error("Error retrieving asset for collection", collection.id, err);
            }
          }
        })
      );
    }

    function sortAlphabetically() {
      sortType.value = 'alphabetical';
      collections.value.sort((a, b) => a.title.localeCompare(b.title));
    }

    function sortChronologically() {
      sortType.value = 'chronological';
      collections.value.sort((a, b) =>
        BigInt(b.registration_timestamp || '0') - BigInt(a.registration_timestamp || '0')
      );
    }

    onMounted(async () => {
      loading.value = true;   // Start loading
      await loadCollections();
      await updateArtwork();
      loading.value = false;  // Finished loading

      timer = setInterval(updateArtwork, 3000);
    });

    onUnmounted(() => {
      if (timer) clearInterval(timer);
    });

    return {
      collections,
      sortType,
      loading,
      sortAlphabetically,
      sortChronologically,
    };
  },
};
</script>

<style scoped>
/* Reserve space for the global menu header (assumed to be 80px high) */
.work {
  position: relative;
  height: 100vh;
  overflow: hidden;
  background-color: #faf8ff;
  color: #0b0a0a;
  box-sizing: border-box;
  padding-top: 80px;
}

/* Work header fixed below the global header */
.work-header {
  position: fixed;
  top: 40px; /* Adjust based on your global header height */
  left: 0;
  right: 0;
  height: 80px;
  background-color: #faf8ff;
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem;
  z-index: 1100;
}
.work-title {
  font-size: 2rem;
  font-weight: bold;
  color: #0b0a0a;
}
.sort-buttons {
  display: flex;
  gap: 0.5rem;
}
.sort-buttons button {
  background: transparent;
  border: none;
  font-size: 0.9rem;
  cursor: pointer;
  color: #afaca9;
}
.sort-buttons button.active {
  color: #0b0a0a;
}

/* Fixed gradient divider */
.fixed-divider {
  position: fixed;
  top: 120px; /* 40px (global header) + 80px (work header) */
  left: 0;
  right: 0;
  height: 2px;
  background: linear-gradient(90deg, rgba(9,249,90,1) 0%, rgba(186,184,28,1) 16%, rgba(249,162,7,1) 33%, rgba(255,34,23,1) 49%, rgba(250,14,140,1) 63%, rgba(119,58,201,1) 75%, rgba(8,97,242,1) 86%, rgba(10,171,166,1) 100%);
  background-size: 200% 100%;
  animation: gradientMove 3s linear infinite alternate;
  z-index: 1150;
  pointer-events: none;
}

/* Keyframes for the gradient animation */
@keyframes gradientMove {
  0% { background-position: 0% 50%; }
  100% { background-position: 100% 50%; }
}

/* Grid container starts immediately below the divider */
.grid-container {
  position: absolute;
  top: 122px;
  bottom: 0;
  left: 0;
  right: 0;
  overflow-y: auto;
  z-index: 2000;
}

/* LOADING SCREEN */
.loading-screen {
  /* Make it fill the entire grid container space */
  position: absolute;
  top: 0;
  right: 0;
  bottom: 0;
  left: 0;

  /* Same gradient as your divider / card animations */
  background: linear-gradient(90deg, rgba(9,249,90,1) 0%, rgba(186,184,28,1) 16%, rgba(249,162,7,1) 33%, rgba(255,34,23,1) 49%, rgba(250,14,140,1) 63%, rgba(119,58,201,1) 75%, rgba(8,97,242,1) 86%, rgba(10,171,166,1) 100%);
  background-size: 200% 100%;
  animation: gradientMove 3s linear infinite alternate;

  display: flex;
  justify-content: center;
  align-items: center;
  color: white;
  font-size: 2rem;
  font-weight: bold;
  z-index: 3000; /* On top of everything else */
  text-align: center;
}

/* Collection grid */
.collections-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 1rem;
  padding: 1rem;
}
@media (max-width: 1200px) {
  .collections-grid {
    grid-template-columns: repeat(3, 1fr);
  }
}
@media (max-width: 768px) {
  .collections-grid {
    grid-template-columns: repeat(2, 1fr);
  }
}
@media (max-width: 480px) {
  .collections-grid {
    grid-template-columns: 1fr;
  }
}

/* Collection card: outer container with 2px gradient border */
.collection-card {
  position: relative;
  padding: 2px;
  background: linear-gradient(90deg, rgba(9,249,90,1) 0%, rgba(186,184,28,1) 16%, rgba(249,162,7,1) 33%, rgba(255,34,23,1) 49%, rgba(250,14,140,1) 63%, rgba(119,58,201,1) 75%, rgba(8,97,242,1) 86%, rgba(10,171,166,1) 100%);
  background-size: 200% 100%;
  border-radius: 10px;
  overflow: hidden;
  animation: gradientMove 3s linear infinite alternate;
  transition: padding 0.3s ease;
}
.collection-card:hover {
  padding: 5px;
}

/* Inner container for card content */
.card-inner {
  border-radius: 8px;
  background-color: #0b0a0a;
  padding: 1rem;
  width: 100%;
  height: 100%;
  box-sizing: border-box;
}

/* Artwork container centered */
.artwork-container {
  width: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
}
.artwork-inner {
  width: 100%;
  aspect-ratio: 1 / 1;
  padding: 0.5rem;
  background-color: #0b0a0a;
  display: flex;
  align-items: center;
  justify-content: center;
  overflow: hidden;
}
.artwork-image {
  max-width: 100%;
  max-height: 100%;
  object-fit: contain;
}
.collection-details {
  width: 100%;
  padding: 0.5rem;
  text-align: center;
  color: #faf8ff;
}
.collection-title {
  font-size: 2rem;
  font-weight: bold;
  color: #0b0a0a;
}
.collection-description {
  margin-top: 0.5rem;
  font-size: 0.9rem;
}
.collection-artist {
  margin-top: 0.5rem;
  font-size: 0.9rem;
}
.collection-external-link {
  margin-top: 0.5rem;
}
.collection-external-link a {
  color: #faf8ff;
  text-decoration: underline;
}
.collection-timestamp {
  margin-top: 0.5rem;
  font-size: 0.8rem;
  color: #afaca9;
}

/* Spinner styling */
.spinner {
  border: 4px solid #f3f3f3;
  border-top: 4px solid #0b0a0a;
  border-radius: 50%;
  width: 30px;
  height: 30px;
  animation: spin 1s linear infinite;
  margin: auto;
}
@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}
</style>

