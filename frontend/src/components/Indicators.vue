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
        <template v-slot:item.indicators_struct_names="{ item }">
          <!-- <span>{{ item.indicators_struct_names.join(", ") }}</span> -->
          <v-chip
            v-for="(indicator, index) in item.indicators_struct_names"
            :key="index"
            color="success"
            class="mr-2 mb-1 mt-2"
          >
            {{ indicator }}
          </v-chip>
        </template>
      </v-data-table>
    </v-row>

    <!-- Dialog to create an indicator combination -->
    <v-dialog v-model="dialogCreate" max-width="900px" opacity="0">
      <v-card>
        <v-card-title class="mt-3 ml-3"
          >Create indicator combination</v-card-title
        >
        <v-card-text>
          <v-form ref="form" v-model="valid">
            <v-col>
              <v-row>
                <v-col>
                  <v-text-field
                    max-width="500"
                    variant="outlined"
                    v-model="indicatorCombination.name"
                    label="Combination name"
                    required
                  />
                </v-col>
              </v-row>
              <v-divider class="mb-5" />
              <v-row
                v-for="(indicator, index) in indicatorCombination.indicators"
                :key="index"
              >
                <v-col>
                  <v-row no-gutters>
                    <v-col>
                      <v-select
                        :model-value="
                          indicatorCombination.indicators[index].name
                        "
                        :items="indicators.map((indicator) => indicator.name)"
                        label="Indicator"
                        required
                        @update:model-value="updateIndicator(index, $event)"
                      />
                    </v-col>
                    <v-col>
                      <v-btn
                        class="ma-1"
                        color="error"
                        variant="text"
                        icon="mdi-delete-outline"
                        @click="removeIndicator(index)"
                      />
                    </v-col>
                  </v-row>

                  <div>
                    <v-row v-if="indicatorCombination.indicators[index]">
                      <v-col
                        v-for="(param, key) in indicatorCombination
                          .possibleParameters[index]"
                        :key="key"
                        align="start"
                      >
                        <v-text-field
                          v-model="
                            indicatorCombination.possibleParameters[index][key][
                              'value'
                            ]
                          "
                          max-width="420"
                          variant="outlined"
                          type="number"
                          :label="
                            param.name +
                            ' (' +
                            param.type +
                            ') : ' +
                            param.description
                          "
                          required
                        />
                      </v-col>
                    </v-row>
                  </div>
                </v-col>

                <v-divider />
              </v-row>
              <v-row align="start">
                <v-btn
                  color="primary"
                  prepend-icon="mdi-plus"
                  class="mt-4"
                  @click="addIndicator"
                >
                  Add indicator
                </v-btn>
              </v-row>
            </v-col>
            <div align="end">
              <v-btn
                color="success"
                :disabled="
                  !valid ||
                  !indicatorCombination.name ||
                  indicatorCombination.indicators.length === 0 ||
                  indicatorCombination.possibleParameters.filter(
                    (param) => Object.keys(param).length === 0
                  ).length > 0
                "
                @click="createIndicatorCombination"
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
        possibleParameters: [],
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
      this.indicatorCombination.possibleParameters.push({});
    },
    removeIndicator(index: number) {
      console.log("Removing indicator at index:", index);
      this.indicatorCombination.indicators.splice(index, 1);
      this.indicatorCombination.possibleParameters.splice(index, 1);
    },
    updateIndicator(index: number, value: string) {
      console.log("Updating indicator at index:", index, "with value:", value);
      this.indicatorCombination.indicators[index] = { name: value };
      const indicator = this.indicators.find((i) => i.name === value);
      this.indicatorCombination.possibleParameters[index] = {};
      indicator.parameters.forEach((param) => {
        (this.indicatorCombination.possibleParameters[index][param.name] =
          param),
          (this.indicatorCombination.possibleParameters[index][param.name][
            "value"
          ] = String(param.default));
      });
    },
    createIndicatorCombination() {
      console.log("Creating indicator combination:", this.indicatorCombination);

      if (!this.valid || !this.indicatorCombination.name) {
        console.error("Invalid form data");
        return;
      }

      const combination = {
        name: this.indicatorCombination.name,
        indicators: this.indicatorCombination.indicators.map(
          (indicatorInList, index) => {
            const indicator = this.indicators.find(
              (i) => i.name === indicatorInList.name
            );
            const parameters = {};
            indicator.parameters.forEach((param) => {
              parameters[param.name] = Number(
                this.indicatorCombination.possibleParameters[index][param.name][
                  "value"
                ]
              );
            });
            return JSON.stringify({
              indicator_struct_name: indicator.struct_name,
              parameters: JSON.stringify(parameters).replace(/"/g, "'"),
            });
          }
        ),
      };

      console.log("Creating indicator combination:", combination);

      axios
        .post("http://localhost:9797/api/indicator_combinations", combination)
        .then(() => {
          console.log("Indicator combination created successfully");
          this.dialogCreate = false;
          this.indicatorCombination = {
            name: "",
            indicators: [],
            possibleParameters: [],
          };
          this.$emit("refresh-indicators");
        })
        .catch((error) => {
          console.error("Error creating indicator combination:", error);
        });
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
