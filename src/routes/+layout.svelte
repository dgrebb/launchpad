<script lang="ts">
	import '$lib/app.pcss';
	import type { Snippet } from 'svelte';
	import Footer from '@components/ui/Footer/Footer.svelte';
	import Header from '@components/ui/Header/Header.svelte';
	import mockProjects from '$lib/../test/mocks/jira/user-projects.json';
	import { projectsState } from '@state';

	$effect(() => {
		projectsState.setProjects(mockProjects);
	});

	interface Props {
		children: Snippet;
	}

	let { children }: Props = $props();
</script>

<div class="flex min-h-screen w-full flex-col bg-muted/40">
	<Header />

	<div class="flex min-h-screen w-full flex-col bg-muted/40">
		<div class="flex flex-col sm:gap-4 sm:py-4 sm:pl-14">
			<main class="grid flex-1 items-start gap-4 p-4 sm:px-6 sm:py-0 md:gap-8">
				{#if children}
					{@render children()}
				{/if}
			</main>
		</div>
	</div>

	<Footer />
</div>
