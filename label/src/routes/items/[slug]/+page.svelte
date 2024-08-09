<script>
    import {onMount} from "svelte";
    import {invoke} from "@tauri-apps/api/core";
    export let data;

    onMount(async () => {
      return () => {

      };
    });
</script>
<h1>Comparison {data.slug + 1}</h1>
<a href="/items">Main Page</a>
<p>Use your mouse to select pairs of plagiarized spans. This page will save automatically when you leave</p>
