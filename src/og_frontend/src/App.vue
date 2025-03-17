<!-- File: src/og_frontend/src/App.vue -->
<template>
  <div id="app">
    <Header />
    <router-view />
  </div>
</template>

<script>
import Header from './components/Header.vue';

export default {
  name: 'App',
  components: {
    Header,
  },
};
</script>

<style>
@font-face {
  font-family: 'CustomFont';
  src: url('/custom.ttf') format('truetype');
  font-weight: normal;
  font-style: normal;
}

body {
  margin: 0;
  background-color: #faf8ff;
  color: #0b0a0a;
  font-family: 'CustomFont', sans-serif;
}

/* Hide scrollbars universally */
* {
  scrollbar-width: none; /* Firefox */
  -ms-overflow-style: none;  /* IE 10+ */
}
*::-webkit-scrollbar {
  display: none; /* Chrome, Safari, Opera */
}
</style>

