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
		}
	];
</script>

<h1>Mail</h1>

<!-- <input id="greet-input" placeholder="Enter website url..." bind:value={input_url} />
<FancyBtn text="Open window" , on_click={open_window} />
<p>Going to {new_url}</p> -->

{#each app_shortcuts as ap}
	<AppShortcut title={ap.title} icon={ap.icon} url={ap.url} />
{/each}

<!-- <AppShortcut
	title="Gmail"
	icon="https://upload.wikimedia.org/wikipedia/commons/7/7e/Gmail_icon_%282020%29.svg"
	url="https://mail.google.com/mail/u/0/#inbox"
/> -->

<style>
	input {
		width: 80%;
		padding: 0.5em;
		margin: 2em 0.3em;
		margin-bottom: 0;
	}
	* :global(AppShortcut) {
        display: inline-block;
		padding: 20em;
	}
</style>
