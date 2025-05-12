<script lang="ts">
	interface Props {
		href: string;
		enabled?: boolean;
		children?: import('svelte').Snippet;
	}

	let { href, enabled = true, children }: Props = $props();
</script>

<a
	class="border-solid border-2 rounded p-2 m-2 max-w-fit {enabled ? 'hover:bg-gray-50' : ''}"
	href={enabled ? href : '#'}>{@render children?.()}</a
>
