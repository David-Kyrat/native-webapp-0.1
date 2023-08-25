<script>
	import FancyBtn from '../lib/components/FancyBtn.svelte';
	import { invoke } from '@tauri-apps/api/tauri';

	let input_url;
    $: new_url = (input_url && input_url.length > 0) ? input_url : "https://google.com";

	async function open_window() {
		await invoke('open_docs', { new_url });
	}
</script>

<h1>Test openning another window</h1>

<!-- <FancyBtn text="Open Docs", on_click={open_docs} /> -->

<input id="greet-input" placeholder="Enter website url..." bind:value={input_url} />
<FancyBtn text="Open window" , on_click={open_window} />

<p>Going to {new_url}</p>
