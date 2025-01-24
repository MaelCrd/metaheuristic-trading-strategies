<template>
    <v-data-table multi-sort :headers="headers" :items="items" class="pa-6">
    </v-data-table>
</template>

<script lang="ts">

    export default {
        name: 'Metaheuristics',
        data() {
            return {
                headers: [
                    { title: 'Name', value: 'mh_algorithm_name' },
                    { title: 'Parameters', value: 'mh_parameters' },
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
    .metaheuristics {
        /* Add your component styles here */
    }
</style>
