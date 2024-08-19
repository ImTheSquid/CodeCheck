<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { numItems } from '$lib';
	import CodeViewer from '$lib/components/CodeViewer.svelte';
	import Link from '$lib/components/Link.svelte';
	import Button from '$lib/components/Button.svelte';
	import { type MarkSpan, type Mark, HslGenerator, type PairData, type Item } from '$lib/index';
	import { page } from '$app/stores';
	let slug: number = parseInt($page.url.searchParams.get('i') ?? '0');

	let aCurrent: MarkSpan | null = null;
	let bCurrent: MarkSpan | null = null;
	let generator: HslGenerator | null = null;
	let marks: Mark[] = [];
	let aItem: Item | null = null;
	let bItem: Item | null = null;
	let lang: string | null = null;
	let itemCount: number = 0;

	const unsub = numItems.subscribe((n: number) => {
		itemCount = n;
	});

	$: allowCommit = !(aCurrent === null || bCurrent === null);
	$: aMarks = marks.map((m) => ({ span: m.a, color: m.color }));
	$: bMarks = marks.map((m) => ({ span: m.b, color: m.color }));

	async function handleChange() {
		generator = new HslGenerator();
		const pairData: PairData = await invoke('load_pair', { pairIndex: slug });
		aItem = pairData.a;
		bItem = pairData.b;
		marks = pairData.marks;
		lang = pairData.lang;
		for (let mark of marks) {
			mark.color = generator.generateColor();
		}
	}

	onMount(async () => {
		await handleChange();
	});

	onDestroy(() => {
		unsub();
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
			invoke('set_spans', {
				pairIndex: slug,
				marks: marks
			});
		}

		aCurrent = null;
		bCurrent = null;
	}

	function removeSpan(i: number) {
		if (marks.length === 1) {
			marks = [];
		} else {
			marks = marks.filter((_, idx) => idx !== i);
		}
		invoke('set_spans', { pairIndex: slug, marks: marks });
	}
</script>

<div class="flex flex-col h-screen">
	<h1 class="text-xl font-bold">Comparison {slug + 1}</h1>
	<div class="flex">
		<Link href="/items">Main Page</Link>
		<Button
			on:click={(_) => {
				slug -= 1;
				// document.location.href = `/items/view?i=${slug}`;
				handleChange();

				$page.url.searchParams.set('i', `${slug}`);
			}}
			enabled={slug > 0}>Prev</Button
		>
		<Button
			on:click={(_) => {
				slug += 1;
				// document.location.href = `/items/view?i=${slug}`;
				$page.url.searchParams.set('i', `${slug}`);
				handleChange();
			}}
			enabled={slug < itemCount - 1}>Next</Button
		>
	</div>
	<p>
		Use your mouse to select pairs of plagiarized spans via the line numbers, then click
		&quotCommit&quot. This page will save your commited spans automatically when you leave.
	</p>
	<p><span class="font-bold">LEFT:</span> {aItem?.path ?? 'NONE'}</p>
	<p><span class="font-bold">RIGHT:</span> {bItem?.path ?? 'NONE'}</p>
	<div class="flex">
		<Button
			on:click={(_) => commitSpans()}
			border={allowCommit ? 'border-lime-500' : null}
			enabled={allowCommit}>Commit</Button
		>
		<div class="flex overflow-x-scroll">
			{#each marks as mark, i (mark.color)}
				<Button on:click={(_) => removeSpan(i)}>
					<p style="color: {mark.color};">Remove Span {i}</p>
				</Button>
			{/each}
		</div>
	</div>
	<div class="flex flex-row min-h-0">
		{#if aItem !== null && bItem !== null && lang !== null}
			<CodeViewer bind:currentSpan={aCurrent} spans={aMarks} code={aItem.contents} {lang} />
			<CodeViewer bind:currentSpan={bCurrent} spans={bMarks} code={bItem.contents} {lang} />
		{/if}
	</div>
</div>
