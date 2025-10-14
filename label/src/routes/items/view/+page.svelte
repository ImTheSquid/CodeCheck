<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { numItems } from '$lib';
	import CodeViewer from '$lib/components/CodeViewer.svelte';
	import Link from '$lib/components/Link.svelte';
	import Button from '$lib/components/Button.svelte';
	import { type MarkSpan, type Mark, HslGenerator, type PairData, type Item } from '$lib/index';
	import { page } from '$app/stores';
	let slug: number = $state(parseInt($page.url.searchParams.get('i') ?? '0'));

	let aCurrent: MarkSpan | null = $state(null);
	let bCurrent: MarkSpan | null = $state(null);
	let generator: HslGenerator | null = null;
	let marks: Mark[] = $state([]);
	let aItem: Item | null = $state(null);
	let bItem: Item | null = $state(null);
	let lang: string | null = $state(null);
	let itemCount: number = $state(0);
	let chord = $state('');

	const unsub = numItems.subscribe((n: number) => {
		itemCount = n;
	});

	let allowCommit = $derived(!(aCurrent === null || bCurrent === null));
	let aMarks = $derived(marks.map((m) => ({ span: m.a, color: m.color })));
	let bMarks = $derived(marks.map((m) => ({ span: m.b, color: m.color })));

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

	function checkSingleRange(s: string, checkLeft: boolean): boolean {
		let res = s.split('-');
		if (res.length != 2) {
			return false;
		}
		let lower = parseInt(res[0]);
		let upper = parseInt(res[1]);
		return (
			lower >= 1 &&
			lower <= upper &&
			upper <
				((checkLeft ? aItem?.contents.split('\n').length : bItem?.contents.split('\n').length) ??
					-1) +
					1
		);
	}

	function chordHasValidRange(checkTwo: boolean = false): boolean {
		if (checkTwo) {
			let res = chord.split(/[LR]/);
			if (res.length != 2) {
				return false;
			}
			return checkSingleRange(res[0], true) && checkSingleRange(res[1], false);
		} else {
			return checkSingleRange(chord, true);
		}
	}

	function onkeydown(ev: KeyboardEvent) {
		if (ev.code == 'KeyN' && slug < itemCount - 1) {
			nextPage();
		} else if (ev.code == 'KeyP' && slug > 0) {
			prevPage();
		} else if (ev.code == 'Escape') {
			aCurrent = null;
			bCurrent = null;
			chord = '';
		} else if (ev.key >= '0' && ev.key <= '9') {
			console.log('Number key pressed:', ev.key);
			chord += ev.key;
		} else if (ev.code == 'Backspace' && chord.length > 0) {
			ev.preventDefault();
			chord = chord.slice(0, -1);
		} else if (ev.code == 'Minus') {
			chord += '-';
		} else if (ev.code == 'KeyL' && chordHasValidRange(chord.includes('R'))) {
			chord += 'L';
		} else if (ev.code == 'KeyR' && chordHasValidRange(chord.includes('L'))) {
			chord += 'R';
		}

		if (chordHasValidRange(true)) {
			function parseLines(s: string): number[] {
				let res = s.split('-');
				return [parseInt(res[0]), parseInt(res[1])];
			}

			let res = chord.split(/[LR]/);
			let left = parseLines(res[0]);
			let right = parseLines(res[1]);
			// If R is actually the first set, swap
			if (chord.includes('R')) {
				let tmp = left;
				left = right;
				right = tmp;
			}
			aCurrent = { start: left[0], end: left[1] };
			bCurrent = { start: right[0], end: right[1] };
			commitSpans();
			chord = '';
		}
	}

	function prevPage() {
		slug -= 1;
		// document.location.href = `/items/view?i=${slug}`;
		handleChange();

		$page.url.searchParams.set('i', `${slug}`);
	}

	function nextPage() {
		slug += 1;
		// document.location.href = `/items/view?i=${slug}`;
		$page.url.searchParams.set('i', `${slug}`);
		handleChange();
	}
</script>

<svelte:window {onkeydown} />

<div class="flex flex-col h-screen">
	<h1 class="text-xl font-bold">Comparison {slug + 1}</h1>
	<div class="flex">
		<Link href="/items">Main Page</Link>
		<Button
			on:click={(_) => {
				prevPage();
			}}
			enabled={slug > 0}>Prev</Button
		>
		<Button
			on:click={(_) => {
				nextPage();
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
	<div class="flex flex-row">
		<span class="mr-1">Current Chord:</span>
		{#if chord.length == 0}
			<p class="text-slate-500">None</p>
		{:else}
			<code>{chord}</code>
		{/if}
	</div>
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
