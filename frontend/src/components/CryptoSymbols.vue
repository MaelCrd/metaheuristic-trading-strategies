<template>
    <v-col class="main-content">
        <v-row align="center" justify="space-between" class="mt-4 mr-8">
            <v-card-title>
                <h3>Crypto symbols</h3>
            </v-card-title>
            <div>
                <v-btn prepend-icon="mdi-refresh" color="primary" @click="refreshSymbols">Refresh</v-btn>
            </div>
        </v-row>
        <v-row>
            <v-data-table multi-sort :headers="headers" :items="items" class="pa-6">
                <template v-slot:item.available="{ item }">
                    <v-chip :color="item.available ? 'green' : 'red'"
                        :text="item.available ? 'Available' : 'Unavailable'" class="text-uppercase" size="small" label>
                    </v-chip>
                </template>
            </v-data-table>
        </v-row>
    </v-col>
</template>

<script lang="ts">

    export default {
        name: 'CryptoSymbols',
        props: {
            items: Array,
        },
        data() {
            return {
                headers: [
                    { title: 'Symbol', value: 'symbol' },
                    { title: 'Volume', value: 'volume' },
                    { title: 'Available', value: 'available' },
                    { title: 'Last updated', value: 'last_updated' },
                ],
                // Add your component data here
            };
        },
        mounted() {
            // Add your mounted logic here
        },
        methods: {
            // Add your component methods here
            refreshSymbols() {
                console.log('Refreshing symbols');
            },
        },
    }
</script>

<style scoped>
    .crypto-symbols {
        /* Add your component styles here */
    }
</style>
