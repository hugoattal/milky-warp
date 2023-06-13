<template>
    <div class="wrapper" @wheel="updateZoom" @click="toggleMove">
        <div class="screen" :style="screenStyle"></div>
    </div>
</template>


<script setup lang="ts">
import {computed, onBeforeMount, watch, reactive, ref} from "vue";
import {invoke} from "@tauri-apps/api/tauri";
import {window} from "@tauri-apps/api";
import {LogicalPosition, LogicalSize} from "@tauri-apps/api/window";

const props = defineProps<{
    screenshotPath: string;
    isActive: boolean;
}>();

const WINDOW_SIZE_X = 256;
const WINDOW_SIZE_Y = 128;

const move = ref(true);
const lockOnScreen = ref(false);

const zoomLevel = ref(0);
const targetZoomLevel = ref(0);
const scale = computed(() => Math.pow(1.5, targetZoomLevel.value));

const cursor = {x: 0, y: 0};
const savedLocation = reactive({x: 0, y: 0});
const targetLocation = {x: 0, y: 0};
const monitor = {
    size: {x: 0, y: 0},
    position: {x: 0, y: 0},
    scale: 1
}

let forceInstantMove = false;

const screenStyle = computed(() => {
    return {
        backgroundImage: `url(${props.screenshotPath})`,
        backgroundPosition: `${-savedLocation.x + monitor.position.x / monitor.scale}px ${-savedLocation.y + monitor.position.y / monitor.scale}px`,
        backgroundSize: `${monitor.size.x / monitor.scale}px ${monitor.size.y / monitor.scale}px`,
    };
});

onBeforeMount(updateWindowSize);

watch(() => scale.value, updateWindowSize);

watch(() => props.isActive, async () => {
    if (props.isActive) {
        forceInstantMove = true;
        move.value = true;
        lockOnScreen.value = false;
        await moveLoop();
        await updateMonitor();
        lockOnScreen.value = true;
    }
}, {
    immediate: true
});

async function updateMonitor() {
    const currentMonitor = await window.currentMonitor();
    if (!currentMonitor) {
        return;
    }
    const position = currentMonitor.position;
    const size = currentMonitor.size;
    const scaleFactor = currentMonitor.scaleFactor

    monitor.position = {x: position.x, y: position.y};
    monitor.size = {x: size.width, y: size.height};
    monitor.scale = scaleFactor;
}

async function updateZoom(event: WheelEvent) {
    zoomLevel.value -= event.deltaY / 100;
    zoomLevel.value = Math.max(0, zoomLevel.value);
    zoomLevel.value = Math.min(4, zoomLevel.value);
}

async function updateWindowSize() {
    await window.getCurrent().setSize(
        new LogicalSize(
            scale.value * WINDOW_SIZE_X,
            scale.value * WINDOW_SIZE_Y
        )
    );
}

async function windowMove() {
    const mouseLocation = await invoke("get_mouse_location", {});
    const location = mouseLocation.split(";").map(v => parseInt(v) / monitor.scale);
    cursor.x = location[0];
    cursor.y = location[1];

    targetLocation.x = cursor.x - (scale.value * WINDOW_SIZE_X) / 2;
    targetLocation.y = cursor.y - (scale.value * WINDOW_SIZE_Y) / 2;

    const moveWindow = async () => {
        await window.appWindow.setPosition(getWindowPosition());
    };

    await Promise.all([
        updateSavedLocation(),
        moveWindow()
    ]);
}

function getWindowPosition() {
    if (!lockOnScreen.value) {
        return new LogicalPosition(
            targetLocation.x,
            targetLocation.y
        );
    }

    return new LogicalPosition(
        clamp(targetLocation.x, monitor.position.x, monitor.position.x + monitor.size.x / monitor.scale - scale.value * WINDOW_SIZE_X),
        clamp(targetLocation.y, monitor.position.y, monitor.position.y + monitor.size.y / monitor.scale - scale.value * WINDOW_SIZE_Y)
    );

    function clamp(value: number, min: number, max: number) {
        return Math.min(Math.max(value, min), max);
    }
}

async function updateSavedLocation() {
    const alpha = forceInstantMove ? 1 : 0.3;
    forceInstantMove = false;

    targetZoomLevel.value = lerp(targetZoomLevel.value, zoomLevel.value, alpha);

    savedLocation.x = lerp(savedLocation.x, targetLocation.x, alpha);
    savedLocation.y = lerp(savedLocation.y, targetLocation.y, alpha);

    function lerp(a: number, b: number, alpha: number) {
        return a * (1 - alpha) + b * alpha;
    }
}

async function moveLoop() {
    await windowMove();
    if (props.isActive && move.value) {
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
    border-radius: 16px;
    width: 100%;
    height: 100%;
    overflow: hidden;
    display: flex;
    position: relative;

    &::after {
        content: "";
        position: absolute;
        inset: 0;
        border-radius: 16px;
        box-shadow: inset 0 0 0 1px rgba(0, 0, 0, 0.2);
    }

    .screen {
        image-rendering: pixelated;
        opacity: 1;
        width: 100%;
        height: 100%;
        transform: scale(v-bind(scale));
        background-repeat: no-repeat;
    }
}
</style>
