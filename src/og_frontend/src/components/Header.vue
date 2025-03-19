<!-- File: src/og_frontend/src/components/Header.vue -->
<template>
  <header class="header">
    <div class="header-left">
      <router-link to="/" class="logo-link">
        <img src="/logo.png" alt="Logo" class="logo" />
        <span class="brand">onchain.graphics</span>
      </router-link>
    </div>
    <nav class="header-right">
      <ul class="menu">
        <li class="menu-item">
          <router-link to="/about" class="menu-link">About</router-link>
        </li>
        <li class="menu-item">
          <router-link to="/work" class="menu-link">Work</router-link>
        </li>
        <li class="menu-item">
          <router-link to="/roadmap" class="menu-link">Roadmap</router-link>
        </li>
        <li class="menu-item">
          <router-link to="/contact" class="menu-link">Contact</router-link>
        </li>
      </ul>
    </nav>
  </header>
</template>

<script>
export default {
  name: 'Header',
};
</script>

<style scoped>
.header {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  height: 80px; /* Adjust height as needed */
  background-color: #faf8ff;
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0 1rem;
  z-index: 2000;
}

.header-left {
  display: flex;
  align-items: center;
}

.logo-link {
  display: flex;
  align-items: center;
  text-decoration: none;
  color: inherit;
}

.logo {
  height: 40px;
  margin-right: 0.5rem;
}

.brand {
  font-size: 1.5rem;
  font-weight: bold;
}

.menu {
  list-style: none;
  display: flex;
  margin: 0;
  padding: 0;
}

.menu-item {
  margin-left: 1rem;
}

.menu-link {
  text-decoration: none;
  color: #afaca9;
  font-size: 1rem;
  transition: color 0.3s ease, background 0.3s ease;
}

/* Hover effect: animated gradient instead of underline */
.menu-link:hover {
  color: transparent;
  background: linear-gradient(90deg, #09f95a, #bab41c, #f9a207, #ff2217, #fa0e8c, #773ac9, #0861f2, #0aabaa);
  background-clip: text;
  -webkit-background-clip: text;
  animation: gradientLoop 2s linear infinite alternate;
  background-size: 200% auto;
}

.menu-link.router-link-active {
  color: #0b0a0a;
}

@keyframes gradientLoop {
  0% {
    background-position: 0% center;
  }
  100% {
    background-position: 100% center;
  }
}
</style>

