<script lang="ts">
	import '../app.css';
	import Tab, { Icon, Label } from '@smui/tab';
	import TabBar from '@smui/tab-bar';

	// NOTE: ================= SMUI TAB BAR =========================

	/* function capitalize(str: String) {
		return str[0].toUpperCase() + str.slice(1);
	}
	let pages = ['', 'bindings', 'about']; */

	let tabs = [
		{
			icon: 'mail',
			label: 'Mail',
            path: '/',
		},
		{
			icon: 'calendar',
			label: 'Calendar',
            path: '/calendar'
		},
		{
			icon: 'favorite',
			label: 'Favorites',
            path: '/favorite'
		}
	];
	let active = tabs[0];

</script>

<div class="w-[100%] max-w-4xl mx-auto">
	<TabBar
		{tabs}
		let:tab
		bind:active
		style="display: flex; justify-content: center; align-items: center;"
	>
		<a href={tab.path}>
			<Tab {tab}>
				<Icon class="material-icons">{tab.icon}</Icon>
				<Label>{tab.label}</Label>
			</Tab>
		</a>
	</TabBar>
</div>
<hr />

<main class="w-[100%] max-w-4xl mx-auto mt-8">
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
</svelte:head>
