<script lang="ts">
	import type { Projects } from '@types';
	import Ellipsis from 'lucide-svelte/icons/ellipsis';
	import { Badge } from '$lib/components/ui/badge';
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import * as Table from '$lib/components/ui/table';
	import * as Tabs from '$lib/components/ui/tabs';

	interface Props {
		projects: Projects;
	}

	const { projects }: Props = $props();
	console.log('🚀 ~ projects:', projects);
</script>

<Tabs.Root value="all">
	<div class="flex items-center">
		<Tabs.List>
			<Tabs.Trigger value="all">All</Tabs.Trigger>
			<Tabs.Trigger value="active">Visible</Tabs.Trigger>
			<Tabs.Trigger value="draft">Hidden</Tabs.Trigger>
		</Tabs.List>
	</div>
	<Tabs.Content value="all">
		<Card.Root>
			<Card.Header>
				<Card.Title>Jira Project Visibility</Card.Title>
				<Card.Description
					>Control which projects display in specific LaunchPad sections.</Card.Description
				>
			</Card.Header>
			<Card.Content>
				<Table.Root>
					<Table.Header>
						<Table.Row>
							<Table.Head class="hidden w-[100px] sm:table-cell">
								<span class="sr-only">Image</span>
							</Table.Head>
							<Table.Head>Name</Table.Head>
							<Table.Head>Status</Table.Head>
							<Table.Head>Price</Table.Head>
							<Table.Head class="hidden md:table-cell">Total Sales</Table.Head>
							<Table.Head class="hidden md:table-cell">Created at</Table.Head>
							<Table.Head>
								<span class="sr-only">Actions</span>
							</Table.Head>
						</Table.Row>
					</Table.Header>
					<Table.Body>
						{#if projects.length > 0}
							{#each projects as project}
								<h2>{project.name}</h2>
								<h3><code class="">{project.key}</code></h3>
								<Table.Row>
									<Table.Cell class="hidden sm:table-cell">
										<img
											alt="Product example"
											class="aspect-square rounded-md object-cover"
											height="64"
											src="/img/project-avatar-placeholder.svg"
											width="64"
										/>
									</Table.Cell>
									<Table.Cell class="font-medium">Laser Lemonade Machine</Table.Cell>
									<Table.Cell>
										<Badge variant="outline">Draft</Badge>
									</Table.Cell>
									<Table.Cell>$499.99</Table.Cell>
									<Table.Cell class="hidden md:table-cell">25</Table.Cell>
									<Table.Cell class="hidden md:table-cell">2023-07-12 10:42 AM</Table.Cell>
									<Table.Cell>
										<DropdownMenu.Root>
											<DropdownMenu.Trigger asChild let:builder>
												<Button
													aria-haspopup="true"
													size="icon"
													variant="ghost"
													builders={[builder]}
												>
													<Ellipsis class="h-4 w-4" />
													<span class="sr-only">Toggle menu</span>
												</Button>
											</DropdownMenu.Trigger>
											<DropdownMenu.Content align="end">
												<DropdownMenu.Label>Actions</DropdownMenu.Label>
												<DropdownMenu.Item>Edit</DropdownMenu.Item>
												<DropdownMenu.Item>Delete</DropdownMenu.Item>
											</DropdownMenu.Content>
										</DropdownMenu.Root>
									</Table.Cell>
								</Table.Row>
							{/each}
						{/if}
					</Table.Body>
				</Table.Root>
			</Card.Content>
			<Card.Footer>
				<div class="text-xs text-muted-foreground">
					Showing <strong>1-10</strong> of <strong>32</strong> products
				</div>
			</Card.Footer>
		</Card.Root>
	</Tabs.Content>
</Tabs.Root>
