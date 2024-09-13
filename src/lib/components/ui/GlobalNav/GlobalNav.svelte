<script>
	import * as Tooltip from '$lib/components/ui/tooltip';
	import ChartSpline from 'lucide-svelte/icons/chart-spline';
	import FolderKanban from 'lucide-svelte/icons/folder-kanban';
	import House from 'lucide-svelte/icons/house';
	import Package2 from 'lucide-svelte/icons/package-2';
	import Sailboat from 'lucide-svelte/icons/sailboat';
	import Settings from 'lucide-svelte/icons/settings';
</script>

<aside class="fixed inset-y-0 left-0 z-10 hidden w-14 flex-col border-r bg-background sm:flex">
	<nav class="flex flex-col items-center gap-4 px-2 py-4">
		<a
			href="/"
			class="group flex h-9 w-9 shrink-0 items-center justify-center gap-2 rounded-full bg-primary text-lg font-semibold text-primary-foreground md:h-8 md:w-8 md:text-base"
		>
			<Package2 class="h-4 w-4 transition-all group-hover:scale-110" />
			<span class="sr-only">dgrebb</span>
		</a>
		<Tooltip.Root>
			<Tooltip.Trigger asChild let:builder>
				<a
					href="/dashboard"
					class="flex h-9 w-9 items-center justify-center rounded-lg text-muted-foreground transition-colors hover:text-foreground md:h-8 md:w-8"
					use:builder.action
					{...builder}
				>
					<House class="h-5 w-5" />
					<span class="sr-only">Dashboard</span>
				</a>
			</Tooltip.Trigger>
			<Tooltip.Content side="right">Dashboard</Tooltip.Content>
		</Tooltip.Root>
		<Tooltip.Root>
			<Tooltip.Trigger asChild let:builder>
				<a
					href="/jira"
					class="flex h-9 w-9 items-center justify-center rounded-lg transition-colors hover:text-foreground md:h-8 md:w-8"
					use:builder.action
					{...builder}
				>
					<!-- svelte-ignore a11y_img_redundant_alt -->
					<img
						src="/img/jira-icon.svg"
						class="h-5 w-5"
						alt="Jira Icon; and series of arrows resembling page dog-ears or picture frame corners."
					/>
					<span class="sr-only">Jira</span>
				</a>
			</Tooltip.Trigger>
			<Tooltip.Content side="right">Jira</Tooltip.Content>
		</Tooltip.Root>
		<Tooltip.Root>
			<Tooltip.Trigger asChild let:builder>
				<a
					href="/jira/projects"
					class="flex h-9 w-9 items-center justify-center rounded-lg text-accent-foreground transition-colors hover:text-foreground md:h-8 md:w-8"
					use:builder.action
					{...builder}
				>
					<FolderKanban class="h-5 w-5" />
					<span class="sr-only">Projects</span>
				</a>
			</Tooltip.Trigger>
			<Tooltip.Content side="right">Projects</Tooltip.Content>
		</Tooltip.Root>
		<Tooltip.Root>
			<Tooltip.Trigger asChild let:builder>
				<a
					href="/jira/releases"
					class="flex h-9 w-9 items-center justify-center rounded-lg text-muted-foreground transition-colors hover:text-foreground md:h-8 md:w-8"
					use:builder.action
					{...builder}
				>
					<Sailboat class="h-5 w-5" />
					<span class="sr-only">Releases</span>
				</a>
			</Tooltip.Trigger>
			<Tooltip.Content side="right">Releases</Tooltip.Content>
		</Tooltip.Root>
		<Tooltip.Root>
			<Tooltip.Trigger asChild let:builder>
				<a
					href="/jira/reports"
					class="flex h-9 w-9 items-center justify-center rounded-lg text-muted-foreground transition-colors hover:text-foreground md:h-8 md:w-8"
					use:builder.action
					{...builder}
				>
					<ChartSpline class="h-5 w-5" />
					<span class="sr-only">Reports</span>
				</a>
			</Tooltip.Trigger>
			<Tooltip.Content side="right">Reports</Tooltip.Content>
		</Tooltip.Root>
	</nav>
	<nav class="mt-auto flex flex-col items-center gap-4 px-2 py-4">
		<Tooltip.Root>
			<Tooltip.Trigger asChild let:builder>
				<a
					href="/settings"
					class="flex h-9 w-9 items-center justify-center rounded-lg text-muted-foreground transition-colors hover:text-foreground md:h-8 md:w-8"
					use:builder.action
					{...builder}
				>
					<Settings class="h-5 w-5" />
					<span class="sr-only">Settings</span>
				</a>
			</Tooltip.Trigger>
			<Tooltip.Content side="right">Settings</Tooltip.Content>
		</Tooltip.Root>
	</nav>
</aside>
