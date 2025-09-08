<template>
  <nav class="navbar">
    <div class="nav-container">
      <div class="nav-brand">
        <router-link to="/" class="brand-link">
          📁 File Organizer
        </router-link>
      </div>
      
      <ul class="nav-menu">
        <li class="nav-item">
          <router-link to="/" class="nav-link" exact-active-class="active">
            🏠 Home
          </router-link>
        </li>
        <li class="nav-item">
          <router-link to="/file-manager" class="nav-link" active-class="active">
            🗂️ File Manager
          </router-link>
        </li>
        <li class="nav-item">
          <router-link to="/about" class="nav-link" active-class="active">
            ℹ️ About
          </router-link>
        </li>
      </ul>
    </div>
  </nav>
</template>

<script setup lang="ts">
// Navigation component logic
</script>

<style scoped>
.navbar {
  background: #2c3e50;
  color: white;
  padding: 0;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  position: sticky;
  top: 0;
  z-index: 100;
}

.nav-container {
  max-width: 1200px;
  margin: 0 auto;
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0 2rem;
}

.nav-brand {
  font-size: 1.25rem;
  font-weight: bold;
}

.brand-link {
  color: white;
  text-decoration: none;
  display: flex;
  align-items: center;
  padding: 1rem 0;
}

.brand-link:hover {
  color: #3498db;
}

.nav-menu {
  display: flex;
  list-style: none;
  margin: 0;
  padding: 0;
  gap: 0;
}

.nav-item {
  margin: 0;
}

.nav-link {
  display: block;
  color: white;
  text-decoration: none;
  padding: 1rem 1.5rem;
  transition: all 0.3s ease;
  border-bottom: 3px solid transparent;
}

.nav-link:hover {
  background: rgba(255, 255, 255, 0.1);
  color: #3498db;
}

.nav-link.active {
  background: rgba(255, 255, 255, 0.1);
  border-bottom-color: #3498db;
  color: #3498db;
}

/* Responsive design */
@media (max-width: 768px) {
  .nav-container {
    flex-direction: column;
    padding: 1rem;
  }
  
  .nav-menu {
    margin-top: 1rem;
    flex-wrap: wrap;
    justify-content: center;
  }
  
  .nav-link {
    padding: 0.75rem 1rem;
    font-size: 0.9rem;
  }
}
</style>


