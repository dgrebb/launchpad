<script>
	import * as Breadcrumb from '$lib/components/ui/breadcrumb';
	import { Button } from '$lib/components/ui/button';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { Input } from '$lib/components/ui/input';
	import * as Sheet from '$lib/components/ui/sheet';
	import ChartSpline from 'lucide-svelte/icons/chart-spline';
	import House from 'lucide-svelte/icons/house';
	import Moon from 'lucide-svelte/icons/moon';
	import Package from 'lucide-svelte/icons/package';
	import Package2 from 'lucide-svelte/icons/package-2';
	import PanelLeft from 'lucide-svelte/icons/panel-left';
	import Search from 'lucide-svelte/icons/search';
	import ShoppingCart from 'lucide-svelte/icons/shopping-cart';
	import Sun from 'lucide-svelte/icons/sun';
	import UsersRound from 'lucide-svelte/icons/users-round';
	import { mode, ModeWatcher, toggleMode } from 'mode-watcher';
	import GlobalNav from '../GlobalNav/GlobalNav.svelte';
</script>

<GlobalNav />
<div class="flex flex-col sm:gap-4 sm:py-4 sm:pl-14">
	<header
		class="sticky top-0 z-30 flex h-14 items-center gap-4 border-b bg-background px-4 sm:static sm:h-auto sm:border-0 sm:bg-transparent sm:px-6"
	>
		<ModeWatcher />
		<Sheet.Root>
			<Sheet.Trigger asChild let:builder>
				<Button builders={[builder]} size="icon" variant="outline" class="sm:hidden">
					<PanelLeft class="h-5 w-5" />
					<span class="sr-only">Toggle Menu</span>
				</Button>
			</Sheet.Trigger>
			<Sheet.Content side="left" class="sm:max-w-xs">
				<nav class="grid gap-6 text-lg font-medium">
					<a
						href="/"
						class="group flex h-10 w-10 shrink-0 items-center justify-center gap-2 rounded-full bg-primary text-lg font-semibold text-primary-foreground md:text-base"
					>
						<Package2 class="h-5 w-5 transition-all group-hover:scale-110" />
						<span class="sr-only">dgrebb</span>
					</a>
					<a
						href="/dashboard"
						class="flex items-center gap-4 px-2.5 text-muted-foreground hover:text-foreground"
					>
						<House class="h-5 w-5" />
						Dashboard
					</a>
					<a
						href="/jira"
						class="flex items-center gap-4 px-2.5 text-muted-foreground hover:text-foreground"
					>
						<ShoppingCart class="h-5 w-5" />
						Jira
					</a>
					<a href="/jira/projects" class="flex items-center gap-4 px-2.5 text-foreground">
						<Package class="h-5 w-5" />
						Projects
					</a>
					<a
						href="/jira/projects/UIDEV"
						class="flex items-center gap-4 px-2.5 text-muted-foreground hover:text-foreground"
					>
						<UsersRound class="h-5 w-5" />
						UI Development
					</a>
					<a
						href="/jira/projects/UIDEV/releases"
						class="flex items-center gap-4 px-2.5 text-muted-foreground hover:text-foreground"
					>
						<ChartSpline class="h-5 w-5" />
						Releases
					</a>
				</nav>
			</Sheet.Content>
		</Sheet.Root>
		<Breadcrumb.Root class="hidden md:flex">
			<Breadcrumb.List>
				<Breadcrumb.Item>
					<Breadcrumb.Link href="/dashboard">Dashboard</Breadcrumb.Link>
				</Breadcrumb.Item>
				<Breadcrumb.Separator />
				<Breadcrumb.Item>
					<Breadcrumb.Link href="/jira/projects">Projects</Breadcrumb.Link>
				</Breadcrumb.Item>
				<Breadcrumb.Separator />
				<Breadcrumb.Item>
					<Breadcrumb.Page>All Projects</Breadcrumb.Page>
				</Breadcrumb.Item>
			</Breadcrumb.List>
		</Breadcrumb.Root>
		<div class="relative ml-auto flex-1 md:grow-0">
			<Search class="absolute left-2.5 top-2.5 h-4 w-4 text-muted-foreground" />
			<Input
				type="search"
				placeholder="Search..."
				class="w-full rounded-lg bg-background pl-8 md:w-[200px] lg:w-[320px]"
			/>
		</div>
		<DropdownMenu.Root>
			<DropdownMenu.Trigger asChild let:builder>
				<Button
					builders={[builder]}
					variant="outline"
					size="icon"
					class="overflow-hidden rounded-full"
				>
					<img
						src="/images/placeholder-user.jpg"
						width={36}
						height={36}
						alt="Avatar"
						class="overflow-hidden rounded-full"
					/>
				</Button>
			</DropdownMenu.Trigger>
			<DropdownMenu.Content align="end">
				<DropdownMenu.Label>My Account</DropdownMenu.Label>
				<DropdownMenu.Separator />
				<DropdownMenu.Item>Settings</DropdownMenu.Item>
				<DropdownMenu.Item>Support</DropdownMenu.Item>
				<DropdownMenu.Separator />
				<DropdownMenu.Item>Logout</DropdownMenu.Item>
			</DropdownMenu.Content>
		</DropdownMenu.Root>
		<Button on:click={toggleMode} variant="outline" size="icon">
			{#if $mode === 'light'}
				<Moon class="h-5 w-5" />
			{:else}
				<Sun class="h-5 w-5" />
			{/if}
			<span class="sr-only">Toggle theme</span>
		</Button>
	</header>
</div>
