<script lang="ts">
	// import { getUserProjects } from '@components/Jira/utils/API.jira';
	import { projectsState } from '@state';
	import mockProjects from '$lib/../test/mocks/jira/user-projects.json';
	import JiraProjects from '@components/Jira/Projects/JiraProjects.svelte';

	let projects = $state(projectsState.getProjects());

	$effect(() => {
		projectsState.setProjects(mockProjects);
	});

	$effect(() => {
		projects = projectsState.getProjects();
	});
</script>

<section class="dashboard">
	<JiraProjects {projects} />
</section>
