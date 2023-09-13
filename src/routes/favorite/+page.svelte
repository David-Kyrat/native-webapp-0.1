<script>
	import AppShortcut from '$lib/components/AppShortcut.svelte';

	let app_shortcuts = [
		{
			title: 'Gmail',
			icon: 'https://upload.wikimedia.org/wikipedia/commons/7/7e/Gmail_icon_%282020%29.svg',
			url: 'https://mail.google.com/mail/u/0/#inbox'
		},
		{
			title: 'ProtonMail',
			icon: 'https://www.privacyguides.org/en/assets/img/email/protonmail.svg',
			url: 'https://mail.proton.me/u/6/inbox'
		},
		{
			title: 'ProtonMail',
			icon: 'https://www.privacyguides.org/en/assets/img/email/protonmail.svg',
			url: 'https://mail.proton.me/u/6/inbox'
		},
		{
			title: 'ProtonMail',
			icon: 'https://www.privacyguides.org/en/assets/img/email/protonmail.svg',
			url: 'https://mail.proton.me/u/6/inbox'
		},
		{
			title: 'ProtonMail',
			icon: 'https://www.privacyguides.org/en/assets/img/email/protonmail.svg',
			url: 'https://mail.proton.me/u/6/inbox'
		},
		{
			title: 'ProtonMail',
			icon: 'https://www.privacyguides.org/en/assets/img/email/protonmail.svg',
			url: 'https://mail.proton.me/u/6/inbox'
		}
	];
	// $: app_nb = app_shortcuts.length;
	$: col_nb = 4;
	let grid_cols = "grid-cols-3";
	$: grid_cols = `grid-cols-${col_nb}`;
	let gap = `gap-8`;
	gap = `gap-${col_nb * 2}`;
</script>

<h1>Mail</h1>

<!-- App shortcut grid -->
<!-- <div class="flex flex-row flex-wrap justify-between content-between mx-3 max-w-5xl"> -->
<div class="grid grid-cols-3 gap-6 mx-5 place-content-evenly max-w-xl">
	<!-- <div class="grid {app_nb2} gap-{app_nb}"> -->
	{#each app_shortcuts as ap}
		<AppShortcut title={ap.title} icon={ap.icon} url={ap.url} styleclass="" />
	{/each}
</div>

<hr />

<style>
	/*input {
		width: 80%;
		padding: 0.5em;
		margin: 2em 0.3em;
		margin-bottom: 0;
	}*/
	* :global(AppShortcut) {
		display: inline-block;
		padding: 20em;
	}
</style>
