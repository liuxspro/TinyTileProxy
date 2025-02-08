<template>
    <code><a :href="address + path"  target="_blank" rel="noopener noreferrer">{{ address + path }}</a></code>
</template>

<script setup>
const props = defineProps(['path'])
import { ref, onMounted } from 'vue'
const address = ref();

onMounted(() => {
    address.value = window.location.origin;
})
</script>