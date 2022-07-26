<script lang="ts">
    import { appWindow } from "@tauri-apps/api/window";
    import FibButton from "./components/FibButton.svelte";

    let name = "alex";
    let focused = true;

    appWindow.onFocusChanged((state) => {
        focused = state.payload;
    });
</script>

<div
    data-tauri-drag-region
    class="relative h-12 z-10 flex justify-center items-center {focused
        ? 'bg-secondary'
        : 'bg-tertiary'} transition-colors duration-100"
>
    <input
        type="text"
        class="{focused
            ? 'bg-light'
            : 'bg-lightgray text-textlight'} rounded-md transition-colors duration-100 focus-visible:outline-none py-1 px-4"
    />
</div>
<main
    class="bg-primary absolute inset-0 h-auto pt-12 flex justify-center items-center"
>
    <FibButton />
</main>
