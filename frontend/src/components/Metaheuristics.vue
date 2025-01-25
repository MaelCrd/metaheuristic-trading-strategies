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
        hover
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
            v-for="(param, index) in parseParameters(item.mh_parameters)"
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

    <!-- Dialog to create a metaheuristic object -->
    <v-dialog v-model="dialogCreate" max-width="600" opacity="0">
      <v-card>
        <v-card-title class="mt-3 ml-3"
          >Create metaheuristic object</v-card-title
        >
        <v-card-text>
          <v-form ref="form" v-model="valid">
            <v-select
              v-model="mhObject.mh_algorithm_name"
              :items="algorithms.map((algo) => algo.name)"
              label="Algorithm"
              required
              @update:model-value="updateParameters"
            />
            <v-col v-if="selectedAlgorithm">
              <h4 class="mb-4">Parameters</h4>
              <v-row
                v-for="(param, index) in selectedAlgorithm.parameters"
                :key="index"
                align="start"
              >
                <v-col cols="6">
                  <v-text-field
                    v-model="mhObject.mh_parameters[param.name]"
                    variant="outlined"
                    :label="param.name + ' (' + String(param.bounds) + ')'"
                    type="number"
                    :min="param.bounds[0]"
                    :max="param.bounds[1]"
                    :rules="[
                      (value) =>
                        (value >=
                          (param.bounds[0] ? param.bounds[0] : -Infinity) &&
                          value <=
                            (param.bounds[1] ? param.bounds[1] : +Infinity)) ||
                        `Value must be between ${param.bounds[0]} and ${param.bounds[1]}`,
                    ]"
                    required
                  />
                </v-col>
                <v-col>
                  {{ param.description }}
                </v-col>
              </v-row>
            </v-col>
            <div align="end">
              <v-btn
                color="success"
                :disabled="!valid || !selectedAlgorithm"
                @click="createMHObject"
              >
                Create
              </v-btn>
            </div>
          </v-form>
        </v-card-text>
      </v-card>
    </v-dialog>
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
      dialogCreate: false,
      valid: false,
      mhObject: {
        mh_algorithm_name: "",
        mh_parameters: {},
      },
      algorithms: [],
      selectedAlgorithm: null,
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
    this.getMHinfo().then((data) => {
      this.algorithms = data;
    });
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
    createMHObject() {
      console.log("Creating metaheuristic object:", this.mhObject);

      if (
        !this.valid ||
        !this.mhObject.mh_algorithm_name ||
        !this.selectedAlgorithm
      ) {
        console.error("Invalid form data");
        return;
      }

      const mhObject = {
        mh_algorithm_name: this.mhObject.mh_algorithm_name,
        mh_parameters: JSON.stringify(this.mhObject.mh_parameters),
      };

      axios
        .post("http://localhost:9797/api/mh_object", mhObject)
        .then(() => {
          console.log("Metaheuristic object created successfully");
          this.dialogCreate = false;
          this.selectedAlgorithm = null;
          this.mhObject = {
            mh_algorithm_name: "",
            mh_parameters: {},
          };
          this.$emit("refresh-mh-objects");
        })
        .catch((error) => {
          console.error("Error creating metaheuristic object:", error);
        });
    },
    async getMHinfo() {
      // Add your method logic here
      const response = await axios.get("http://localhost:9797/api/algorithms");
      console.log(response.data);

      return response.data;
    },
    updateParameters() {
      this.selectedAlgorithm = this.algorithms.find(
        (algo) => algo.name === this.mhObject.mh_algorithm_name
      );
      console.log("Selected algorithm:", this.selectedAlgorithm);
      // this.mhObject.mh_parameters = {};
      // if (this.selectedAlgorithm) {
      //   this.selectedAlgorithm.parameters.forEach((param) => {
      //     this.mhObject.mh_parameters[param.name] = "";
      //   });
      // }
    },
    parseParameters(parameters: string) {
      try {
        return JSON.parse(parameters);
      } catch (e) {
        console.error("Failed to parse parameters:", e);
        return [];
      }
    },
  },
};
</script>

<style scoped>
.metaheuristics {
  /* Add your component styles here */
}

/* hide the "scrim", it's pointless */
.v-overlay--active .v-overlay__scrim {
  display: none;
}

/* style the overlay container as required */
.v-overlay--active {
  backdrop-filter: blur(3px);
  background: rgb(0 0 0 / 0.2);
}
</style>
