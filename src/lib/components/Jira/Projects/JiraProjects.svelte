<script lang="ts">
	import type { Projects } from '@types';

	interface Props {
		projects: Projects;
	}

	const { projects }: Props = $props();
</script>

<h1>Jira Projects</h1>
{#if projects.length > 0}
	{#each projects as project}
		<h2>{project.name}</h2>
		<h3><code class="">{project.key}</code></h3>
	{/each}
{/if}

<style>
	/* your styles go here */
</style>
