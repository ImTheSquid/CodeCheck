<script lang="ts">
	export let href: string;
	export let enabled: boolean = true;
</script>

<a
	class="border-solid border-2 rounded p-2 m-2 max-w-fit {enabled ? 'hover:bg-gray-50' : ''}"
	href={enabled ? href : '#'}><slot /></a
>
