<template>
  <div class="graphic-view">
    <!-- Header with breadcrumb: Work > Collection Title > Graphic Title -->
    <div class="work-header">
      <span class="work-link" @click="goToWork">Work</span>
      <span class="separator">&gt;</span>
      <span
        class="collection-link"
        :class="{ 'gradient-title': loading, 'standard-text': !loading }"
        @click="goToCollection"
      >
        {{ collection.title }}
      </span>
      <span class="separator">&gt;</span>
      <span
        class="graphic-title"
        :class="{ 'gradient-title': loading, 'standard-text': !loading }"
      >
        {{ graphic.title }}
      </span>
    </div>

    <!-- Fixed gradient divider -->
    <div class="fixed-divider"></div>

    <!-- Main content area -->
    <div class="content-container" v-if="!loading">
      <div class="graphic-content">
        <!-- Left: Graphic Image -->
        <div class="graphic-image-container">
          <img :src="imageUrl" alt="Graphic Image" class="graphic-image" />
        </div>
        <!-- Right: Details (title, description, timestamps) -->
        <div class="graphic-details">
          <div class="detail-title">{{ graphic.title }}</div>
          <div v-if="graphic.description" class="detail-description">
            {{ graphic.description }}
          </div>
          <div class="detail-timestamp">
            <div v-if="graphic.registration_timestamp">
              Registered @ {{ graphic.registration_timestamp }}
            </div>
            <div v-if="graphic.update_timestamp">
              Last Updated @ {{ graphic.update_timestamp }}
            </div>
          </div>
        </div>
      </div>
    </div>
    <div v-else class="loading-screen">LOADING...</div>
  </div>
</template>

<script>
import { ref, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { fetchGraphic, fetchCollection, retrieveAsset } from '@/apiAgent.js';

export default {
  name: 'Graphic',
  setup() {
    const route = useRoute();
    const router = useRouter();
    const collectionId = route.params.collectionId;
    const graphicId = route.params.graphicId;

    const graphic = ref({});
    const collection = ref({});
    const imageUrl = ref('');
    const loading = ref(true);

    async function loadGraphicData() {
      try {
        // Fetch graphic details and collection details, then retrieve the asset image.
        const gDetails = await fetchGraphic(Number(graphicId));
        graphic.value = gDetails;
        const cDetails = await fetchCollection(Number(collectionId));
        collection.value = cDetails;
        imageUrl.value = await retrieveAsset(Number(graphicId));
      } catch (error) {
        console.error('Error loading graphic data', error);
      } finally {
        loading.value = false;
      }
    }

    function goToWork() {
      router.push({ name: 'Work' });
    }

    function goToCollection() {
      router.push({ name: 'Collection', params: { id: collectionId } });
    }

    onMounted(() => {
      loadGraphicData();
    });

    return {
      graphic,
      collection,
      imageUrl,
      loading,
      goToWork,
      goToCollection,
    };
  },
};
</script>

<style scoped>
/* Header forced to one line */
.work-header {
  position: fixed;
  top: 40px;
  left: 0;
  right: 0;
  height: 80px;
  background-color: #faf8ff;
  display: flex;
  align-items: center;
  z-index: 1100;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

/* Shared header elements */
.work-header .work-link,
.work-header .separator,
.work-header .collection-link,
.work-header .graphic-title {
  font-size: 2rem;
  font-weight: bold;
  cursor: pointer;
  transition: color 0.3s ease;
  padding-top: 2rem;
  padding-left: 1rem;
}

.work-header .work-link:hover {
  color: #09f95a;
}

.work-header .separator {
  margin: 0 0.5rem;
  color: #000;
}

/* Apply animated gradient when loading; otherwise use standard text */
.gradient-title {
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
.standard-text {
  color: #000;
}

/* Keyframes for gradient animation */
@keyframes gradientLoop {
  0% {
    background-position: 0% center;
  }
  100% {
    background-position: 100% center;
  }
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
  0% {
    background-position: 0% 50%;
  }
  100% {
    background-position: 100% 50%;
  }
}

/* Main content container exactly fills the remaining space */
.content-container {
  margin-top: 122px;
  height: calc(100vh - 122px);
  display: flex;
  justify-content: center;
  align-items: center;
  background-color: #0b0a0a;
}

/* Graphic content container: default row layout */
.graphic-content {
  display: flex;
  flex-direction: row;
  width: 100%;
  height: 100%;
  overflow: hidden;
  justify-content: center;
  align-items: center;
}

/* For devices <1000px width, change layout to column */
@media (max-width: 1000px) {
  .graphic-content {
    flex-direction: column;
    padding-top: 10vh;
    padding-left: 0;
  }
}

.graphic-image-container {
  flex: 1;
  display: flex;
  justify-content: center;
  align-items: center;
  padding: 1rem;
}

.graphic-image {
  max-width: 100%;
  max-height: 80vh;
  object-fit: contain;
}

.graphic-details {
  flex: 1;
  display: flex;
  flex-direction: column;
  justify-content: center;
  padding: 1rem;
  color: #faf8ff;
}

.detail-title {
  font-size: 2rem;
  font-weight: bold;
  margin-bottom: 1rem;
}

.detail-description {
  font-size: 1.2rem;
  margin-bottom: 1rem;
}

.detail-timestamp {
  font-size: 0.9rem;
  color: #afaca9;
}

/* Consistent loading placeholder */
.loading-screen {
  height: 100vh;
  display: flex;
  justify-content: center;
  align-items: center;
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

@media (max-width: 768px) {
.work-header .work-link,
.work-header .separator,
.work-header .collection-link,
.work-header .graphic-title {
  font-size: 1rem;
}
}
</style>

