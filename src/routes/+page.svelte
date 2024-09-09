<script lang="ts">
	import { getUserProjects } from '$lib/utils/API.jira';
	import { projectsState } from '@state';
	let projects = $state(projectsState.getProjects());

	$effect(() => {
		return async () => {
			// const projects = await getUserProjects();
			// console.log('🚀 ~ return ~ projects:', projects);
			// projectsState.setProjects([
			// 	{
			// 		name: 'Unbelievable Project',
			// 		key: 'UBP'
			// 	}
			// ]);
		};
	});
</script>

<section class="dashboard">
	<h1>Dashboard</h1>
	{#if projects.length > 0}
		{#each projects as project}
			<h2>{project.name}</h2>
			<h3><code>{project.key}</code></h3>
		{/each}
	{/if}
</section>
