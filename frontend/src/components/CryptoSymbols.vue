<template>
  <v-col class="main-content">
    <v-row align="center" justify="space-between" class="mt-2 mr-8">
      <v-card-title>
        <h3>Crypto symbols</h3>
      </v-card-title>
      <div>
        <v-btn
          variant="outlined"
          :prepend-icon="iconRefresh"
          color="primary"
          :loading="loadingRefresh"
          @click="refreshSymbols"
        >
          Refresh
        </v-btn>
      </div>
    </v-row>
    <v-row>
      <v-data-table
        multi-sort
        :headers="headers"
        :sort-by="[
          { key: 'available', order: 'desc' },
          { key: 'volume', order: 'desc' },
        ]"
        :items="items"
        hover
        class="pl-6 pr-12 pb-6"
      >
        <template v-slot:item.available="{ item }">
          <v-chip
            :color="item.available ? 'green' : 'red'"
            :text="item.available ? 'Available' : 'Unavailable'"
            class="text-uppercase"
            size="small"
            label
          />
        </template>
      </v-data-table>
    </v-row>
  </v-col>
</template>

<script lang="ts">
import axios from "axios";

export default {
  name: "CryptoSymbols",
  props: {
    items: Array,
  },
  emits: ["refresh-symbols"],
  data() {
    return {
      headers: [
        { title: "Symbol", value: "symbol", sortable: true, width: "20%" },
        { title: "Volume", value: "volume", sortable: true, width: "20%" },
        {
          title: "Available",
          value: "available",
          sortable: true,
          width: "20%",
        },
        {
          title: "Last updated",
          value: "last_updated",
          sortable: true,
          width: "20%",
        },
      ],
      loadingRefresh: false,
      iconRefresh: "mdi-refresh",
    };
  },
  mounted() {
    // Add your mounted logic here
  },
  methods: {
    // Add your component methods here
    refreshSymbols() {
      console.log("Refreshing symbols");
      this.loadingRefresh = true;

      // Send POST request to the backend
      axios
        .post("http://localhost:9797/api/crypto_symbol/reload")
        .then(() => {
          // console.log(response.data);

          // Refresh the symbols
          this.$emit("refresh-symbols");
          this.loadingRefresh = false;
          this.iconRefresh = "mdi-check";
          setTimeout(() => {
            this.iconRefresh = "mdi-refresh";
          }, 2000);
        })
        .catch((error) => {
          console.error(error);
          this.loadingRefresh = false;
          this.iconRefresh = "mdi-alert-circle";
          setTimeout(() => {
            this.iconRefresh = "mdi-refresh";
          }, 2000);
        });
    },
  },
};
</script>

<style scoped>
.crypto-symbols {
  /* Add your component styles here */
}
</style>
