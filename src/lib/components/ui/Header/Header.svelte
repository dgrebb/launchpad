<script>
	import { ModeWatcher, toggleMode, mode } from 'mode-watcher';
	import { Button } from '$lib/components/ui/button';
	import Icon from '@iconify/svelte';
	import GlobalNav from '../GlobalNav/GlobalNav.svelte';
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
	<GlobalNav />
</header>

<style>
	/* your styles go here */
</style>
