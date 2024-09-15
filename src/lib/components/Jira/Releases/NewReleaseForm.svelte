<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { Textarea } from '$lib/components/ui/textarea';
	import { Button } from '@components/ui/button';
	import type { JiraVersionPostData } from './releases.types';
	import { invoke } from '@tauri-apps/api/tauri';

	interface Props {
		toggleReleaseForm: () => void;
	}

	let { toggleReleaseForm }: Props = $props();

	// Example data for creating a Jira version
	let versionData = $state({
		archived: false,
		description: 'New version description',
		name: 'Version 1.0',
		projectId: 10000,
		released: false
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
</script>

<form class="w-full items-start" onsubmit={() => createJiraVersions(versionData)}>
	<fieldset class="rounded-lg border p-4">
		<legend class="-ml-1 px-1 text-sm font-medium"> Release Details </legend>
		<div class="gap-3">
			<Label for="version-name">Name</Label>
			<Input id="version-name" type="text" placeholder="24.09.15.0" />
		</div>
		<div class="gap-3">
			<Label for="version-description">Description</Label>
			<Label for="content">Content</Label>
			<Textarea id="content" placeholder="This release includes..." class="min-h-[9rem]" />
		</div>
	</fieldset>
	<Button size="sm" class="" onclick={toggleReleaseForm}>Cancel</Button>
	<Button size="sm" class="" type="submit">Create Versions</Button>
</form>

<style>
	/* your styles go here */
</style>
