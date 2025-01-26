<template>
  <v-card>
    <v-card-title class="mt-3 ml-3">Task {{ task.id }} details</v-card-title>
    <!-- <p>{{ task }}</p> -->
    <v-card-text>
      <v-timeline side="end" size="small" truncate-line="both">
        <v-timeline-item small>
          <div>
            <div class="text-h6">Task created</div>
            <p>
              At {{ task.created_at }} with parameters:
              {{ task.other_parameters }}
            </p>
          </div>
        </v-timeline-item>
        <v-timeline-item small>
          <div>
            <div class="text-h6">Fetch cryptos</div>
            <p>
              {{ getCryptoList(task.crypto_list_id) }}
            </p>
          </div>
        </v-timeline-item>
        <v-timeline-item small>
          <div>
            <div class="text-h6">Used MH</div>
            <p>
              {{ getMHObject(task.mh_object_id) }} with indicators:
              {{ getIndicatorCombination(task.indicator_combination_id) }}
            </p>
          </div>
        </v-timeline-item>
        <v-timeline-item small>
          <div>
            <div class="text-h6">Task {{ task.state }}</div>
            <p>Duration: XXX seconds</p>
          </div>
        </v-timeline-item>
      </v-timeline>
    </v-card-text>
  </v-card>
</template>

<script lang="ts">
export default {
  name: "TaskDetails",
  props: {
    task: Object,
    cryptoLists: Array,
    mhObjects: Array,
    indicators: Array,
  },
  data() {
    return {
      // Add your component data here
    };
  },
  computed: {
    // Add your computed properties here
  },
  mounted() {
    // Add your mounted logic here
  },
  methods: {
    // Add your component methods here
    getCryptoList(id: number) {
      return this.cryptoLists.find((item) => item.id === id);
    },
    getMHObject(id: number) {
      return this.mhObjects.find((item) => item.id === id);
    },
    getIndicatorCombination(id: number) {
      return this.indicators.find((item) => item.id === id);
    },
  },
};
</script>

<style scoped></style>
