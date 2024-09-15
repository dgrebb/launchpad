<script lang="ts">
	// import { getUserProjects } from '@components/Jira/utils/API.jira';
	import { projectsState } from '@state';

	let projects = $state(projectsState.getProjects());

	$effect(() => {
		projects = projectsState.getProjects();
	});
</script>

<section class="dashboard">
	<JiraProjects {projects} />
</section>
