<script lang="ts">
	import '$lib/app.pcss';
	import type { Snippet } from 'svelte';
	import Footer from '@components/ui/Footer/Footer.svelte';
	import Header from '@components/ui/Header/Header.svelte';

	interface Props {
		children: Snippet;
	}

	let { children }: Props = $props();
</script>

<Header />
<main>
	{#if children}
		{@render children()}
	{/if}
</main>
<Footer />
