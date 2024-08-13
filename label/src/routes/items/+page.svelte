<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
	import Error from '$lib/components/Error.svelte';
	let err: string | null = null;
	let overviewData: (number | null)[] | null = null;
	let goto: number = 0;
	$: {
		if (goto < 0) {
			goto = 0;
		} else if (goto >= (overviewData?.length ?? 1)) {
			goto = (overviewData?.length ?? 1) - 1;
		}
	}
	onMount(async () => {
		try {
			overviewData = await invoke('get_overview');
		} catch (error) {
			err = error;
		}
	});
	const minCellWidth = 150;
</script>

<h1 class="text-xl font-bold">Dataset Items Overview</h1>
<Error error={err} />
{#if overviewData !== null}
	<p>
		This dataset has {overviewData.length} comparisons and is {overviewData.filter(
			(itm) => itm !== null
		).length / overviewData.length}% complete.
	</p>

	<div class="flex p-1">
		<p class="font-bold">GOTO INDEX:</p>
		<input type="number" class="pl-1 pr-1" bind:value={goto} />
		<a href="items/view?i={goto}" class="text-green-600">JUMP</a>
	</div>

	{#if overviewData.length < 20_000}
		<div class="grid-table" style="--cell-width: {minCellWidth}px;">
			{#each overviewData as item, i}
				<a
					class="grid-cell {item !== null ? 'done' : ''}"
					data-index={i + 1}
					href="items/view?i={i}"
				>
					{#if item === null}
						<p>NOT VIEWED</p>
					{:else}
						{item}
					{/if}
				</a>
			{/each}
		</div>
	{:else}
		<p>Too many comparisons to show graphic :(</p>
	{/if}
{/if}

<style lang="css">
	.grid-table {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(var(--cell-width), 1fr));
		gap: 10px;
		width: 100%;
	}

	.grid-cell {
		position: relative;
		padding: 20px;
		border: 1px solid #ccc;
		text-align: center;
		background-color: #f9f9f9;
		overflow: hidden;
		cursor: pointer;
		text-decoration: none;
		color: inherit;
	}

	.grid-cell::before {
		content: attr(data-index);
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		font-size: 4rem; /* Adjust size as needed */
		color: rgba(0, 0, 0, 0.1); /* Light color for the background number */
		z-index: 0;
	}

	.grid-cell > * {
		position: relative;
		z-index: 1;
	}

	.done {
		background-color: #55f955;
	}
</style>
