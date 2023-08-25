<script>
	import { invoke } from '@tauri-apps/api/tauri';
    import FancyBtn from './components/FancyBtn.svelte';

	let name = '';
	let greetMsg = '';
	let clicked = 0;

	async function greet() {
		greetMsg = await invoke('greet', { name });
	}
</script>

<div>
	<input id="greet-input" placeholder="Enter a name..." bind:value={name} />
	<!-- <button on:click={greet}>Greet</button> -->
    <FancyBtn text="Greet", on_click={greet} />

	<p>{greetMsg}</p>
</div>
