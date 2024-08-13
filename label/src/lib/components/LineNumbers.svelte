<script lang="ts">
	import { type ColoredMarkSpan, type MarkSpan } from '$lib/index';
	export let length: number;
	export let currentSpan: MarkSpan | null;
	export let spans: ColoredMarkSpan[];

	$: maxChars = Math.floor(Math.log10(length)) + 1;

	function generateLineNumber(i: number): string {
		const charsForI: number = Math.floor(Math.log10(i)) + 1;
		const spaces: string = Array.from({ length: maxChars - charsForI }, (_) => ' ').join('');

		const markers: string = Array.from({ length: spans.length }, (_, ix) => {
			// Start from the outside and work in, with the first spans closest to the numbers
			const index = spans.length - ix - 1;
			const current = spans[index];
			const spanOpen = `<span style="color: ${current.color}">`;
			const spanClose = '</span>';
			if (current.span.start === i || current.span.end === i) {
				return `${spanOpen}●${spanClose}`;
			} else if (current.span.start < i && current.span.end > i) {
				return `${spanOpen}┊${spanClose}`;
			} else {
				return ' ';
			}
		}).join('');

		// Add an extra character for spacing
		return ` ${markers}${spaces}${i}`;
	}

	function processClick(i: number) {
		i = i + 1;
		if (
			currentSpan === null ||
			(currentSpan.start != currentSpan.end && (i < currentSpan.start || i > currentSpan.end))
		) {
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
    {#key spans}
		{#each { length: length } as _, i}
			<button on:click={(_) => processClick(i)} class="contents text-right">
          <code
					class:highlight={currentSpan !== null &&
						currentSpan.start - 1 <= i &&
						currentSpan.end - 1 >= i}>{@html generateLineNumber(i + 1)}</code
				>
        </button>
		{/each}
	{/key}
</pre>

<style lang="css">
	.highlight {
		background-color: #ffe536;
	}
</style>
