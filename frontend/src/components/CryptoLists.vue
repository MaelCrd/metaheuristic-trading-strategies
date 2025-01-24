<template>
    <v-col class="main-content">
        <v-row align="center" justify="space-between" class="mt-2 mr-8">
            <v-card-title>
                <h3>Crypto lists</h3>
            </v-card-title>
            <div>
                <v-btn variant="outlined" prepend-icon="mdi-plus" color="primary" @click="dialogCreate = true">
                    Create</v-btn>
            </div>
        </v-row>
        <v-row>
            <v-data-table multi-sort :headers="headers" :items="items" class="pl-6 pr-6 pb-6">
                <template v-slot:item.hidden="{ item }">
                    <v-chip :color="item.hidden ? 'green' : 'red'" :text="item.hidden ? 'Hidden' : 'Visible'"
                        class="text-uppercase" size="small" label>
                    </v-chip>
                </template>
            </v-data-table>
        </v-row>

        <!--  -->
        <v-dialog v-model="dialogCreate" max-width="500px" opacity="0.1">
            <v-card>
                <v-card-title class="mt-3 ml-3">Create crypto list</v-card-title>
                <v-card-text>
                    <v-form ref="form" v-model="valid">
                        <v-text-field variant="outlined" v-model="formData.name" label="List name"
                            :rules="[v => !!v || 'Name is required']" required />
                        <h4 class="mb-1">Interval</h4>
                        <v-btn-toggle v-model="intervalSelected" color="accent" rounded="0" group
                            class="multi-line-btn-toggle mb-8">
                            <v-btn v-for="interval in intervals" :key="interval" :value="interval"
                                class="text-capitalize">{{
                                    interval }}</v-btn>
                        </v-btn-toggle>
                        <v-row>
                            <v-col cols="6">
                                <v-text-field variant="outlined" v-model="formData.durationValue" label="Duration"
                                    type="number" :rules="[v => !!v || 'Duration is required']" required />
                            </v-col>
                            <v-col cols="6">
                                <v-select variant="outlined" v-model="formData.durationUnit" :items="durations"
                                    label="Unit" :rules="[v => !!v || 'Unit is required']" required />
                            </v-col>
                        </v-row>
                        <v-select variant="outlined" v-model="formData.type" :items="types" label="Type"
                            :rules="[v => !!v || 'Type is required']" required />
                        <v-btn class="mb-6" color="secondary" prepend-icon="mdi-selection-search"
                            @click="dialogSelectSymbols = true">
                            Change symbols ({{ selectedSymbols.length }})
                        </v-btn>
                        <div></div>
                        <v-btn color="success" @click="createList" :loading="loadingCreate"
                            :disabled="!valid || selectedSymbols.length === 0 || !intervalSelected">
                            Create
                        </v-btn>
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
                            <v-text-field v-model="search" label="Search" prepend-inner-icon="mdi-magnify"
                                variant="outlined" hide-details single-line></v-text-field>
                        </v-col>
                    </v-row>
                    <v-row>
                        <v-col cols="12">
                            <v-data-table :headers="headersSymbols" :items="crypto_symbols" density="compact"
                                class="pa-2"
                                :sort-by="[{ key: 'available', order: 'desc' }, { key: 'volume', order: 'desc' }]"
                                :search="search" show-select v-model="selectedSymbols" multi-sort>
                                <template v-slot:item.available="{ item }">
                                    <v-icon :color="item.available ? 'green' : 'red'">
                                        {{ item.available ? 'mdi-check' : 'mdi-close' }}
                                    </v-icon>
                                </template>
                            </v-data-table>
                        </v-col>
                    </v-row>
                    <v-row align="center" justify="space-between">
                        <v-col cols="4">
                            <v-btn variant="outlined" color="error" @click="selectedSymbols = []"
                                prepend-icon="mdi-close">
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
    import axios from 'axios';


    export default {
        name: 'CryptoLists',
        props: {
            items: Array,
            crypto_symbols: Array,
        },
        emits: ['refresh-lists'],
        data() {
            return {
                // Add your component data here
                headers: [
                    { title: 'Name', value: 'name' },
                    { title: 'Interval', value: 'interval' },
                    { title: 'Limit', value: 'limit_count' },
                    { title: 'Type', value: 'type' },
                ],
                dialogCreate: false,
                dialogSelectSymbols: false,
                headersSymbols: [
                    { title: 'Symbol', value: 'symbol', sortable: true },
                    { title: 'Volume', value: 'volume', sortable: true },
                    { title: 'Available', value: 'available', sortable: true },
                ],
                selectedSymbols: [],
                valid: false,
                search: "",
                intervalSelected: null,
                loadingCreate: false,
                formData: {
                    name: '',
                    interval: null,
                    durationValue: null,
                    durationUnit: null,
                    type: null,
                },
                intervals: [
                    '1m', '5m', '15m', '30m', '1h', '2h', '4h', '6h', '8h', '12h', '1d', '3d', '1w', '1M',
                ],
                durations: [
                    'Minutes', 'Hours', 'Days', 'Weeks', 'Months', 'Years',
                ],
                types: [
                    'Type X',
                    'Type Y',
                    'Type Z'
                ],
            };
        },
        mounted() {
            // Add your mounted logic here
        },
        methods: {
            // Add your component methods here
            createList() {
                console.log('Creating list');
                this.loadingCreate = true;
                console.log(this.formData);
                console.log(this.selectedSymbols);

                let limit_minutes = this.formData.durationValue;
                switch (this.formData.durationUnit) {
                    case 'Hours':
                        limit_minutes *= 60;
                        break;
                    case 'Days':
                        limit_minutes *= 60 * 24;
                        break;
                    case 'Weeks':
                        limit_minutes *= 60 * 24 * 7;
                        break;
                    case 'Months':
                        limit_minutes *= 60 * 24 * 30;
                        break;
                    case 'Years':
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

                axios.post('http://localhost:9797/api/crypto_list', properData)
                    .then(() => {
                        console.log('List created');
                        this.dialogCreate = false;
                        this.$emit('refresh-lists');
                        this.loadingCreate = false;
                    })
                    .catch(error => {
                        console.error(error);
                        this.loadingCreate = false;
                    });
            },
        },
    }
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
</style>