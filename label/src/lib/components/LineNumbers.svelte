<script lang="ts">
    import {type MarkSpan} from "$lib/index.ts";
    export let length: number;
    export let currentSpan: MarkSpan | null;

    $: maxChars = Math.floor(Math.log10(length)) + 1;

    function generateLineNumber(i: number): string {
      const charsForI: number = Math.floor(Math.log10(i)) + 1;
      const spaces: string = Array.from({length: maxChars - charsForI}, _ => " ").join("");
      // Add an extra character for spacing
      return ` ${spaces}${i}`
    }

    function processClick(i: number) {
      i = i + 1;
      if (currentSpan === null || (currentSpan.start != currentSpan.end && (i < currentSpan.start || i > currentSpan.end))) {
        currentSpan = {
          start: i,
          end: i
        };
      } else if (currentSpan.start === currentSpan.end) {
        if (i < currentSpan.start) {
          currentSpan.start = i;
        } else if (i > currentSpan.end) {
          currentSpan.end = i;
        } else {
          currentSpan = null;
        }
      } else {
        currentSpan = null;
      }
    }
</script>

<pre class="text-right bg-slate-100 flex flex-col">
    {#each {length: length} as _, i}
      <button on:click={_ => processClick(i)} class="contents text-right" >
          <code class:highlight={currentSpan !== null && currentSpan.start - 1 <= i && currentSpan.end - 1 >= i}>{@html generateLineNumber(i+1)}</code>
        </button>
    {/each}
</pre>

<style lang="css">
    .highlight {
        background-color: #ffe536;
    }
</style>
