<template>
    <MZoom :screenshot-path="screenshotPath" :is-active="isWindowDisplayed"/>
</template>

<script setup lang="ts">
import MZoom from "./components/Zoom.vue";
import { register } from "@tauri-apps/api/globalShortcut";
import { window } from "@tauri-apps/api";
import { convertFileSrc, invoke } from "@tauri-apps/api/tauri";
import { nextTick, onMounted, ref } from "vue";
import { config } from "./config";

const isWindowDisplayed = ref(false);
const screenshotPath = ref("");

onMounted(async () => {
    screenshotPath.value = convertFileSrc((await invoke("get_screenshot_path", {})).replaceAll("\\", "/"));
    //await window.getCurrent().setIgnoreCursorEvents(true);
});

register(config.shortcut, async () => {
    isWindowDisplayed.value = !isWindowDisplayed.value;

    await nextTick();

    if (!isWindowDisplayed.value) {
        await window.getCurrent().hide();
    }
    else {
        screenshotPath.value = convertFileSrc((await invoke("update_screenshot", {})).replaceAll("\\", "/")) + "?" + Date.now();
        await window.getCurrent().show();
    }

});
</script>

<style lang="scss">
html, body {
    margin: 0;
    background-color: transparent;

    #app {
        height: 100vh;
        overflow-y: auto;
    }
}
</style>
