<template>
  <v-col class="main-content">
    <v-row align="center" justify="space-between" class="mt-2 mr-8">
      <v-card-title>
        <h3>Metaheuristics objects</h3>
      </v-card-title>
      <div>
        <v-btn
          variant="plain"
          :prepend-icon="showHidden ? 'mdi-eye-off-outline' : 'mdi-eye-outline'"
          color="primary"
          @click="handleShowHidden"
        >
          {{ showHidden ? "Hide hidden" : "Show hidden" }}
        </v-btn>
        <v-btn
          class="ml-4"
          variant="outlined"
          prepend-icon="mdi-plus"
          color="primary"
          @click="dialogCreate = true"
        >
          Create
        </v-btn>
      </div>
    </v-row>
    <v-row>
      <v-data-table
        multi-sort
        :headers="headers"
        :items="filteredItems"
        class="pl-6 pr-12 pb-6"
      >
        <template v-slot:item.mh_algorithm_name="{ item }">
          <span>{{ item.mh_algorithm_name }}</span>
          <v-chip v-if="item.hidden" color="warning" small class="ml-2">
            Hidden
          </v-chip>
        </template>
        <template v-slot:item.mh_parameters="{ item }">
          <v-chip
            v-for="(param, index) in JSON.parse(item.mh_parameters)"
            :key="index"
            size="small"
            class="mr-2"
            color="success"
          >
            {{ index }} : {{ param }}
          </v-chip>
        </template>
        <template v-slot:item.actions="{ item }">
          <v-btn icon @click="hideItem(item)">
            <v-icon v-if="item.hidden">mdi-eye-outline</v-icon>
            <v-icon v-else>mdi-eye-off-outline</v-icon>
          </v-btn>
        </template>
      </v-data-table>
    </v-row>
  </v-col>
</template>

<script lang="ts">
import axios from "axios";

export default {
  name: "Metaheuristics",
  props: {
    items: Array,
  },
  emits: ["refresh-mh-objects"],
  data() {
    return {
      headers: [
        { title: "Name", value: "mh_algorithm_name", width: "25%" },
        { title: "Parameters", value: "mh_parameters", width: "65%" },
        { title: "Actions", value: "actions", width: "10%" },
      ],
      showHidden: false,
      // Add your component data here
    };
  },
  computed: {
    // Add your computed properties here
    filteredItems() {
      return this.showHidden
        ? this.items
        : this.items.filter((item: any) => !item.hidden);
    },
  },
  mounted() {
    // Add your mounted logic here
  },
  methods: {
    // Add your component methods here
    handleShowHidden() {
      this.showHidden = !this.showHidden;
      console.log("Show hidden:", this.showHidden);
    },
    hideItem(item: any) {
      console.log("Hiding item:", item);

      axios
        .put(
          `http://localhost:9797/api/mh_object?id=${
            item.id
          }&hidden=${!item.hidden}`
        )
        .then(() => {
          console.log("Item hidden successfully");
          this.$emit("refresh-mh-objects");
        })
        .catch((error) => {
          console.error("Error hiding item:", error);
        });
    },
  },
};
</script>

<style scoped>
.metaheuristics {
  /* Add your component styles here */
}
</style>
