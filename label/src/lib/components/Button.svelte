<script lang="ts">
	import { createBubbler } from 'svelte/legacy';

	const bubble = createBubbler();
	interface Props {
		enabled?: boolean;
		border?: string | null;
		children?: import('svelte').Snippet;
	}

	let { enabled = true, border = null, children }: Props = $props();
</script>

<button
	onclick={bubble('click')}
	class="border-solid border-2 rounded p-2 m-2 max-w-fit {border ?? ''} {enabled
		? 'hover:bg-gray-50'
		: ''}"
	disabled={!enabled}>{@render children?.()}</button
>
