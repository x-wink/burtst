<template>
    <div class="container">
        <label v-for="(item, index) in mouseOpts" :key="index">
            <input type="radio" v-model="config.mouse" name="mouse" :value="item.value" />
            {{ item.label }}
        </label>
        <hr />
        <label v-for="(item, index) in keyboardOpts" :key="index">
            <input type="radio" v-model="config.keyboard" name="keyboard" :value="item.value" />
            {{ item.label }}
        </label>
        <hr />
        {{ config }}
        <hr />
        <button @click="start">开始连发</button>
        <button @click="stop">停止连发</button>
    </div>
</template>
<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { message } from "@tauri-apps/plugin-dialog";
import { register } from "@tauri-apps/plugin-global-shortcut";
import { ref } from "vue";
const mouseOpts = ["无", "左键", "右键", "中键"].map((label, value) => ({ label, value }));
const keyboardOpts = ["无", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "a", "b", "c",
    "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u",
    "v", "w", "x", "y", "z", "Num 1", "Num 2", "Num 3", "Num 4", "Num 5", "Num 6", "Num 7", "Num 8", "Num 9", "Num 0"
].map((label, value) => ({
    label,
    value,
}));
const alert = async (content: string, title?: string) => {
    message(content, { title, okLabel: "知道了" });
};

const config = ref({
    mouse: 0,
    keyboard: 1,
    interval: 100,
});
const save = async () => {
    await invoke("setup", {
        config: config.value,
    });
};
save();
const start = async () => {
    alert("开始连发");
    await invoke("start");
};
const stop = async () => {
    await invoke("stop");
    alert("停止连发");
};
register("CommandOrControl+Plus", start);
register("CommandOrControl+Minus", stop);
</script>
<style lang="less">
.container {
    display: flex;
    flex-direction: column;
}

:root {
    --fc: #0f0f0f;
    --bgc: #f6f6f6;

    color: var(--fc);
    background-color: var(--bgc);

    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;
    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-text-size-adjust: 100%;
}

@media (prefers-color-scheme: dark) {
    :root {
        --fc: #f6f6f6;
        --bgc: #2f2f2f;
    }
}
</style>
