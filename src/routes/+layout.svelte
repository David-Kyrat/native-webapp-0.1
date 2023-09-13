<script lang="ts">
	import '../app.css';
	import Tab, { Icon, Label } from '@smui/tab';
	import TabBar from '@smui/tab-bar';

	// NOTE: ================= SMUI TAB BAR =========================

	let tabs = [
		{
			icon: 'mail',
			label: 'Mail',
			path: '/'
		},
		{
			icon: 'light_mode',
			label: '  Clendar',
			path: '/calendar'
		},
		{
			icon: 'favorite',
			label: 'Favorites',
			path: '/favorite'
		},
		{
			icon: 'settings',
			label: 'Tests',
			path: '/test'
		}
	];
	let active = tabs[0];
</script>

<div class="w-[100%] max-w-5xl mx-auto">
	<TabBar
		{tabs}
		let:tab
		bind:active
		style="display: flex; justify-content: center; align-items: center;"
	>
		<Tab {tab} href={tab.path}>
			{#if tab.label.includes('Calendar')}
				<i class="nf nf-md-calendar pr-2" />
			{:else}
				<Icon class="material-icons">{tab.icon}</Icon>
			{/if}
			<Label>{tab.label}</Label>
		</Tab>
	</TabBar>
</div>
<hr />

<main class="w-[100%] max-w-5xl mx-auto mt-8">
	<div class="prose-lg prose-img:rounded-xl prose-a:text-blue-600">
		<slot />
	</div>
</main>

<svelte:head>
	<!-- Fonts -->
	<link rel="stylesheet" href="https://fonts.googleapis.com/icon?family=Material+Icons" />
	<link
		rel="stylesheet"
		href="https://fonts.googleapis.com/css?family=Roboto:300,400,500,600,700"
	/>

	<!-- Material Typography -->
	<link
		rel="stylesheet"
		href="https://unpkg.com/@material/typography@14.0.0/dist/mdc.typography.css"
	/>

	<!-- SMUI -->
	<link rel="stylesheet" href="https://unpkg.com/svelte-material-ui/bare.css" />

	<!-- NerdFonts -->
	<link rel="stylesheet" href="https://www.nerdfonts.com/assets/css/webfont.css" />
</svelte:head>
