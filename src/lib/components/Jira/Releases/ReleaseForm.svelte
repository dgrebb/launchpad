<script lang="ts">
	import { Calendar } from '$lib/components/ui/calendar';
	import * as Card from '$lib/components/ui/card';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import * as Popover from '$lib/components/ui/popover';
	import { Textarea } from '$lib/components/ui/textarea';
	import { cn } from '$lib/utils';
	import { Button } from '@components/ui/button';
	import { DateFormatter, type DateValue, getLocalTimeZone } from '@internationalized/date';
	import { invoke } from '@tauri-apps/api/tauri';
	import CalendarIcon from 'svelte-radix/Calendar.svelte';
	import type { JiraVersionPostData } from './releases.types';
	import { formatDateForJira } from '../jira.utils';
	import type { ProjectWithSelection } from '@types';
	import { Badge } from '@components/ui/badge';

	interface Props {
		toggleReleaseForm: () => void;
		selectedProjects?: ProjectWithSelection[];
	}

	const df = new DateFormatter('en-US', {
		dateStyle: 'long'
	});

	const versionNamePlaceholder = `R${new Date().toISOString().slice(2, 10).replace(/-/g, '.')}`;

	let { toggleReleaseForm, selectedProjects }: Props = $props();
	let projects = $state(selectedProjects);

	let description: string | null | string[] | number = $state('');
	let name: string = $state(versionNamePlaceholder);
	let projectId: number = $state(10000);
	let released: boolean = $state(false);
	let archived: boolean = $state(false);
	let startDate: DateValue | undefined = $state(undefined);
	let releaseDate: DateValue | undefined = $state(undefined);

	// Example data for creating a Jira version
	let versionData: JiraVersionPostData = $derived({
		archived,
		description,
		name,
		projectId,
		released,
		releaseDate,
		startDate
	});

	const createJiraVersions = async (versionData: JiraVersionPostData) => {
		// Call the Rust function
		await invoke('create_jira_version', { versionData })
			.then((response) => {
				console.log('Version Created:', response);
			})
			.catch((error) => {
				console.error('Error creating version:', error);
			});
	};

	const handleSubmit = async (event: SubmitEvent) => {
		event.preventDefault();
		versionData.releaseDate = formatDateForJira(releaseDate);
		versionData.startDate = formatDateForJira(startDate);

		// console.log('🚀 ~ createJiraVersions ~ versionData:', versionData, description);
		await createJiraVersions(versionData as JiraVersionPostData);
	};

	$effect(() => {
		projects = selectedProjects;
	});
</script>

<Card.Root class="2xl:grid-item flex w-full flex-col">
	<Card.Header class="p-2 pt-0 md:p-4">
		<Card.Title>New Version</Card.Title>
	</Card.Header>
	<form class="" onsubmit={(e) => handleSubmit(e)}>
		<Card.Content class="relative flex flex-1 flex-col p-2 pt-0 md:p-4 md:pt-0">
			<fieldset class="flex w-full flex-col rounded-lg border p-4">
				<legend class="-ml-1 px-1 text-sm font-medium">Version Details</legend>
				<div class="flex flex-row justify-between gap-3 p-3 pl-0">
					<Label for="version-name" class="w-[33%] text-left">Name</Label>
					<Input
						id="version-name"
						bind:value={versionData['name']}
						type="text"
						placeholder="24.09.15.0"
					/>
				</div>
				<div class="flex flex-row justify-between p-3 pl-0">
					<Label for="version-description" class="w-[33%] text-left">Description</Label>
					<Textarea
						id="version-description"
						placeholder="This release includes..."
						class="min-h-[9rem]"
						bind:value={description}
					/>
				</div>
			</fieldset>
			<fieldset class="mt-4 flex w-full flex-col rounded-lg border p-4">
				<legend class="-ml-1 px-1 text-sm font-medium">Version Release Dates</legend>
				<div class="gap-3">
					<Label for="version-start">Release Start Date</Label>
					<Popover.Root>
						<Popover.Trigger asChild let:builder>
							<Button
								variant="outline"
								class={cn(
									'w-[240px] justify-start text-left font-normal',
									!startDate && 'text-muted-foreground'
								)}
								builders={[builder]}
							>
								<CalendarIcon class="mr-2 h-4 w-4" />
								{startDate ? df.format(startDate.toDate(getLocalTimeZone())) : 'Pick a date'}
							</Button>
						</Popover.Trigger>
						<Popover.Content class="w-auto p-0" align="start">
							<Calendar bind:value={startDate} />
						</Popover.Content>
					</Popover.Root>
					<Label for="version-start">Release Date</Label>
					<Popover.Root>
						<Popover.Trigger asChild let:builder>
							<Button
								variant="outline"
								class={cn(
									'w-[240px] justify-start text-left font-normal',
									!releaseDate && 'text-muted-foreground'
								)}
								builders={[builder]}
							>
								<CalendarIcon class="mr-2 h-4 w-4" />
								{releaseDate ? df.format(releaseDate.toDate(getLocalTimeZone())) : 'Pick a date'}
							</Button>
						</Popover.Trigger>
						<Popover.Content class="w-auto p-0" align="start">
							<Calendar bind:value={releaseDate} />
						</Popover.Content>
					</Popover.Root>
				</div>
			</fieldset>
		</Card.Content>
		{#if projects && projects.length > 0}
			{#key projects}
				<Card.Content class="relative flex flex-1 border-t p-2 md:p-4">
					{#each projects as project}
						{#if project.selected === true}<Badge>{project.key}</Badge>{/if}
					{/each}
				</Card.Content>
			{/key}
		{/if}
		<Card.Footer class="justify-end border-t px-6 py-4">
			<Button size="sm" class="ml-2" onclick={toggleReleaseForm}>Cancel</Button>
			<Button size="sm" class="ml-2" type="submit">Create Versions</Button>
		</Card.Footer>
	</form>
</Card.Root>

<style>
	/* your styles go here */
</style>
