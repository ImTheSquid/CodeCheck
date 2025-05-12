<script lang="ts">
	let { error } = $props();
</script>

{#if error !== null}
	<p style="color: red">Error: {error}</p>
{/if}
