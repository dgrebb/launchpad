<script lang="ts">
	import '$lib/styles/app.css';
	import type { Snippet } from 'svelte';
	import { ModeWatcher, toggleMode, mode } from 'mode-watcher';
	import { Button } from '$lib/components/ui/button/index.js';
	import Icon from '@iconify/svelte';

	interface Props {
		children: Snippet;
	}

	let { children }: Props = $props();
</script>

<header>
	<h1>LaunchPad</h1>
	<ModeWatcher />
	<Button on:click={toggleMode} variant="outline" size="icon">
		<Icon
			icon={$mode === 'light' ? 'mingcute:light-fill' : 'mingcute:light-line'}
			height={33}
			width={33}
		/>
		<span class="sr-only">Toggle theme</span>
	</Button>
</header>
<main>
	{#if children}
		{@render children()}
	{/if}
</main>
