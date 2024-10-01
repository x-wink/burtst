<template>
    <div class="container">
        <label>鼠标连点</label>
        <div class="list">
            <label v-for="(item, index) in mouseOpts" :key="index">
                <input type="radio" v-model="config.mouse" name="mouse" :value="item.value" />
                {{ item.label }}
            </label>
        </div>
        <label>按键连点</label>
        <div class="list">
            <select name="keyboard" v-model="config.keyboard">
                <option v-for="(item, index) in keyboardOpts" :key="index" :value="item.value">{{ item.label }}</option>
            </select>
        </div>
        <label>间隔时间（毫秒）</label>
        <input type="number" :min="10" :max="9999" v-model="config.interval" />
        <div class="btns">
            <button @click="save">保存设置</button>
            <button class="primary" @click="running ? stop() : start()">{{ running ? "停止" : "开始" }}连发</button>
        </div>
    </div>
</template>
<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { message } from "@tauri-apps/plugin-dialog";
import { ref } from "vue";
const mouseOpts = ["无", "左键", "右键", "中键"].map((label, value) => ({ label, value }));
const keyboardOpts = [
    "无",
    "1",
    "2",
    "3",
    "4",
    "5",
    "6",
    "7",
    "8",
    "9",
    "0",
    "A",
    "B",
    "C",
    "D",
    "E",
    "F",
    "G",
    "H",
    "I",
    "J",
    "K",
    "L",
    "M",
    "N",
    "O",
    "P",
    "Q",
    "R",
    "S",
    "T",
    "U",
    "V",
    "W",
    "X",
    "Y",
    "Z",
    "Num1",
    "Num2",
    "Num3",
    "Num4",
    "Num5",
    "Num6",
    "Num7",
    "Num8",
    "Num9",
    "Num0",
].map((label, value) => ({
    label,
    value,
}));
const alert = async (content: string, title?: string) => {
    message(content, { title, okLabel: "知道了" });
};

const config = ref(
    localStorage.getItem("config")
        ? JSON.parse(localStorage.getItem("config")!)
        : {
              mouse: 0,
              keyboard: 1,
              interval: 100,
          }
);
const save = async () => {
    localStorage.setItem("config", JSON.stringify(config.value));
    await invoke("setup", {
        config: config.value,
    });
    alert("保存成功");
};
const running = ref(false);
const start = async () => {
    alert("开始连发");
    running.value = true;
    await invoke("start");
};
const stop = async () => {
    await invoke("stop");
    running.value = false;
    alert("停止连发");
};
</script>
<style lang="less">
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
* {
    padding: 0;
    margin: 0;
    box-sizing: border-box;
}
button {
    padding: 5px 10px;
    border: none;
    cursor: pointer;
    border-radius: 5px;
    &.primary {
        background-color: #007bff;
        color: #fff;
    }
}
input:not([type="radio"]),
select {
    width: 120px;
}
.container {
    width: 100vw;
    height: 100vh;
    padding: 10px;
    display: flex;
    flex-direction: column;
    gap: 10px;
    .list {
        display: flex;
        gap: 20px;
    }
    .btns {
        margin-top: auto;
        display: flex;
        gap: 10px;
        justify-content: flex-end;
    }
}
</style>
