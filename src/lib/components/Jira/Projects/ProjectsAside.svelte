<script lang="ts">
	import { Checkbox } from '$lib/components/ui/checkbox';
	import { Label } from '$lib/components/ui/label';
	import { Button } from '@components/ui/button';
	import type { ProjectWithSelection } from '@types';
	import Bell from 'lucide-svelte/icons/bell';
	import Package2 from 'lucide-svelte/icons/package-2';

	interface Props {
		projects: ProjectWithSelection[];
	}

	let { projects }: Props = $props();
</script>

<div class="hidden border-r bg-muted/40 md:block">
	<div class="flex h-full max-h-screen flex-col gap-2">
		<div class="flex h-14 items-center border-b px-4 lg:h-[60px] lg:px-6">
			<a href="/" class="flex items-center gap-2 font-semibold">
				<Package2 class="h-6 w-6" />
				<span class="">Jira Projects</span>
			</a>
			<Button variant="outline" size="icon" class="ml-auto h-8 w-8">
				<Bell class="h-4 w-4" />
				<span class="sr-only">Toggle notifications</span>
			</Button>
		</div>
		<div class="flex-1">
			<nav class="grid items-start px-2 text-sm font-medium lg:px-4">
				{#each projects as project, index}
					<Checkbox id={project.key} bind:checked={projects[index].selected} />
					<Label
						for={project.key}
						class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
					>
						{project.name}
					</Label>
				{/each}
			</nav>
		</div>
	</div>
</div>

<style>
	/* your styles go here */
</style>
