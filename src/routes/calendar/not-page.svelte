<script>
	import FancyBtn from '$lib/components/FancyBtn.svelte';
	import { invoke } from '@tauri-apps/api/tauri';

	let input_url;
	$: new_url = input_url && input_url.length > 0 ? input_url : 'https://google.com';

	async function open_window() {
		await invoke('open_window', {
			url: new_url
		});
	}
</script>

<h1>Calendar</h1>

<input
	id="greet-input"
	placeholder="Enter website url..."
	bind:value={input_url}
/>

<FancyBtn text="Open window" , on_click={open_window} />

<p>Going to {new_url}</p>

<style>
	input {
        width: 80%;
        padding: 0.5em;
        margin: 2em 0.3em;
	}
</style>
