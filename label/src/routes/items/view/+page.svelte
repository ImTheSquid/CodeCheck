<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
	import CodeViewer from '$lib/components/CodeViewer.svelte';
	import { type MarkSpan, type Mark, HslGenerator, type PairData, type Item } from '$lib/index';
	import { page } from '$app/stores';
	const slug: string = $page.url.searchParams.get('i') ?? '0';

	let aCurrent: MarkSpan | null = null;
	let bCurrent: MarkSpan | null = null;
	let generator: HslGenerator | null = null;
	let marks: Mark[] = [];
	let aItem: Item | null = null;
	let bItem: Item | null = null;
	let lang: string | null = null;

	$: allowCommit = !(aCurrent === null || bCurrent === null);
	$: aMarks = marks.map((m) => ({ span: m.a, color: m.color }));
	$: bMarks = marks.map((m) => ({ span: m.b, color: m.color }));

	onMount(async () => {
		generator = new HslGenerator();
		const pairData: PairData = await invoke('load_pair', { pairIndex: parseInt(slug) });
		aItem = pairData.a;
		bItem = pairData.b;
		marks = pairData.marks;
		lang = pairData.lang;
		for (let mark of marks) {
			mark.color = generator.generateColor();
		}
		return () => {};
	});

	function commitSpans() {
		if (!marks.some((m) => m.a === aCurrent && m.b === bCurrent)) {
			marks = [
				...marks,
				{
					a: aCurrent,
					b: bCurrent,
					color: generator?.generateColor() ?? '#000'
				}
			];
		}

		aCurrent = null;
		bCurrent = null;
	}
</script>

<div class="flex flex-col h-screen">
	<h1 class="text-xl font-bold">Comparison {parseInt(slug) + 1}</h1>
	<a class="border-solid border-2 rounded p-2 m-2 max-w-fit" href="/items">Main Page</a>
	<p>
		Use your mouse to select pairs of plagiarized spans via the line numbers, then click
		&quotCommit&quot. This page will save your commited spans automatically when you leave.
	</p>
	<p><span class="font-bold">LEFT:</span> {aItem?.path ?? 'NONE'}</p>
	<p><span class="font-bold">RIGHT:</span> {bItem?.path ?? 'NONE'}</p>
	<button
		on:click={(_) => commitSpans()}
		class="border-solid border-2 rounded p-2 m-2 max-w-fit {allowCommit
			? 'border-lime-500'
			: 'border-slate-50'}"
		disabled={!allowCommit}>Commit</button
	>
	<div class="flex flex-row min-h-0">
		{#if aItem !== null && bItem !== null && lang !== null}
			<CodeViewer bind:currentSpan={aCurrent} spans={aMarks} code={aItem.contents} {lang} />
			<CodeViewer bind:currentSpan={bCurrent} spans={bMarks} code={bItem.contents} {lang} />
		{/if}
	</div>
</div>
