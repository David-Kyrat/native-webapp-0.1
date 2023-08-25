<!-- <script lang="ts">
  import Tab, { Icon, Label } from '@smui/tab';
  import TabBar from '@smui/tab-bar';
 
  let tabs = [
    {
      icon: 'access_time',
      label: 'Recents',
    },
    {
      icon: 'near_me',
      label: 'Nearby',
    },
    {
      icon: 'favorite',
      label: 'Favorites',
    },
  ];
  let active = tabs[0];
</script> -->

<script lang="ts">
	import '../app.css';
	import Tab, { Icon, Label } from '@smui/tab';
	import TabBar from '@smui/tab-bar';

	// NOTE: ================= SMUI TAB BAR =========================

	function capitalize(str) {
		return str[0].toUpperCase() + str.slice(1);
	}

	let pages = ['', 'bindings', 'about'];

	let tabs = [
		{
			icon: 'mail',
			label: 'Mail'
		},
		{
			icon: 'calendar',
			label: 'Calendar'
		},
		{
			icon: 'favorite',
			label: 'Favorites'
		}
	];
	let active = tabs[0];

	function get_path(tabLabel: String) {
		return './' + tabLabel.toLowerCase() + '/+page.svelte';
	}
</script>

<div>
	<TabBar {tabs} let:tab bind:active>
		<Tab {tab}>
			<Icon class="material-icons">{tab.icon}</Icon>
			<a href={get_path(tab.label)}> <Label>{tab.label}</Label></a>
		</Tab>
	</TabBar>
</div>

<main class="w-[90%] max-w-3xl mx-auto mt-24">
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
