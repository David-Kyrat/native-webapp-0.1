<script>
	import FancyBtn from '$lib/components/FancyBtn.svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import AppShortcut from '$lib/components/AppShortcut.svelte';

	let clicked = 0;

	let input_url;
	$: new_url = input_url && input_url.length > 0 ? input_url : 'https://google.com';

	async function open_window() {
		await invoke('open_window', {
			url: new_url
		});
	}
</script>

<h1>Mail</h1>

<input id="greet-input" placeholder="Enter website url..." bind:value={input_url} />

<FancyBtn text="Open window" , on_click={open_window} />

<p>Going to {new_url}</p>

<AppShortcut title="Gmail" icon="https://upload.wikimedia.org/wikipedia/commons/7/7e/Gmail_icon_%282020%29.svg" />


<style>
	input {
		width: 80%;
		padding: 0.5em;
		margin: 2em 0.3em;
		margin-bottom: 0;
	}
</style>
