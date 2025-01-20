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
        <v-card>
          <v-card-title style="margin-top: 10px;">
            <h3>Crypto symbols</h3>
          </v-card-title>
          <v-card-text>
            <v-btn @click="fetchData">Refresh</v-btn>
          </v-card-text>
          <v-divider />

          <v-data-table multi-sort :headers="headers" :items="items" class="pa-6">
            <template v-slot:item.available="{ item }">
              <v-chip :color="item.available ? 'green' : 'red'" :text="item.available ? 'Available' : 'Unavailable'"
                class="text-uppercase" size="small" label>
              </v-chip>
            </template>
          </v-data-table>
        </v-card>
      </v-col>
    </v-row>
  </v-grid>
</template>

<script setup lang="ts">
  // import { ref, reactive } from 'vue'
  import axios from 'axios'

  const headers = ref([
    { title: 'Symbol', value: 'symbol' },
    { title: 'Volume', value: 'volume' },
    { title: 'Available', value: 'available' },
    { title: 'Last updated', value: 'last_updated' },
  ]);

  let items = ref([
    { name: 'John', age: 30, city: 'New York' },
    { name: 'Jane', age: 25, city: 'Los Angeles' },
    { name: 'Doe', age: 40, city: 'Chicago' },
  ]);

  // Refresh data
  const fetchData = async () => {
    const response = await axios.get('http://localhost:9797/api/crypto_symbol')
    items.value = response.data
  }

  fetchData()

</script>

<style scoped>
  .main {
    width: 100%;
    height: 100%;
  }

  .main-content {
    margin-left: 15px;
    margin-right: 150px;
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