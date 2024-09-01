<script>
	import '../app.css';
	import '$lib/styles/global.pcss';
	import type { Snippet } from 'svelte';

	interface Props {
		children: Snippet;
	}

	let { children }: Props = $props();
</script>

<main>
	{#if children}
		{@render children()}
	{/if}
</main>
