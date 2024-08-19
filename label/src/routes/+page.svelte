<script lang="ts">
	import { open } from '@tauri-apps/plugin-dialog';
	import { invoke } from '@tauri-apps/api/core';
	import Error from '$lib/components/Error.svelte';
	import Button from '$lib/components/Button.svelte';
	let err: string | null = null;
	const loadDirectory = async () => {
		const res = await open({
			directory: true,
			title: 'Select a Dataset Directory'
		});

		if (res === null) {
			return;
		}

		try {
			await invoke('validate_directory', { path: res });
			window.location.href = '/items';
		} catch (error) {
			err = error;
		}
	};
</script>

<h1 class="text-xl font-bold">CodeCheck Labelling System</h1>
<p>Load a dataset directory to get started.</p>
<Button on:click={(_) => loadDirectory()}>Load Directory</Button>

<Error error={err} />
