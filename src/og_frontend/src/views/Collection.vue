<template>
  <div class="work">
    <!-- Static header: displays "Work > [collection title]" -->
    <div class="work-header">
      <div class="work-title">
        <span class="work-prefix clickable" @click="goBackToWork">Work</span>
        <span class="separator"> > </span>
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
        <!-- 
          Each graphic card now matches the style from Work.vue:
          1) Title at the top
          2) Optional fields in the middle (description)
          3) Timestamps pinned at the bottom
          4) All text center-aligned
        -->
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
                  @click="openModal(graphic.imageUrl)"
                />
                <div v-else class="spinner"></div>
              </div>
            </div>

            <!-- 
              .graphic-details is now a flex column:
              - .details-top pinned at top
              - .details-middle grows to center optional fields
              - .details-bottom pinned at bottom
            -->
            <div class="graphic-details">
              <!-- Title always present at top -->
              <div class="details-top">
                <div class="graphic-title">
                  {{ graphic.title }}
                </div>
              </div>

              <!-- Optional fields in the middle -->
              <div class="details-middle">
                <div v-if="graphic.description" class="graphic-description">
                  {{ graphic.description }}
                </div>
              </div>

              <!-- Timestamps pinned to the bottom -->
              <div class="details-bottom">
                <div
                  v-if="graphic.registration_timestamp"
                  class="graphic-timestamp"
                >
                  Registered @ {{ graphic.registration_timestamp }}
                </div>
                <div v-if="graphic.update_timestamp" class="graphic-update">
                  Last Updated @ {{ graphic.update_timestamp }}
                </div>
              </div>
            </div>
            <!-- end .graphic-details -->
          </div>
          <!-- end .card-inner -->
        </div>
        <!-- end .collection-card -->

        <div v-if="!graphics.length" class="no-collections">
          No graphics found in this collection.
        </div>
      </div>
      <!-- end .collections-grid -->
    </div>

    <!-- Modal overlay for high-res artwork -->
    <div v-if="modalVisible" class="modal-overlay" @click.self="closeModal">
      <div class="modal-content">
        <img :src="modalImage" alt="High Resolution Graphic" class="modal-image" />
      </div>
      <button class="modal-close" @click="closeModal">✕</button>
    </div>
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
    const modalVisible = ref(false);
    const modalImage = ref('');

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

    function openModal(imageUrl) {
      modalImage.value = imageUrl;
      modalVisible.value = true;
    }

    function closeModal() {
      modalVisible.value = false;
      modalImage.value = '';
    }

    function goBackToWork() {
      router.push({ name: 'Work' });
    }

    onMounted(() => {
      loadCollectionData();
    });

    return {
      collection,
      graphics,
      loading,
      modalVisible,
      modalImage,
      openModal,
      closeModal,
      goBackToWork,
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
  align-items: center;
  padding: 1rem;
  z-index: 1100;
}

.work-title {
  font-size: 2rem;
  font-weight: bold;
  color: #000000;
}

.work-prefix {
  transition: all 0.3s ease;
  cursor: pointer;
  color: #000000;
}
.work-prefix:hover {
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

.separator,
.collection-title-text.standard-text {
  color: #000000;
}

.collection-title-text.gradient-text {
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
   .graphic-details is flex with top, middle, and bottom regions
   pinned in place. All text is center-aligned.
*/
.graphic-details {
  flex: 1;
  display: flex;
  flex-direction: column;
  justify-content: space-between; /* top & bottom pinned */
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
  color: #faf8ff; /* same as collection-description */
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

/* 
   Modal overlay for high-res image 
*/
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.95);
  z-index: 4000;
  display: flex;
  justify-content: center;
  align-items: center;
  overflow: hidden;
}
.modal-content {
  position: relative;
  width: 100vw;
  height: 100vh;
  padding: 100px;
  box-sizing: border-box;
}
.modal-image {
  width: 100%;
  height: 100%;
  object-fit: contain;
  display: block;
}
.modal-close {
  position: fixed;
  top: 30px;
  right: 30px;
  font-size: 3rem;
  font-weight: bold;
  color: #ffffff;
  text-shadow: 0 0 10px #ffffff;
  background: none;
  border: none;
  cursor: pointer;
  transition: color 0.3s ease, text-shadow 0.3s ease, background 0.3s ease;
}
.modal-close:hover {
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
</style>

