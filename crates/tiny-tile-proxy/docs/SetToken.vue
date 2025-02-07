<template>
    <div id="pwdcontainer" style="margin-top: 2em">
        <div class="pwd">
            <label>请输入密码:</label>
            <input v-model="pwd" placeholder="请输入密码" type="password" />
            <button @click="auth">访问</button>
        </div>
        <p class="tip" v-if="pwd == 'ttp123456'">请注意修改默认密码!</p>
    </div>
    <div style="margin-top: 2em" id="tkset" v-if="authed">
        <div class="tk">
            <label>吉林一号 Token:</label>
            <input v-model="tokens.jl1" placeholder="未设置,请输入" />
        </div>
        <div class="tk">
            <label>吉林一号(共生地球) Token:</label>
            <input v-model="tokens.jl1earth" placeholder="未设置,请输入" />
        </div>
        <div class="tk">
            <label>地质云 Token:</label>
            <input v-model="tokens.geocloud" placeholder="未设置,请输入" />
        </div>
        <div class="save">
            <button @click="saveToken">保存</button>
        </div>
    </div>
</template>

<script setup>
import { ref } from "vue";
const pwd = ref("");
const authed = ref(false);
const tokens = ref({ jl1: "", jl1earth: "", geocloud: "" });

async function getTokens() {
    const r = await fetch(`/config/tokens?pwd=${pwd.value}`);
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
    const r = await fetch(`/config/set_tokens?pwd=${pwd.value}`, options);
    const ok = await r.text();
    if (ok == "ok") {
        alert("已保存");
    } else {
        alert("保存失败");
    }
}

async function auth() {
    const r = await fetch(`/config/auth?pwd=${pwd.value}`);
    const ok = await r.text();
    if (ok == "Ok") {
        authed.value = true;
        tokens.value = await getTokens();
    } else {
        alert("密码错误");
    }
}
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

div.tk,
div.pwd {
    display: flex;
    gap: 2em;
    margin-bottom: 0.4em;
    align-items: center;
}

div.save {
    text-align: right;
}

button {
    padding: 0.4em 1.4em;
    border-radius: 0.8em;
    color: var(--vp-button-brand-text);
    background-color: var(--vp-button-brand-bg);
}

p.tip {
    color: red;
    font-size: 0.85em;
}
</style>
