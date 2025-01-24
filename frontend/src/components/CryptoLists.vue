<template>
    <v-data-table multi-sort :headers="headers" :items="items" class="pa-6">
        <template v-slot:item.hidden="{ item }">
            <v-chip :color="item.hidden ? 'green' : 'red'" :text="item.hidden ? 'Hidden' : 'Visible'"
                class="text-uppercase" size="small" label>
            </v-chip>
        </template>
    </v-data-table>
</template>

<script lang="ts">

    export default {
        name: 'CryptoLists',
        data() {
            return {
                headers: [
                    { title: 'Name', value: 'name' },
                    { title: 'Interval', value: 'interval' },
                    { title: 'Limit', value: 'limit_count' },
                    { title: 'Type', value: 'type' },
                ],
                // Add your component data here
            };
        },
        mounted() {
            // Add your mounted logic here
        },
        methods: {
            // Add your component methods here
        },
    }
</script>

<style scoped>
    .crypto-lists {
        /* Add your component styles here */
    }
</style>
