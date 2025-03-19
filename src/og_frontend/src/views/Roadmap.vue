<!-- File: src/og_frontend/src/views/Roadmap.vue -->
<template>
  <div class="roadmap">
    <!-- Fixed header with page title -->
    <div class="work-header">
      <div class="work-title">Roadmap</div>
    </div>

    <!-- Fixed gradient divider -->
    <div class="fixed-divider"></div>

    <!-- Content container with placeholder -->
    <div class="content-container">
      <div class="placeholder">
        UNDER CONSTRUCTION...
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'Roadmap',
};
</script>

<style scoped>
.roadmap {
  position: relative;
  height: 100vh;
  background-color: #faf8ff;
  color: #0b0a0a;
  box-sizing: border-box;
  padding-top: 80px;
}

/* Fixed header (reuse Work.vue header style) */
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

/* Fixed gradient divider */
.fixed-divider {
  position: fixed;
  top: 120px;
  left: 0;
  right: 0;
  height: 2px;
  background: linear-gradient(90deg, rgba(9,249,90,1) 0%, rgba(186,184,28,1) 16%, rgba(249,162,7,1) 33%, rgba(255,34,23,1) 49%, rgba(250,14,140,1) 63%, rgba(119,58,201,1) 75%, rgba(8,97,242,1) 86%, rgba(10,171,166,1) 100%);
  background-size: 200% 100%;
  animation: gradientMove 3s linear infinite alternate;
  z-index: 1150;
  pointer-events: none;
}
@keyframes gradientMove {
  0% { background-position: 0% 50%; }
  100% { background-position: 100% 50%; }
}

/* Content container for placeholder */
.content-container {
  margin-top: 0px;
  height: calc(100vh - 122px);
  display: flex;
  justify-content: center;
  align-items: center;
}

/* Placeholder content with animated gradient background */
.placeholder {
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, #09f95a, #bab41c, #f9a207, #ff2217, #fa0e8c, #773ac9, #0861f2, #0aabaa);
  background-size: 200% 200%;
  animation: gradientMove 3s linear infinite alternate;
  display: flex;
  justify-content: center;
  align-items: center;
  font-size: 3rem;
  color: #ffffff;
  font-weight: bold;
  text-transform: uppercase;
  letter-spacing: 2px;
}
</style>

