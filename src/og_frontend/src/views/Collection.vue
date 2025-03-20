<template>
  <div class="work">
    <!-- Static header: displays "Work > [collection title]" -->
    <div class="work-header">
      <div class="work-title">
        <span class="work-prefix clickable" @click="goBackToWork">Work</span>
        <span class="separator">&gt;</span>
        <span
          class="collection-title-text"
          :class="{ 'gradient-text': loading, 'standard-text': !loading }"
        >
          {{ collection.title || '' }}
        </span>
      </div>
    </div>

    <!-- Fixed gradient divider -->
    <div class="fixed-divider"></div>

    <!-- Grid container for collection graphics -->
    <div class="grid-container">
      <div v-if="loading" class="loading-screen">
        LOADING...
      </div>
      <div v-else class="collections-grid">
        <div
          v-for="graphic in graphics"
          :key="graphic.ogid"
          class="collection-card"
        >
          <div class="card-inner">
            <div class="artwork-container">
              <div class="artwork-inner">
                <img
                  v-if="graphic.imageUrl"
                  :src="graphic.imageUrl"
                  alt="Graphic Image"
                  class="artwork-image"
                  @click="goToGraphic(graphic)"
                />
                <div v-else class="spinner"></div>
              </div>
            </div>

            <!-- Graphic card details -->
            <div class="graphic-details">
              <div class="details-top">
                <div class="graphic-title">
                  {{ graphic.title }}
                </div>
              </div>
              <div class="details-middle">
                <div v-if="graphic.description" class="graphic-description">
                  {{ graphic.description }}
                </div>
              </div>
              <div class="details-bottom">
                <div v-if="graphic.registration_timestamp" class="graphic-timestamp">
                  Registered @ {{ graphic.registration_timestamp }}
                </div>
                <div v-if="graphic.update_timestamp" class="graphic-update">
                  Last Updated @ {{ graphic.update_timestamp }}
                </div>
              </div>
            </div>
          </div>
        </div>
        <div v-if="!graphics.length" class="no-collections">
          No graphics found in this collection.
        </div>
      </div>
    </div>
    <!-- Removed modal overlay -->
  </div>
</template>

<script>
import { ref, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import {
  fetchCollection,
  fetchGraphics,
  fetchGraphic,
  retrieveAsset,
} from '@/apiAgent.js';

export default {
  name: 'Collection',
  setup() {
    const route = useRoute();
    const router = useRouter();
    const collectionId = route.params.id;
    const collection = ref({});
    const graphics = ref([]);
    const loading = ref(true);

    async function loadCollectionData() {
      try {
        const details = await fetchCollection(Number(collectionId));
        collection.value = details;
        const graphicIds = await fetchGraphics(Number(collectionId));
        const graphicPromises = graphicIds.map(async (ogid) => {
          try {
            const gDetails = await fetchGraphic(ogid);
            const imageUrl = await retrieveAsset(ogid);
            return { ...gDetails, imageUrl };
          } catch (error) {
            console.error('Error loading graphic', ogid, error);
            return null;
          }
        });
        const graphicResults = await Promise.all(graphicPromises);
        graphics.value = graphicResults.filter((g) => g !== null);
      } catch (error) {
        console.error('Error loading collection data', error);
      } finally {
        loading.value = false;
      }
    }

    function goBackToWork() {
      router.push({ name: 'Work' });
    }

    function goToGraphic(graphic) {
      // Use collection.value.collection_id (not id) as returned by fetchCollection
      router.push({
        name: 'Graphic',
        params: { collectionId: collection.value.collection_id, graphicId: graphic.ogid },
      });
    }

    onMounted(() => {
      loadCollectionData();
    });

    return {
      collection,
      graphics,
      loading,
      goBackToWork,
      goToGraphic,
    };
  },
};
</script>

<style scoped>
/* Header styles (ensure consistency across pages) */
.work-header {
  position: fixed;
  top: 40px;
  left: 0;
  right: 0;
  height: 80px;
  background-color: #faf8ff;
  display: flex;
  align-items: center;
  padding: 1rem;
  z-index: 1100;
}
.work-title {
  font-size: 2rem;
  font-weight: bold;
  color: #000;
  display: flex;
  align-items: center;
}
.work-prefix {
  cursor: pointer;
  transition: color 0.3s ease;
}
.work-prefix:hover {
  color: #09f95a;
}
.separator {
  font-size: 2rem;
  margin: 0 0.5rem;
  color: #000;
}

/* Force one line for the collection title */
.collection-title-text {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

/* Animated gradient for title while loading */
.gradient-text {
  color: transparent;
  background: linear-gradient(90deg, #09f95a, #bab41c, #f9a207, #ff2217, #fa0e8c, #773ac9, #0861f2, #0aabaa);
  background-clip: text;
  -webkit-background-clip: text;
  animation: gradientLoop 2s linear infinite alternate;
  background-size: 200% auto;
}
.standard-text {
  color: #000;
}

/* Fixed gradient divider */
.fixed-divider {
  position: fixed;
  top: 120px;
  left: 0;
  right: 0;
  height: 2px;
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
  background-size: 200% 100%;
  animation: gradientMove 3s linear infinite alternate;
  z-index: 1150;
  pointer-events: none;
}
@keyframes gradientMove {
  0% { background-position: 0% 50%; }
  100% { background-position: 100% 50%; }
}
@keyframes gradientLoop {
  0% { background-position: 0% center; }
  100% { background-position: 100% center; }
}

/* Grid container: occupy space below header and divider */
.grid-container {
  position: absolute;
  top: 122px;
  bottom: 0;
  left: 0;
  right: 0;
  overflow-y: auto;
  z-index: 2000;
}

/* Consistent loading placeholder */
.loading-screen {
  height: calc(100vh - 122px);
  display: flex;
  justify-content: center;
  align-items: center;
  background: linear-gradient(
    90deg,
    rgba(9,249,90,1) 0%,
    rgba(186,184,28,1) 16%,
    rgba(249,162,7,1) 33%,
    rgba(255,34,23,1) 49%,
    rgba(250,14,140,1) 63%,
    rgba(119,58,201,1) 75%,
    rgba(8,97,242,1) 86%,
    rgba(10,171,166,1) 100%
  );
  background-size: 200% 100%;
  animation: gradientMove 3s linear infinite alternate;
  color: white;
  font-size: 2rem;
  font-weight: bold;
  z-index: 3000;
  text-align: center;
}

/* (Rest of your grid and card styles remain as defined) */

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
@media (max-width: 768px) {
  .work-title {
    font-size: 1rem;
  }
  .separator {
  font-size: 1rem;
  }
}
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

/* Card text sections */
.graphic-details {
  flex: 1;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  margin-top: 0.5rem;
  text-align: center;
}
.details-top,
.details-bottom {
  flex-shrink: 0;
  margin: 0.25rem 0;
}
.details-middle {
  flex: 1;
  display: flex;
  justify-content: center;
  align-items: center;
  flex-direction: column;
}
.graphic-title {
  font-size: 1.2rem;
  font-weight: bold;
  color: #faf8ff;
}
.graphic-description {
  margin-top: 0.5rem;
  font-size: 0.9rem;
  color: #faf8ff;
}
.graphic-timestamp,
.graphic-update {
  margin-top: 0.5rem;
  font-size: 0.8rem;
  color: #afaca9;
}
.no-collections {
  grid-column: 1 / -1;
  text-align: center;
  padding: 2rem;
  color: #0b0a0a;
}
</style>

