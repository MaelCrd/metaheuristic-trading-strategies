<template>
  <v-grid class="main">
    <v-row class="full-height" no-gutters>
      <v-col class="left-nav full-height">
        <!-- Left navigation content here -->

        <v-list density="compact" nav class="full-height pa-3">
          <h3 class="nav-title">Navigation</h3>
          <v-list-item
            prepend-icon="mdi-view-dashboard"
            title="Home"
            value="home"
            @click="handleNavigation('home')"
          />
          <v-list-item
            prepend-icon="mdi-currency-btc"
            title="Crypto symbols"
            value="symbols"
            @click="handleNavigation('symbols')"
          />
          <v-list-item
            prepend-icon="mdi-format-list-bulleted"
            title="Symbols lists"
            value="lists"
            @click="handleNavigation('lists')"
          />
          <v-list-item
            prepend-icon="mdi-chip"
            title="Metaheuristics"
            value="mh-objects"
            @click="handleNavigation('mh-objects')"
          />
          <v-list-item
            prepend-icon="mdi-vector-polyline"
            title="Indicators"
            value="indicators"
            @click="handleNavigation('indicators')"
          />
          <v-list-item
            prepend-icon="mdi-code-block-tags"
            title="Tasks"
            value="tasks"
            @click="handleNavigation('tasks')"
          />
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

          <CryptoSymbols
            v-if="selected === 'symbols'"
            :items="crypto_symbols"
            @refresh-symbols="handleRefreshSymbols"
          />
          <CryptoLists
            v-if="selected === 'lists'"
            :items="crypto_lists"
            :crypto_symbols="crypto_symbols"
            @refresh-lists="handleRefreshLists"
          />
          <Metaheuristics
            v-if="selected === 'mh-objects'"
            :items="mh_objects"
            @refresh-mh-objects="handleRefreshMHObjects"
          />
          <Indicators
            v-if="selected === 'indicators'"
            :items="indicators"
            @refresh-indicators="handleRefreshIndicators"
          />
          <Tasks
            v-if="selected === 'tasks'"
            :items="tasks"
            :crypto-lists="crypto_lists"
            :mh-objects="mh_objects"
            :indicators="indicators"
            @refresh-tasks="handleRefreshTasks"
          />
        </v-card>
        <!-- <p>{{ streamData }}</p> -->
      </v-col>
    </v-row>
  </v-grid>
</template>

<script setup lang="ts">
import { ref } from "vue";
import axios from "axios";
import CryptoSymbols from "./CryptoSymbols.vue";
import CryptoLists from "./CryptoLists.vue";
import Metaheuristics from "./Metaheuristics.vue";
import Tasks from "./Tasks.vue";
import moment from "moment";

let selected = ref("home");

let crypto_symbols = ref([
  { symbol: "BTC", volume: 1000, available: true, last_updated: "2021-10-10" },
  { symbol: "ETH", volume: 2000, available: false, last_updated: "2021-10-11" },
  { symbol: "LTC", volume: 3000, available: true, last_updated: "2021-10-12" },
]);

let crypto_lists = ref([
  { name: "List 1", interval: "1m", limit_count: 100, type: "type 1" },
  { name: "List 2", interval: "5m", limit_count: 200, type: "type 2" },
  { name: "List 3", interval: "15m", limit_count: 300, type: "type 3" },
]);

let mh_objects = ref([
  { mh_algorithm_name: "Algorithm 1", mh_parameters: '{"param 1": 2}' },
  { mh_algorithm_name: "Algorithm 2", mh_parameters: '{"param 1": 2}' },
  { mh_algorithm_name: "Algorithm 3", mh_parameters: '{"param 1": 2}' },
]);

let indicators = ref([
  { name: "Indicator 1", indicators_struct_names: ["struct 1"] },
  { name: "Indicator 2", indicators_struct_names: ["struct 2"] },
  { name: "Indicator 3", indicators_struct_names: ["struct 3"] },
]);

let tasks = ref([
  { state: "state 1", created_at: "2021-10-10" },
  { state: "state 2", created_at: "2021-10-11" },
  { state: "state 3", created_at: "2021-10-12" },
]);

const handleNavigation = (value: string) => {
  selected.value = value;
};

const fetchSymbols = async () => {
  console.log("Fetching symbols");
  const response = await axios.get("http://localhost:9797/api/crypto_symbol");
  // Set volume to integer
  response.data.forEach((item: any) => {
    item.volume = parseInt(item.volume);
  });

  // Set 'last_updated' to human readable format
  response.data.forEach((item: any) => {
    item.last_updated = moment(item.last_updated).format("YYYY-MM-DD HH:mm:ss");
  });

  // // Add 'selected' field
  // response.data.forEach((item: any) => {
  //   item.selected = false
  // })

  // Sort by id
  response.data.sort((a: any, b: any) => a.id - b.id);

  crypto_symbols.value = response.data;
};

const fetchCryptoLists = async () => {
  console.log("Fetching crypto lists");
  const response = await axios.get("http://localhost:9797/api/crypto_list");
  // Replace 'interval'
  response.data.forEach((item: any) => {
    item.interval = item.interval.replace("Int", "");
  });
  // Replace 'limit_count'
  response.data.forEach((item: any) => {
    // Limit is in minutes, convert to bigger units :
    // ex: '13' = 13 minutes,
    // ex: '68' = 1h 8m
    // ex: '31680' = 22d
    const limit = parseInt(item.limit_count);
    const duration = moment.duration(limit, "minutes");
    if (limit < 60) {
      item.limit_count = `${duration.minutes()}m`;
    } else if (limit < 1440) {
      item.limit_count =
        `${duration.hours()}h` +
        (duration.minutes() > 0 ? ` ${duration.minutes()}m` : "");
    } else if (limit < 10080) {
      item.limit_count =
        `${duration.days()}d` +
        (duration.hours() > 0 ? ` ${duration.hours()}h` : "");
    } else if (limit < 43200) {
      item.limit_count =
        `${Math.floor(duration.asWeeks())}w` +
        (duration.days() > 0 ? ` ${duration.days()}d` : "");
    } else if (limit < 525600) {
      item.limit_count =
        `${Math.floor(duration.asMonths())}M` +
        (duration.days() > 0 ? ` ${duration.days()}d` : "");
    } else {
      item.limit_count =
        `${Math.floor(duration.asYears())}y` +
        (duration.days() > 0 ? ` ${duration.days()}d` : "");
    }
  });

  // Sort by id
  response.data.sort((a: any, b: any) => a.id - b.id);

  crypto_lists.value = response.data;
};

const fetchMetaheuristics = async () => {
  console.log("Fetching metaheuristics");
  const response = await axios.get("http://localhost:9797/api/mh_object");

  // Sort by id
  response.data.sort((a: any, b: any) => a.id - b.id);

  mh_objects.value = response.data;
};

const fetchIndicators = async () => {
  console.log("Fetching indicators");
  const response = await axios.get(
    "http://localhost:9797/api/indicator_combinations"
  );

  // Sort by id
  response.data.sort((a: any, b: any) => a.id - b.id);

  indicators.value = response.data;
};

const fetchTasks = async () => {
  console.log("Fetching tasks");
  const response = await axios.get("http://localhost:9797/api/task");

  // Set 'created_at' to human readable format
  response.data.forEach((item: any) => {
    item.created_at = moment(item.created_at).format("YYYY-MM-DD HH:mm:ss");
  });

  // Sort by id
  response.data.sort((a: any, b: any) => a.id - b.id);

  tasks.value = response.data;
};

const handleRefreshSymbols = () => {
  fetchSymbols();
};

const handleRefreshLists = () => {
  fetchCryptoLists();
};

const handleRefreshMHObjects = () => {
  fetchMetaheuristics();
};

const handleRefreshIndicators = () => {
  fetchIndicators();
};

const handleRefreshTasks = () => {
  fetchTasks();
};

// Refresh data
const fetchData = async () => {
  fetchSymbols();
  fetchCryptoLists();
  fetchMetaheuristics();
  fetchIndicators();
  fetchTasks();

  console.log("Data refreshed");
};

fetchData();

handleNavigation("tasks");

const streamData = ref<string[]>([]);
const isLoading = ref(true);
const error = ref<Error | null>(null);

const fetchStream = async () => {
  console.log("Fetching stream data");
  try {
    const response = await fetch("http://localhost:9797/api/task-updates");

    if (!response.body) {
      throw new Error("No response body");
    }

    const reader = response.body.getReader();
    const decoder = new TextDecoder();

    while (true) {
      const { done, value } = await reader.read();

      if (done) break;

      const chunk = decoder.decode(value);
      streamData.value.push(chunk);

      // Receive data ex: {"task_id": 1, "state": "running"}
      console.log(chunk);
      try {
        const data = JSON.parse(chunk);
        console.log(data);

        // Find the task in the tasks list
        const task = tasks.value.find((task) => task.id === data.task_id);
        if (task) {
          task.state = data.state;

          // Update the task in the tasks list
          tasks.value = [...tasks.value];
        }
      } catch (e) {
        console.error("Failed to parse JSON:", e);
      }
    }

    isLoading.value = false;
  } catch (err) {
    error.value = err instanceof Error ? err : new Error("Unknown error");
    isLoading.value = false;
  }
};

fetchStream();
</script>

<style scoped>
.main {
  width: 100%;
  height: 100%;
}

.main-content {
  margin-left: 15px;
  margin-right: 15px;
  max-width: 1400px;
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
