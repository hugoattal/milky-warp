<template>
    <div class="wrapper" @wheel="updateZoom">
        <div class="screen" :style="screenStyle"></div>
    </div>
</template>


<script setup lang="ts">
import { computed, onBeforeMount, onMounted, onUnmounted, reactive, ref, watchEffect } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { window } from "@tauri-apps/api";
import { LogicalPosition, LogicalSize } from "@tauri-apps/api/window";

const props = defineProps<{
    screenshotPath: string;
}>();

const WINDOW_SIZE_X = 256;
const WINDOW_SIZE_Y = 128;

const move = ref(false);

const cursor = reactive({ x: 0, y: 0 });

const zoomLevel = ref(0);
const targetZoomLevel = ref(0);
const scale = computed(() => Math.pow(1.5, targetZoomLevel.value));

const savedLocation = reactive({ x: 0, y: 0 });
const targetLocation = reactive({ x: 0, y: 0 });

const screenStyle = computed(() => {
    return {
        backgroundImage: `url(${props.screenshotPath})`,
        backgroundPosition: `${-savedLocation.x}px ${-savedLocation.y}px`
    };
});


onBeforeMount(async () => {
    await Promise.all([
        await window.getCurrent().setDecorations(false),
        await window.getCurrent().setAlwaysOnTop(true),
        await window.getCurrent().setSize(new LogicalSize(WINDOW_SIZE_X, WINDOW_SIZE_Y))
    ]);
});

onMounted(async () => {
    move.value = true;
    await moveLoop();
});

onUnmounted(() => {
    move.value = false;
});

async function updateZoom(event: WheelEvent) {
    zoomLevel.value -= event.deltaY / 100;
    zoomLevel.value = Math.max(0, zoomLevel.value);
    zoomLevel.value = Math.min(4, zoomLevel.value);
}

watchEffect(async () => {
    await window.getCurrent().setSize(new LogicalSize(scale.value * WINDOW_SIZE_X, scale.value * WINDOW_SIZE_Y));
});

async function windowMove() {
    const mouseLocation = await invoke("get_mouse_location", {});
    const location = mouseLocation.split(";").map(v => parseInt(v));
    cursor.x = location[0];
    cursor.y = location[1];

    targetLocation.x = cursor.x - (scale.value * WINDOW_SIZE_X) / 2;
    targetLocation.y = cursor.y - (scale.value * WINDOW_SIZE_Y) / 2;

    const windowLocation = new LogicalPosition(targetLocation.x, targetLocation.y);

    await Promise.all([
        updateSavedLocation(),
        window.appWindow.setPosition(windowLocation)
    ]);
}

async function updateSavedLocation() {
    targetZoomLevel.value = alpha(targetZoomLevel.value, zoomLevel.value, 0.2);

    savedLocation.x = alpha(savedLocation.x, targetLocation.x, 0.2);
    savedLocation.y = alpha(savedLocation.y, targetLocation.y, 0.2);

    function alpha(a: number, b: number, alpha: number) {
        return a * (1 - alpha) + b * alpha;
    }
}

async function moveLoop() {
    await windowMove();

    if (move.value) {
        requestAnimationFrame(moveLoop);
    }
}

async function toggleMove() {
    move.value = !move.value;
    await moveLoop();
}
</script>

<style scoped lang="scss">
.wrapper {
    width: 100%;
    height: 100%;
    overflow: hidden;
    display: flex;

    .screen {
        width: 100%;
        height: 100%;
        transform: scale(v-bind(scale));
    }
}
</style>
