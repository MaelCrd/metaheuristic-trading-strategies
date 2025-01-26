<template>
  <v-col class="main-content">
    <v-row align="center" justify="space-between" class="mt-2 mr-8">
      <v-card-title>
        <h3>Metaheuristics objects</h3>
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
        :items="filteredItems"
        density="comfortable"
        class="pl-6 pr-12 pb-6"
        hover
        items-per-page="-1"
      >
        <!-- Chip for state -->
        <template v-slot:item.state="{ item }">
          <v-chip
            :color="getStateColor(item.state)"
            :text="item.state"
            class="text-uppercase"
            size="small"
            label
          />
        </template>
        <!-- Cancel icon button -->
        <template v-slot:item.actions="{ item }">
          <div class="d-flex justify-end">
            <v-icon
              v-if="
                item.state.toLowerCase() !== 'completed' &&
                item.state.toLowerCase() !== 'cancelled' &&
                item.state.toLowerCase() !== 'cancelling' &&
                item.state.toLowerCase() !== 'failed'
              "
              @click="cancelTask(item.id)"
              class="ml-4"
              size="24"
              >mdi-close</v-icon
            >
            <v-icon @click="showTaskDetails(item.id)" class="ml-4" size="24"
              >mdi-information-outline</v-icon
            >
          </div>
        </template>
      </v-data-table>
    </v-row>

    <!-- Dialog to create a task -->
    <v-dialog v-model="dialogCreate" max-width="600px" opacity="0">
      <v-card>
        <v-card-title class="mt-3 ml-3">Create Task</v-card-title>
        <v-card-text>
          <v-form ref="form" v-model="valid">
            <v-select
              v-model="selectedCryptoList"
              label="Select Crypto List"
              :items="styledCryptoLists"
              item-value="id"
              required
            />
            <v-select
              v-model="selectedMHObject"
              label="Select Metaheuristic Object"
              :items="styledMHObjects"
              item-value="id"
              required
            />
            <v-select
              v-model="selectedIndicatorCombination"
              label="Select Indicator Combination"
              :items="styledIndicators"
              item-value="id"
              required
            />
            <h4 class="mb-2">Task parameters</h4>
            <v-checkbox
              v-model="taskParameters.queue"
              label="Queue task"
              density="comfortable"
              hide-details
            />
            <v-checkbox
              v-model="taskParameters.force_fetch"
              label="Force fetch"
              density="comfortable"
              hide-details
            />
            <h4 class="mt-4 mb-2">Training percentage</h4>
            <v-slider
              v-model="taskParameters.training_percentage"
              min="0"
              max="100"
              step="1"
            >
              <template v-slot:append>
                <v-text-field
                  v-model="taskParameters.training_percentage"
                  density="compact"
                  style="width: 105px"
                  type="number"
                  hide-details
                  single-line
                  append-inner-icon="mdi-percent"
                />
              </template>
            </v-slider>
          </v-form>
        </v-card-text>
        <v-card-actions>
          <v-btn color="primary" @click="createTask">Create</v-btn>
          <v-btn @click="dialogCreate = false">Cancel</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <!-- Dialog to show task details -->
    <v-dialog v-model="selectedTask" max-width="900px" opacity="0">
      <TaskDetails
        :task="selectedTask"
        :crypto-lists="cryptoLists"
        :mh-objects="mhObjects"
        :indicators="indicators"
      />
    </v-dialog>
  </v-col>
</template>

<script lang="ts">
import axios from "axios";

export default {
  name: "Tasks",
  props: {
    items: Array,
    cryptoLists: Array,
    mhObjects: Array,
    indicators: Array,
  },
  emits: ["refresh-tasks"],
  data() {
    return {
      // Add your component data here
      headers: [
        { title: "Created at", value: "created_at", width: "20%" },
        { title: "State", value: "state" },
        {
          title: "Actions",
          value: "actions",
          sortable: false,
          width: "20%",
          align: "end",
        },
      ],
      dialogCreate: false,
      valid: false,
      selectedCryptoList: null,
      selectedMHObject: null,
      selectedIndicatorCombination: null,
      taskParameters: {
        queue: true,
        force_fetch: false,
        training_percentage: 85,
      },
      selectedTask: null,
    };
  },
  computed: {
    // Add your computed properties here
    filteredItems() {
      return this.showHidden
        ? this.items
        : this.items.filter((item) => !item.hidden);
    },
    styledCryptoLists() {
      return this.cryptoLists
        ?.filter((item) => !item.hidden)
        .map((item) => ({
          title: item.interval + " - " + item.name,
          id: item.id,
          props: {
            subtitle: item.type + " - " + item.limit_count,
          },
        }));
    },
    styledMHObjects() {
      return this.mhObjects
        ?.filter((item) => !item.hidden)
        .map((item) => ({
          title: item.mh_algorithm_name,
          id: item.id,
          props: {
            subtitle: item.mh_parameters,
          },
        }));
    },
    styledIndicators() {
      return this.indicators
        ?.filter((item) => !item.hidden)
        .map((item) => ({
          title: item.name,
          id: item.id,
          props: {
            subtitle: (item.indicators_struct_names || []).join(", "),
          },
        }));
    },
  },
  mounted() {
    // Add your mounted logic here
  },
  methods: {
    // Add your component methods here
    // handleShowHidden() {
    //   this.showHidden = !this.showHidden;
    // },
    showTaskDetails(taskId: any) {
      console.log("Showing task details for task ID:", taskId, this.items);
      this.selectedTask = this.items.find((item) => item.id === taskId) || null;
      console.log("Selected task:", this.selectedTask);
    },
    cancelTask(taskId: number) {
      axios
        .put(`http://localhost:9797/api/task/cancel?id=${taskId}`)
        .then(() => {
          this.$emit("refresh-tasks");
        })
        .catch((error) => {
          console.error("Error canceling task:", error);
        });
    },
    createTask() {
      if (this.$refs.form.validate()) {
        // ex: {"mh_object_id": 4, "crypto_list_id": 2, "other_parameters": "{\"force_fetch\": false, \"training_percentage\": 0.85}", "indicator_combination_id": 1}
        const parameters = {
          force_fetch: this.taskParameters.force_fetch,
          training_percentage: this.taskParameters.training_percentage / 100,
        };
        const taskData = {
          mh_object_id: this.selectedMHObject,
          crypto_list_id: this.selectedCryptoList,
          indicator_combination_id: this.selectedIndicatorCombination,
          other_parameters: JSON.stringify(parameters),
        };
        axios
          .post(
            `http://localhost:9797/api/task?queue=${this.taskParameters.queue}`,
            taskData
          )
          .then(() => {
            this.dialogCreate = false;
            this.$refs.form.reset();
            this.taskParameters = {
              queue: true,
              force_fetch: false,
              training_percentage: 85,
            };
            this.selectedCryptoList = null;
            this.selectedMHObject = null;
            this.selectedIndicatorCombination = null;
            this.$emit("refresh-tasks");
          })
          .catch((error) => {
            console.error("Error creating task:", error);
          });
      }
    },
    getStateColor(state: string) {
      switch (state.toLowerCase()) {
        case "created":
          return "info";
        case "pending":
          return "info";
        case "running":
          return "primary";
        case "completed":
          return "success";
        case "failed":
          return "error";
        case "cancelled":
          return "warning";
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
