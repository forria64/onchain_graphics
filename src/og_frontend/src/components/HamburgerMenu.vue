<template>
  <div class="hamburger-menu">
    <button class="hamburger-button" @click="toggleMenu">
      <span class="bar" v-for="n in 3" :key="n"></span>
    </button>
    <!-- Teleport the overlay to the body so it appears on top of everything -->
    <teleport to="body">
      <transition name="slide-down">
        <div
          v-if="isOpen"
          class="menu-overlay"
          @click.self="closeMenu"
          @touchstart="handleTouchStart"
          @touchend="handleTouchEnd"
        >
          <ul class="mobile-menu-list">
            <li class="mobile-menu-item">
              <router-link to="/about" @click="closeMenu">About</router-link>
            </li>
            <li class="mobile-menu-item">
              <router-link to="/work" @click="closeMenu">Work</router-link>
            </li>
            <li class="mobile-menu-item">
              <router-link to="/roadmap" @click="closeMenu">Roadmap</router-link>
            </li>
            <li class="mobile-menu-item">
              <router-link to="/contact" @click="closeMenu">Contact</router-link>
            </li>
          </ul>
        </div>
      </transition>
    </teleport>
  </div>
</template>

<script>
export default {
  name: "HamburgerMenu",
  data() {
    return {
      isOpen: false,
      startY: null, // used for swipe-up detection
    };
  },
  methods: {
    toggleMenu() {
      this.isOpen = !this.isOpen;
    },
    closeMenu() {
      this.isOpen = false;
    },
    handleTouchStart(e) {
      this.startY = e.touches[0].clientY;
    },
    handleTouchEnd(e) {
      const endY = e.changedTouches[0].clientY;
      // If the swipe-up distance is greater than 50px, close the menu
      if (this.startY - endY > 50) {
        this.closeMenu();
      }
    },
  },
};
</script>

<style scoped>
.hamburger-menu {
  position: relative;
}

/* Updated hamburger button bars */
.hamburger-button {
  background: none;
  border: none;
  cursor: pointer;
  padding: 0.5rem;
  width: 40px;
  height: 40px;
  display: flex;
  flex-direction: column;
  justify-content: center;
}

.bar {
  display: block;
  width: 100%;
  height: 6px;
  margin: 3px 0;
  border-radius: 3px;
  /* Use an animated gradient like the divider */
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
  background-size: 200% auto;
  animation: gradientLoop 2s linear infinite alternate;
}

@keyframes gradientLoop {
  0% {
    background-position: 0% center;
  }
  100% {
    background-position: 100% center;
  }
}

/* Transition classes for smooth slide-in from the top and slide-out to the top */
.slide-down-enter-active,
.slide-down-leave-active {
  transition: transform 0.5s ease;
}
.slide-down-enter-from,
.slide-down-leave-to {
  transform: translateY(-100%);
}
.slide-down-enter-to,
.slide-down-leave-from {
  transform: translateY(0);
}

/* Mobile menu overlay styles */
.menu-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: #000000;
  z-index: 9999999;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
}

/* Mobile menu list */
.mobile-menu-list {
  list-style: none;
  padding: 0;
  margin: 0;
  text-align: center;
}

/* Mobile menu items */
.mobile-menu-item a {
  text-decoration: none;
  color: #ffffff;
  font-size: 2.5rem;
  font-weight: 600;
  text-shadow: 0 0 10px #ffffff;
  transition: color 0.3s ease, background 0.3s ease;
}

/* When a menu link is active (i.e. selected) apply the animated gradient */
.mobile-menu-item a.router-link-active {
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
  background-size: 200% auto;
  background-clip: text;
  -webkit-background-clip: text;
  text-shadow: none;
  animation: gradientLoop 2s linear infinite alternate;
}

/* Optionally, also update non-active links on hover */
.mobile-menu-item a:hover {
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
  background-size: 200% auto;
  background-clip: text;
  -webkit-background-clip: text;
  animation: gradientLoop 2s linear infinite alternate;
}
</style>

