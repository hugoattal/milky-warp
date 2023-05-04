<template>
    <MZoom :screenshot-path="screenshotPath" v-if="isWindowDisplayed"/>
</template>

<script setup lang="ts">
import MZoom from "./components/Zoom.vue";
import { register } from "@tauri-apps/api/globalShortcut";
import { window } from "@tauri-apps/api";
import { convertFileSrc, invoke } from "@tauri-apps/api/tauri";
import { onMounted, ref } from "vue";

const isWindowDisplayed = ref(false);
const screenshotPath = ref("");

onMounted(async () => {
    await window.getCurrent().hide();
    screenshotPath.value = convertFileSrc((await invoke("get_screenshot_path", {})).replaceAll("\\", "/"));
    //await window.getCurrent().setIgnoreCursorEvents(true);
});

register("Alt+Z", async () => {
    if (isWindowDisplayed.value) {
        await window.getCurrent().hide();
    }
    else {
        screenshotPath.value = convertFileSrc((await invoke("update_screenshot", {})).replaceAll("\\", "/")) + "?" + Date.now();
        await window.getCurrent().show();
    }

    isWindowDisplayed.value = !isWindowDisplayed.value;
});
</script>

<style lang="scss">
html, body {
    margin: 0;

    #app {
        height: 100vh;
        overflow-y: auto;
    }
}
</style>
