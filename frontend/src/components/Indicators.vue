<template>
  <v-col class="main-content">
    <v-row align="center" justify="space-between" class="mt-2 mr-8">
      <v-card-title>
        <h3>Indicators combinations</h3>
      </v-card-title>
      <div>
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
        :items="items"
        class="pl-6 pr-12 pb-6"
      >
      </v-data-table>
    </v-row>

    <!-- Dialog to create an indicator combination -->
    <v-dialog v-model="dialogCreate" max-width="600" opacity="0">
      <v-card>
        <v-card-title>Create indicator combination</v-card-title>
        <v-card-text>
          <v-form ref="form" v-model="valid">
            <!-- <v-select
              v-model="indicatorCombination.name"
              :items="indicators.map((indicator) => indicator.name)"
              label="Indicator"
              required
              @update:model-value="updateParameters"
            /> -->

            <p>{{ indicatorCombination }}</p>

            <v-col no-gutters>
              <v-row
                v-for="(indicator, index) in indicatorCombination.indicators"
                :key="index"
                align="center"
                justify="space-between"
              >
                <v-col>
                  <v-select
                    :model-value="indicatorCombination.indicators[index].name"
                    :items="indicators.map((indicator) => indicator.name)"
                    label="Indicators"
                    required
                    @update:model-value="updateIndicator(index, $event)"
                  />
                </v-col>
                <v-col>
                  <v-btn
                    color="error"
                    icon="mdi-delete-outline"
                    @click="removeIndicator(index)"
                  ></v-btn>
                </v-col>

                <v-divider />
              </v-row>
              <v-row align="start">
                <v-btn color="primary" @click="addIndicator" class="mt-4"
                  >Add indicator</v-btn
                >
              </v-row>
            </v-col>
            <div align="end">
              <v-btn
                color="primary"
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
  name: "Indicators",
  props: {
    items: Array,
  },
  emits: ["refresh-indicators"],
  data() {
    return {
      // Add your component data here
      headers: [
        { title: "Name", value: "name", width: "25%" },
        { title: "Indicators", value: "indicators_struct_names", width: "65%" },
        { title: "Actions", value: "actions", width: "10%" },
      ],
      dialogCreate: false,
      valid: false,
      indicatorCombination: {
        name: "",
        indicators: [],
      },
      indicators: [],
    };
  },
  computed: {
    // Add your computed properties here
  },
  mounted() {
    // Add your mounted logic here
    this.getIndicatorInfo().then((data) => {
      this.indicators = data;
    });
  },
  methods: {
    // Add your component methods here
    addIndicator() {
      this.indicatorCombination.indicators.push({});
    },
    removeIndicator(index: number) {
      console.log("Removing indicator at index:", index);

      for (let i = 0; i < this.indicatorCombination.indicators.length; i++) {
        console.log(
          "Indicator at index",
          i,
          ":",
          this.indicatorCombination.indicators[i]
        );
      }

      this.indicatorCombination.indicators.splice(index, 1);

      for (let i = 0; i < this.indicatorCombination.indicators.length; i++) {
        console.log(
          "Indicator at index",
          i,
          ":",
          this.indicatorCombination.indicators[i]
        );
      }
    },
    updateIndicator(index: number, value: string) {
      console.log("Updating indicator at index:", index, "with value:", value);
      this.indicatorCombination.indicators[index]["name"] = value;
    },
    createIndicatorCombination() {
      console.log("Creating indicator combination:", this.indicatorCombination);

      if (!this.valid || !this.indicatorCombination.name) {
        console.error("Invalid form data");
        return;
      }

      // const mhObject = {
      //   mh_algorithm_name: this.mhObject.mh_algorithm_name,
      //   mh_parameters: JSON.stringify(this.mhObject.mh_parameters),
      // };

      // axios
      //   .post("http://localhost:9797/api/mh_object", mhObject)
      //   .then(() => {
      //     console.log("Metaheuristic object created successfully");
      //     this.dialogCreate = false;
      //     this.selectedAlgorithm = null;
      //     this.mhObject = {
      //       mh_algorithm_name: "",
      //       mh_parameters: {},
      //     };
      //     this.$emit("refresh-mh-objects");
      //   })
      //   .catch((error) => {
      //     console.error("Error creating metaheuristic object:", error);
      //   });
    },
    async getIndicatorInfo() {
      // Add your method logic here
      const response = await axios.get("http://localhost:9797/api/indicators");
      console.log(response.data);

      return response.data;
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
