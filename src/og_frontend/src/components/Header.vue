<template>
  <header class="header">
    <div class="header-left">
      <router-link to="/" class="logo-link">
        <img src="/logo.png" alt="Logo" class="logo" />
        <span class="brand">onchain.graphics</span>
      </router-link>
    </div>
    <!-- Desktop navigation menu (hidden on mobile) -->
    <nav class="header-right desktop-menu">
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
    <!-- Mobile hamburger menu (visible only on mobile) -->
    <div class="mobile-menu">
      <HamburgerMenu />
    </div>
  </header>
</template>

<script>
import HamburgerMenu from './HamburgerMenu.vue';

export default {
  name: 'Header',
  components: {
    HamburgerMenu,
  },
};
</script>

<style scoped>
.header {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  height: 80px;
  background-color: #faf8ff;
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0 1rem;
  z-index: 2000; /* Lower than the modal overlay's z-index */
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

/* Desktop menu styles */
.desktop-menu .menu {
  list-style: none;
  display: flex;
  margin: 0;
  padding: 0;
}

.desktop-menu .menu-item {
  margin-left: 1rem;
}

.desktop-menu .menu-link {
  text-decoration: none;
  color: #afaca9;
  font-size: 1rem;
  transition: color 0.3s ease, background 0.3s ease;
}

.desktop-menu .menu-link:hover {
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

.desktop-menu .menu-link.router-link-active {
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

/* Hide mobile menu on desktop */
.mobile-menu {
  display: none;
}

/* Responsive adjustments for mobile (screens 768px and below) */
@media (max-width: 768px) {
  .brand {
    font-size: 1.2rem;
  }
  .desktop-menu {
    display: none;
  }
  .mobile-menu {
    display: block;
  }
}
</style>

