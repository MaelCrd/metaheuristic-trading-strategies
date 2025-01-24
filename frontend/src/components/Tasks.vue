<template>
    <v-data-table multi-sort :headers="headers" :items="items" class="pa-6">
    </v-data-table>
</template>

<script lang="ts">

    export default {
        name: 'Tasks',
        data() {
            return {
                headers: [
                    { title: 'State', value: 'state' },
                    { title: 'Created at', value: 'created_at' },
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
