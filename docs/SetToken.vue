<template>
    <div style="margin-top: 2em">
        <div class="tk">
            <label>吉林一号 Token:</label>
            <input v-model="tokens.jl1" placeholder="未设置,请输入:" />
        </div>
        <div class="tk">
            <label>吉林一号(共生地球) Token:</label>
            <input v-model="tokens.jl1earth" placeholder="未设置,请输入:" />
        </div>
        <div class="tk">
            <label>地质云 Token:</label>
            <input v-model="tokens.geocloud" placeholder="未设置,请输入:" />
        </div>
    </div>
    <div class="save">
        <button @click="saveToken">保存</button>
    </div>
</template>

<script setup>
import { ref, onMounted } from "vue";
const tokens = ref({ jl1: "", jl1earth: "", geocloud: "" });

async function getTokens() {
    const r = await fetch("/config/tokens");
    const tk = await r.json();
    return tk;
}

async function saveToken() {
    const options = {
        method: "POST", // 请求方法
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(tokens.value),
    };
    const r = await fetch("/config/set_tokens", options);
    const ok = await r.text();
    if (ok == "ok") {
        alert("已保存");
    } else {
        alert("保存失败");
    }
}

onMounted(async () => {
    tokens.value = await getTokens();
});
</script>

<style scoped>
input {
    border: 1px solid #ccc;
    border-radius: 0.5em;
    padding: 0.2em;
    flex-grow: 1;
    font-family: monospace;
    padding-left: 0.5em;
}

input::placeholder {
    padding-left: 0.5em;
}

div.tk {
    display: flex;
    gap: 2em;
    margin-bottom: 0.4em;
}

div.save {
    text-align: right;
}

button {
    padding: 0.5em 1.8em;
    border-radius: 0.8em;
    color: var(--vp-button-brand-text);
    background-color: var(--vp-button-brand-bg);
}
</style>
