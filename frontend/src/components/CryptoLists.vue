<template>
  <v-col class="main-content">
    <v-row align="center" justify="space-between" class="mt-2 mr-8">
      <v-card-title>
        <h3>Crypto lists</h3>
      </v-card-title>
      <div>
        <v-btn
          variant="plain"
          :prepend-icon="showHiddenIcon"
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
        <template v-slot:item.name="{ item }">
          <span>{{ item.name }}</span>
          <v-chip v-if="item.hidden" color="warning" small class="ml-2">
            Hidden
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

    <!--  -->
    <v-dialog v-model="dialogCreate" max-width="500px" opacity="0">
      <v-card>
        <v-card-title class="mt-3 ml-3">Create crypto list</v-card-title>
        <v-card-text>
          <v-form ref="form" v-model="valid">
            <v-text-field
              variant="outlined"
              v-model="formData.name"
              label="List name"
              :rules="[(v) => !!v || 'Name is required']"
              required
            />
            <h4 class="mb-1">Interval</h4>
            <v-btn-toggle
              v-model="intervalSelected"
              color="accent"
              rounded="0"
              group
              class="multi-line-btn-toggle mb-8"
            >
              <v-btn
                v-for="interval in intervals"
                :key="interval"
                :value="interval"
                class="text-capitalize"
              >
                {{ interval }}
              </v-btn>
            </v-btn-toggle>
            <v-row>
              <v-col cols="6">
                <v-text-field
                  v-model="formData.durationValue"
                  variant="outlined"
                  label="Duration"
                  type="number"
                  :rules="[(v) => !!v || 'Duration is required']"
                  required
                />
              </v-col>
              <v-col cols="6">
                <v-select
                  v-model="formData.durationUnit"
                  variant="outlined"
                  :items="durations"
                  label="Unit"
                  :rules="[(v) => !!v || 'Unit is required']"
                  required
                />
              </v-col>
            </v-row>
            <v-select
              v-model="formData.type"
              variant="outlined"
              :items="types"
              label="Type"
              :rules="[(v) => !!v || 'Type is required']"
              required
            />
            <v-btn
              class="mb-6"
              color="secondary"
              prepend-icon="mdi-selection-search"
              @click="dialogSelectSymbols = true"
            >
              Change symbols ({{ selectedSymbols.length }})
            </v-btn>
            <div />
            <div align="end">
              <v-btn
                color="success"
                :loading="loadingCreate"
                :disabled="
                  !valid || selectedSymbols.length === 0 || !intervalSelected
                "
                type="submit"
                @click="createList"
              >
                Create
              </v-btn>
            </div>
          </v-form>
        </v-card-text>
      </v-card>
    </v-dialog>

    <!-- Dialog to select symbols -->
    <v-dialog v-model="dialogSelectSymbols" max-width="700px" opacity="0">
      <v-card>
        <v-card-title class="mt-3 ml-3">Select symbols</v-card-title>
        <v-card-text>
          <v-row align="center" justify="space-between">
            <v-col cols="12">
              <v-text-field
                v-model="search"
                label="Search"
                prepend-inner-icon="mdi-magnify"
                variant="outlined"
                hide-details
                single-line
              />
            </v-col>
          </v-row>
          <v-row>
            <v-col cols="12">
              <v-data-table
                v-model="selectedSymbols"
                :headers="headersSymbols"
                :items="crypto_symbols"
                density="compact"
                class="pa-2"
                :sort-by="[
                  { key: 'available', order: 'desc' },
                  { key: 'volume', order: 'desc' },
                ]"
                :search="search"
                show-select
                multi-sort
              >
                <template v-slot:item.available="{ item }">
                  <v-icon :color="item.available ? 'green' : 'red'">
                    {{ item.available ? "mdi-check" : "mdi-close" }}
                  </v-icon>
                </template>
              </v-data-table>
            </v-col>
          </v-row>
          <v-row align="center" justify="space-between">
            <v-col cols="4">
              <v-btn
                variant="outlined"
                color="error"
                prepend-icon="mdi-close"
                @click="selectedSymbols = []"
              >
                Clear selection
              </v-btn>
            </v-col>
            <v-col cols="auto">
              <v-btn color="success" @click="dialogSelectSymbols = false">
                Done
              </v-btn>
            </v-col>
          </v-row>
        </v-card-text>
      </v-card>
    </v-dialog>
  </v-col>
</template>

<script lang="ts">
import axios from "axios";

export default {
  name: "CryptoLists",
  props: {
    items: Array,
    crypto_symbols: Array,
  },
  emits: ["refresh-lists"],
  data() {
    return {
      // Add your component data here
      headers: [
        { title: "Name", value: "name", width: "30%" },
        { title: "Interval", value: "interval", width: "20%" },
        { title: "Limit", value: "limit_count", width: "20%" },
        { title: "Type", value: "type", width: "20%" },
        { title: "Actions", value: "actions", sortable: false, width: "10%" },
      ],
      showHidden: false,
      showHiddenIcon: "mdi-eye-off-outline",
      dialogCreate: false,
      dialogSelectSymbols: false,
      headersSymbols: [
        { title: "Symbol", value: "symbol", sortable: true },
        { title: "Volume", value: "volume", sortable: true },
        { title: "Available", value: "available", sortable: true },
      ],
      selectedSymbols: [],
      valid: false,
      search: "",
      intervalSelected: null,
      loadingCreate: false,
      formData: {
        name: "",
        interval: null,
        durationValue: null,
        durationUnit: null,
        type: null,
      },
      intervals: [
        "1m",
        "5m",
        "15m",
        "30m",
        "1h",
        "2h",
        "4h",
        "6h",
        "8h",
        "12h",
        "1d",
        "3d",
        "1w",
        "1M",
      ],
      durations: ["Minutes", "Hours", "Days", "Weeks", "Months", "Years"],
      types: ["Type X", "Type Y", "Type Z"],
    };
  },
  computed: {
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
    handleShowHidden() {
      console.log("Show hidden");
      this.showHidden = !this.showHidden;
      this.showHiddenIcon = this.showHidden
        ? "mdi-eye-outline"
        : "mdi-eye-off-outline";
    },
    hideItem(item) {
      console.log("Hide item");

      axios
        .put(
          `http://localhost:9797/api/crypto_list?id=${
            item.id
          }&hidden=${!item.hidden}`
        )
        .then(() => {
          console.log("Item hidden");
          this.$emit("refresh-lists");
        })
        .catch((error) => {
          console.error(error);
        });
    },
    createList() {
      console.log("Creating list");
      this.loadingCreate = true;
      console.log(this.formData);
      console.log(this.selectedSymbols);

      let limit_minutes = this.formData.durationValue;
      switch (this.formData.durationUnit) {
        case "Hours":
          limit_minutes *= 60;
          break;
        case "Days":
          limit_minutes *= 60 * 24;
          break;
        case "Weeks":
          limit_minutes *= 60 * 24 * 7;
          break;
        case "Months":
          limit_minutes *= 60 * 24 * 30;
          break;
        case "Years":
          limit_minutes *= 60 * 24 * 365;
          break;
      }

      const properData = {
        name: this.formData.name,
        interval: "Int" + this.intervalSelected,
        limit_count: limit_minutes,
        type: this.formData.type,
        crypto_symbols: this.selectedSymbols,
      };

      axios
        .post("http://localhost:9797/api/crypto_list", properData)
        .then(() => {
          console.log("List created");
          this.dialogCreate = false;
          this.$emit("refresh-lists");
          this.loadingCreate = false;
        })
        .catch((error) => {
          console.error(error);
          this.loadingCreate = false;
        });
    },
  },
};
</script>

<style scoped>
.crypto-lists {
  /* Add your component styles here */
}

.multi-line-btn-toggle {
  display: flex;
  flex-wrap: wrap;
  height: auto;
}

.multi-line-btn-toggle .v-btn {
  /* margin: 4px; */
  padding: 10px;
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
