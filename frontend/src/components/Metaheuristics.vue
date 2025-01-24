<template>
    <v-col class="main-content">
        <v-row align="center" justify="space-between" class="mt-2 mr-8">
            <v-card-title>
                <h3>Metaheuristics objects</h3>
            </v-card-title>
            <div>
                <v-btn variant="outlined" prepend-icon="mdi-plus" color="primary" @click="dialogCreate = true">
                    Create</v-btn>
            </div>
        </v-row>
        <v-row>
            <v-data-table multi-sort :headers="headers" :items="items" class="pl-6 pr-6 pb-6">
                <template v-slot:item.mh_parameters="{ item }">
                    <v-chip v-for="(param, index) in JSON.parse(item.mh_parameters)" :key="index" size="small"
                        class="mr-2" color="success">
                        {{ index }} : {{ param }}
                    </v-chip>
                </template>
            </v-data-table>
        </v-row>
    </v-col>

</template>

<script lang="ts">

    export default {
        name: 'Metaheuristics',
        props: {
            items: Array,
        },
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
