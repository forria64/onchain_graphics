import { createRouter, createWebHistory } from 'vue-router';
import Home from '../views/Home.vue';
import About from '../views/About.vue';
import Work from '../views/Work.vue';
import Collection from '../views/Collection.vue';
import Graphic from '../views/Graphic.vue'; // <-- Import new view
import Roadmap from '../views/Roadmap.vue';
import Contact from '../views/Contact.vue';

const routes = [
  { path: '/', name: 'Home', component: Home },
  { path: '/about', name: 'About', component: About },
  { path: '/work', name: 'Work', component: Work },
  { path: '/collection/:id', name: 'Collection', component: Collection },
  { path: '/graphic/:collectionId/:graphicId', name: 'Graphic', component: Graphic }, // New route
  { path: '/roadmap', name: 'Roadmap', component: Roadmap },
  { path: '/contact', name: 'Contact', component: Contact },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;

