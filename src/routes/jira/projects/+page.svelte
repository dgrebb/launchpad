<script lang="ts">
	import JiraProjects from '@components/Jira/Projects/JiraProjects.svelte';

	// import { getUserProjects } from '@components/Jira/utils/API.jira';
	import { projectsState } from '@state';

	let projects = $state(projectsState.getProjects());

	$effect(() => {
		projects = projectsState.getProjects();
	});
</script>

<JiraProjects {projects} />
