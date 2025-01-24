<template>
  <v-grid class="main">
    <v-row class="full-height" no-gutters>
      <v-col class="left-nav full-height">
        <!-- Left navigation content here -->

        <v-list density="compact" nav class="full-height">
          <h3 class="nav-title">Navigation</h3>
          <v-list-item prepend-icon="mdi-view-dashboard" title="Home" value="home" @click="handleNavigation('home')" />
          <v-list-item prepend-icon="mdi-currency-btc" title="Crypto symbols" value="symbols"
            @click="handleNavigation('symbols')" />
          <v-list-item prepend-icon="mdi-format-list-bulleted" title="Symbols lists" value="lists"
            @click="handleNavigation('lists')" />
          <v-list-item prepend-icon="mdi-chip" title="Metaheuristics" value="mh-objects"
            @click="handleNavigation('mh-objects')" />
          <v-list-item prepend-icon="mdi-code-block-tags" title="Tasks" value="tasks"
            @click="handleNavigation('tasks')" />
        </v-list>
      </v-col>
      <v-col class="main-content">
        <!-- Main content here -->
        <!-- Main content here -->
        <v-card>
          <!-- <v-card-title style="margin-top: 10px;">
            <h3>Crypto symbols</h3>
          </v-card-title> -->
          <!-- <v-card-text>
            <v-btn @click="fetchData">Refresh</v-btn>
          </v-card-text>
          <v-divider /> -->

          <!-- <v-btn color="primary" class="ma-8" @click="fetchData">Refresh</v-btn> -->

          <CryptoSymbols v-if="selected === 'symbols'" :items="crypto_symbols" />
          <CryptoLists v-if="selected === 'lists'" :items="crypto_lists" />
          <Metaheuristics v-if="selected === 'mh-objects'" :items="mh_objects" />
          <Tasks v-if="selected === 'tasks'" :items="tasks" />
        </v-card>
      </v-col>
    </v-row>
  </v-grid>
</template>

<script setup lang="ts">
  import { ref } from 'vue'
  import axios from 'axios'
  import CryptoSymbols from './CryptoSymbols.vue';
  import CryptoLists from './CryptoLists.vue';
  import Metaheuristics from './Metaheuristics.vue';
  import Tasks from './Tasks.vue';

  let selected = ref('home');

  let crypto_symbols = ref([
    { symbol: 'BTC', volume: 1000, available: true, last_updated: '2021-10-10' },
    { symbol: 'ETH', volume: 2000, available: false, last_updated: '2021-10-11' },
    { symbol: 'LTC', volume: 3000, available: true, last_updated: '2021-10-12' },
  ]);

  let crypto_lists = ref([
    { name: 'List 1', interval: '1m', limit_count: 100, type: 'type 1' },
    { name: 'List 2', interval: '5m', limit_count: 200, type: 'type 2' },
    { name: 'List 3', interval: '15m', limit_count: 300, type: 'type 3' },
  ]);

  let mh_objects = ref([
    { mh_algorithm_name: 'Algorithm 1', mh_parameters: 'param 1' },
    { mh_algorithm_name: 'Algorithm 2', mh_parameters: 'param 2' },
    { mh_algorithm_name: 'Algorithm 3', mh_parameters: 'param 3' },
  ]);

  let tasks = ref([
    { state: 'state 1', created_at: '2021-10-10' },
    { state: 'state 2', created_at: '2021-10-11' },
    { state: 'state 3', created_at: '2021-10-12' },
  ]);

  // Refresh data
  const fetchData = async () => {
    const response = await axios.get('http://localhost:9797/api/crypto_symbol')
    crypto_symbols.value = response.data
    const response2 = await axios.get('http://localhost:9797/api/crypto_list')
    crypto_lists.value = response2.data
    const response3 = await axios.get('http://localhost:9797/api/mh_object')
    mh_objects.value = response3.data
    const response4 = await axios.get('http://localhost:9797/api/task')
    tasks.value = response4.data

    console.log('Data refreshed')
  }

  fetchData()

  const handleNavigation = (value: string) => {
    selected.value = value
  }

</script>

<style scoped>
  .main {
    width: 100%;
    height: 100%;
  }

  .main-content {
    margin-left: 15px;
    margin-right: 15px;
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