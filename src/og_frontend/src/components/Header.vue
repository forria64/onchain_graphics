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
}

.menu-link.router-link-active {
  color: #0b0a0a;
}

.menu-link:hover {
  text-decoration: underline;
}
</style>

