<script lang="ts">
    import Highlight from "svelte-highlight";
    import c from "svelte-highlight/languages/c";
    import cpp from "svelte-highlight/languages/cpp";
    import java from "svelte-highlight/languages/java";
    import python from "svelte-highlight/languages/python";
    import "svelte-highlight/styles/github.css";
    import LineNumbers from "./LineNumbers.svelte";
    import {type MarkSpan} from "$lib/index.ts";

    export let spans: MarkSpan[] = [];
    export let currentSpan: MarkSpan | null;

    function code() {
      let data: String[] = ["def hi():"]
      for (let i = 0; i < 100; i++) {
        data.push("  print('Hello, world! This is a very long string that should overflow the page and cause a scroll bar to appear.')")
      }

      return data.join("\n");
    };
</script>

<div class="w-1/2 overflow-y-auto">
    <div class="flex">
        <LineNumbers length={code().split("\n").length} bind:currentSpan={currentSpan}/>
        <!-- <pre class="overflow-x-auto w-full"><code>{code()}</code></pre> -->
        <div class="overflow-x-auto overflow-y-visible">
            <Highlight language={python} code={code()}/>
        </div>
    </div>
</div>

<style>
  :global(code.hljs) {
    padding: 0 0 1em 0 !important;
    /* margin: 0 !important; */
  }
</style>
