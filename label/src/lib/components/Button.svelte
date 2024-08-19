<script lang="ts">
	export let enabled: boolean = true;
	export let border: string | null = null;
</script>

<button
	on:click
	class="border-solid border-2 rounded p-2 m-2 max-w-fit {border ?? ''} {enabled
		? 'hover:bg-gray-50'
		: ''}"
	disabled={!enabled}><slot /></button
>
