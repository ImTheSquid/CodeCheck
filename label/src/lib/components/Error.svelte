<script>
	export let error;
</script>

{#if error !== null}
	<p style="color: red">Error: {error}</p>
{/if}
