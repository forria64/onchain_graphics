<template>
  <div class="work">
    <!-- Global header always visible -->
    <div class="work-header">
      <div class="work-title">Work</div>
      <div class="sort-buttons">
        <button
          :class="{ active: sortType === 'alphabetical' }"
          @click="sortAlphabetically"
        >
          Sort A–Z
        </button>
        <button
          :class="{ active: sortType === 'chronological' }"
          @click="sortChronologically"
        >
          Sort Recent
        </button>
      </div>
    </div>

    <!-- Fixed gradient divider -->
    <div class="fixed-divider"></div>

    <!-- Scrollable grid container -->
    <div class="grid-container">
      <!-- LOADING SCREEN -->
      <div v-if="store.loading" class="loading-screen">
        LOADING...
      </div>

      <!-- COLLECTION GRID -->
      <div v-else class="collections-grid">
        <div
          v-for="collection in store.collections"
          :key="collection.id"
          class="collection-card"
        >
          <div class="card-inner">
            <!-- Artwork container clickable to navigate to Collection.vue -->
            <div class="artwork-container" @click="goToCollection(collection.id)">
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

            <!-- Card layout with top, middle, bottom regions. -->
            <div class="collection-details">
              <!-- Always-present top -->
              <div class="details-top">
                <div class="collection-title">
                  {{ collection.title }}
                </div>
              </div>

              <!-- Optional fields in the middle -->
              <div class="details-middle">
                              <div class="collection-artist" v-if="collection.artist">
                  by {{ collection.artist }}
                </div>
                <div
                  class="collection-description"
                  v-if="collection.description"
                >
                  {{ collection.description }}
                </div>

                <div
                  class="collection-external-link"
                  v-if="collection.external_link"
                >
                  <a :href="collection.external_link" target="_blank">
                    VISIT EXTERNAL LINK
                  </a>
                </div>
              </div>

              <!-- Timestamps pinned at bottom -->
              <div class="details-bottom">
                <div class="collection-timestamp">
                  Registered @ {{ collection.registration_timestamp }}
                </div>
                <div class="collection-update" v-if="collection.update_timestamp">
                  Last Updated @ {{ collection.update_timestamp }}
                </div>
              </div>
            </div>
            <!-- end .collection-details -->
          </div>
          <!-- end .card-inner -->
        </div>
        <!-- end .collection-card -->

      </div>
      <!-- end .collections-grid -->
    </div>
    <!-- end .grid-container -->
  </div>
</template>

<script>
import { onMounted, onUnmounted, ref } from 'vue';
import { useRouter } from 'vue-router';
import { useWorkStore } from '@/store/workStore';

export default {
  name: 'Work',
  setup() {
    const router = useRouter();
    const store = useWorkStore();
    const sortType = ref('chronological');
    let timer = null;

    function sortAlphabetically() {
      sortType.value = 'alphabetical';
      store.collections.sort((a, b) => a.title.localeCompare(b.title));
    }

    function sortChronologically() {
      sortType.value = 'chronological';
      // Sort based solely on registration_timestamp (ignoring update_timestamp)
      store.collections.sort(
        (a, b) =>
          new Date(b.registration_timestamp) - new Date(a.registration_timestamp)
      );
    }

    function goToCollection(id) {
      router.push({ name: 'Collection', params: { id } });
    }

    onMounted(async () => {
      if (!store.collections.length) {
        store.loading = true;
        await store.loadCollections();
        await store.updateArtwork();
        store.loading = false;
      }
      timer = setInterval(() => {
        store.updateArtwork();
      }, 3000);
    });

    onUnmounted(() => {
      if (timer) clearInterval(timer);
    });

    return {
      store,
      sortType,
      sortAlphabetically,
      sortChronologically,
      goToCollection,
    };
  },
};
</script>

<style scoped>
.work {
  position: relative;
  height: 100vh;
  overflow: hidden;
  background-color: #faf8ff;
  color: #0b0a0a;
  box-sizing: border-box;
  padding-top: 80px;
}

.work-header {
  position: fixed;
  top: 40px;
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
  font-size: 1rem;
  cursor: pointer;
  color: #afaca9;
  transition: color 0.3s ease, background 0.3s ease;
}

.sort-buttons button.active {
  color: #0b0a0a;
}

.sort-buttons button:not(.active):hover {
  color: transparent;
  background: linear-gradient(
    90deg,
    #09f95a,
    #bab41c,
    #f9a207,
    #ff2217,
    #fa0e8c,
    #773ac9,
    #0861f2,
    #0aabaa
  );
  background-clip: text;
  -webkit-background-clip: text;
  animation: gradientLoop 2s linear infinite alternate;
  background-size: 200% auto;
}

@keyframes gradientLoop {
  0% {
    background-position: 0% center;
  }
  100% {
    background-position: 100% center;
  }
}

.fixed-divider {
  position: fixed;
  top: 120px;
  left: 0;
  right: 0;
  height: 2px;
  background: linear-gradient(
    90deg,
    rgba(9, 249, 90, 1) 0%,
    rgba(186, 184, 28, 1) 16%,
    rgba(249, 162, 7, 1) 33%,
    rgba(255, 34, 23, 1) 49%,
    rgba(250, 14, 140, 1) 63%,
    rgba(119, 58, 201, 1) 75%,
    rgba(8, 97, 242, 1) 86%,
    rgba(10, 171, 166, 1) 100%
  );
  background-size: 200% 100%;
  animation: gradientMove 3s linear infinite alternate;
  z-index: 1150;
  pointer-events: none;
}

@keyframes gradientMove {
  0% {
    background-position: 0% 50%;
  }
  100% {
    background-position: 100% 50%;
  }
}

.grid-container {
  position: absolute;
  top: 122px;
  bottom: 0;
  left: 0;
  right: 0;
  overflow-y: auto;
  z-index: 2000;
}

.loading-screen {
  position: absolute;
  top: 0;
  right: 0;
  bottom: 0;
  left: 0;
  background: linear-gradient(
    90deg,
    rgba(9, 249, 90, 1) 0%,
    rgba(186, 184, 28, 1) 16%,
    rgba(249, 162, 7, 1) 33%,
    rgba(255, 34, 23, 1) 49%,
    rgba(250, 14, 140, 1) 63%,
    rgba(119, 58, 201, 1) 75%,
    rgba(8, 97, 242, 1) 86%,
    rgba(10, 171, 166, 1) 100%
  );
  background-size: 200% 100%;
  animation: gradientMove 3s linear infinite alternate;
  display: flex;
  justify-content: center;
  align-items: center;
  color: white;
  font-size: 2rem;
  font-weight: bold;
  z-index: 3000;
  text-align: center;
}

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

.collection-card {
  position: relative;
  padding: 2px;
  background: linear-gradient(
    90deg,
    rgba(9, 249, 90, 1) 0%,
    rgba(186, 184, 28, 1) 16%,
    rgba(249, 162, 7, 1) 33%,
    rgba(255, 34, 23, 1) 49%,
    rgba(250, 14, 140, 1) 63%,
    rgba(119, 58, 201, 1) 75%,
    rgba(8, 97, 242, 1) 86%,
    rgba(10, 171, 166, 1) 100%
  );
  background-size: 200% 100%;
  border-radius: 10px;
  overflow: hidden;
  animation: gradientMove 3s linear infinite alternate;
  transition: padding 0.3s ease;
}

.collection-card:hover {
  padding: 5px;
}

.card-inner {
  border-radius: 8px;
  background-color: #0b0a0a;
  padding: 1rem;
  width: 100%;
  height: 100%;
  box-sizing: border-box;
  display: flex;
  flex-direction: column;
}

.artwork-container {
  width: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
  cursor: pointer;
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

/*
   .collection-details is flex with:
   - details-top pinned at top
   - details-middle (optional fields) centered
   - details-bottom pinned at bottom
   - all text center-aligned
*/
.collection-details {
  flex: 1;
  display: flex;
  flex-direction: column;
  justify-content: space-between; /* top & bottom pinned */
  margin-top: 0.5rem;
  text-align: center; /* center-align everything */
}

.details-top,
.details-bottom {
  flex-shrink: 0;
  margin: 0.25rem 0;
}

/* Middle region grows or shrinks to fill space */
.details-middle {
  display: flex;
  flex: 1;
  flex-direction: column;
  justify-content: center;
  align-items: center;
}

.collection-title {
  font-size: 2rem;
  font-weight: bold;
  color: #faf8ff;
}

.collection-description {
  margin-top: 0.5rem;
  font-size: 0.9rem;
  color: #faf8ff; /* Updated color */
}

.collection-artist {
  margin-top: 0.5rem;
  font-size: 0.9rem;
  color: #faf8ff; /* Updated color */
}

.collection-external-link {
  margin-top: 0.5rem;
}

.collection-external-link a {
  color: #faf8ff;
  text-decoration: none;
}

.collection-external-link a:hover {
  color: transparent;
  text-shadow: none;
  background: linear-gradient(
    90deg,
    #09f95a,
    #bab41c,
    #f9a207,
    #ff2217,
    #fa0e8c,
    #773ac9,
    #0861f2,
    #0aabaa
  );
  background-clip: text;
  -webkit-background-clip: text;
  animation: gradientLoop 2s linear infinite alternate;
  background-size: 200% auto;
}

.collection-timestamp,
.collection-update {
  margin-top: 0.5rem;
  font-size: 0.8rem;
  color: #afaca9;
}

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
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
}

.work-title span {
  vertical-align: middle;
}

/* Mobile media query: work-title reduced to 1rem */
@media (max-width: 768px) {
  .work-title {
    font-size: 1rem;
  }
  .sort-buttons button {
  font-size: 1rem;}
}
</style>

