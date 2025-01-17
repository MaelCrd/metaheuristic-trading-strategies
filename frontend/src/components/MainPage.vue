<template>
  <v-grid class="main">
    <v-row class="full-height" no-gutters>
      <v-col class="left-nav full-height">
        <!-- Left navigation content here -->

        <v-list density="compact" nav class="full-height">
          <h3 class="nav-title">Navigation</h3>
          <v-list-item prepend-icon="mdi-view-dashboard" title="Home" value="home" />
          <v-list-item prepend-icon="mdi-currency-btc" title="Crypto symbols" value="symbols" />
          <v-list-item prepend-icon="mdi-format-list-bulleted" title="Symbols lists" value="lists" />
          <v-list-item prepend-icon="mdi-chip" title="Metaheuristics" value="mh-objects" />
          <v-list-item prepend-icon="mdi-code-block-tags" title="Tasks" value="tasks" />
        </v-list>
      </v-col>
      <v-col class="main-content">
        <!-- Main content here -->
        <!-- Main content here -->
      </v-col>
    </v-row>
  </v-grid>
</template>

<script setup lang="ts">
  // import { ref, reactive } from 'vue'
  // import axios from 'axios'

</script>

<style scoped>
  .main {
    width: 100%;
    height: 100%;
  }

  .left-nav {
    max-width: 300px;
    height: 100%;
  }

  .nav-title {
    font-size: 1.5em;
    margin-top: 7px;
    margin-left: 8px;
    margin-bottom: 10px;
  }

  .full-height {
    height: 100%;
  }

  pre {
    white-space: pre-wrap;
    word-wrap: break-word;
  }
</style>