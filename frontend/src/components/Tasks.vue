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
        class="pa-6"
      >
      </v-data-table>
    </v-row>

    <!-- Dialog to create a task -->
    <v-dialog v-model="dialogCreate" max-width="600px">
      <v-card>
        <v-card-title>Create Task</v-card-title>
        <v-card-text>
          <v-form ref="form" v-model="valid">
            <v-select
              v-model="selectedCryptoList"
              :items="cryptoLists"
              item-text="name"
              item-value="id"
              label="Select Crypto List"
              required
            ></v-select>
            <v-select
              v-model="selectedMHObject"
              :items="mhObjects"
              item-text="mh_algorithm_name"
              item-value="id"
              label="Select Metaheuristic Object"
              required
            ></v-select>
            <v-text-field
              v-model="taskParameters"
              label="Task Parameters"
              required
            ></v-text-field>
          </v-form>
        </v-card-text>
        <v-card-actions>
          <v-btn color="primary" @click="createTask">Create</v-btn>
          <v-btn @click="dialogCreate = false">Cancel</v-btn>
        </v-card-actions>
      </v-card>
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
  },
  emits: ["refresh-tasks"],
  data() {
    return {
      headers: [
        { title: "Created at", value: "created_at", width: "20%" },
        { title: "State", value: "state" },
        { title: "Actions", value: "actions", sortable: false, width: "10%" },
      ],
      dialogCreate: false,
      valid: false,
      selectedCryptoList: null,
      selectedMHObject: null,
      taskParameters: "",
      // Add your component data here
    };
  },
  computed: {
    // Add your computed properties here
    filteredItems() {
      return this.showHidden
        ? this.items
        : this.items.filter((item) => !item.hidden);
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
    createTask() {
      if (this.$refs.form.validate()) {
        const taskData = {
          crypto_list_id: this.selectedCryptoList,
          mh_object_id: this.selectedMHObject,
          other_parameters: this.taskParameters,
        };
        axios
          .post("http://localhost:9797/api/task", taskData)
          .then(() => {
            this.dialogCreate = false;
            this.$emit("refresh-tasks");
          })
          .catch((error) => {
            console.error("Error creating task:", error);
          });
      }
    },
  },
};
</script>

<style scoped>
.metaheuristics {
  /* Add your component styles here */
}
</style>
