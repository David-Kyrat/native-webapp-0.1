<script>
	import Card, { Content, PrimaryAction, Media, MediaContent } from '@smui/card';
	import { invoke } from '@tauri-apps/api/tauri';

	export let title = 'Title';
	export let icon = 'https://placehold.co/320x320?text=square';

    export let styleclass;
	export let url;
	$: new_url = url && url.length > 0 ? url : 'https://google.com';

	async function open_window() {
		await invoke('open_window', { url: url, title: title });
	}
</script>

<div class="card-container {styleclass}">
	<Card style="min-width: 100px; display: flex;" >
		<PrimaryAction on:click={() => open_window()}>
			<Media class="card-media-16x9" aspectRatio="16x9" style="background-image: url({icon}); ">
				<div style="color: #fff; position: absolute; bottom: 5px; left: 16px;">
					{#if icon.includes('placehold.co')}
						<h3 class="mdc-typography--Caption" style="margin: 0;">{title}</h3>
					{/if}
				</div>
			</Media>
		</PrimaryAction>
	</Card>
</div>

<style>
	.card-container {
        max-width: 80%;
		/* width: 17%; */
	}

	* :global(.card-media-16x9) {
		margin: 0.5em 0em;
		background-size: contain;
	}
</style>
