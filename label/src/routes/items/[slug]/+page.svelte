<script lang="ts">
    import {onMount} from "svelte";
    import {invoke} from "@tauri-apps/api/core";
    import CodeViewer from "$lib/components/CodeViewer.svelte";
    import {type MarkSpan, type Mark} from "$lib/index.ts";
    export let data;

    let aCurrent: MarkSpan | null = null;
    let bCurrent: MarkSpan | null = null;
    let marks: Mark[] = [];

    $: allowCommit = !(aCurrent === null || bCurrent === null);
    $: aMarks = marks.map(m => m.a);
    $: bMarks = marks.map(m => m.b);

    onMount(async () => {
      return () => {

      };
    });

    function commitSpans() {
      marks = [...marks, {
        a: aCurrent,
        b: bCurrent,
      }];

      aCurrent = null;
      bCurrent = null;
    }
</script>
<div class="flex flex-col h-screen">
    <h1 class="text-xl font-bold">Comparison {data.slug + 1}</h1>
    <a class="border-solid border-2 rounded p-2 m-2 max-w-fit" href="/items">Main Page</a>
    <p>Use your mouse to select pairs of plagiarized spans via the line numbers, then click &quotCommit&quot. This page will save your commited spans automatically when you leave.</p>
    <button on:click={_ => commitSpans()} class="border-solid border-2 rounded p-2 m-2 max-w-fit {allowCommit ? 'border-lime-500' : 'border-slate-50'}" disabled={!allowCommit}>Commit</button>
    <div class="flex flex-row min-h-0">
        <CodeViewer bind:currentSpan={aCurrent}/>
        <CodeViewer bind:currentSpan={bCurrent}/>
    </div>
</div>
