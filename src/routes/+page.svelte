<script>
	import Paper, { Title, Content } from '@smui/paper';
	import Textfield from '@smui/textfield';
	import HelperText from '@smui/textfield/helper-text';
	import { page } from '$app/stores';

	let value = '';
	$: is_active = $page.url.pathname === '/';
    
    async function browse() {
        let url = value;
        if (!url.includes("http")) url = `https://${url}`
        window.location = url
    }
</script>

<Paper class="max-w-[95%] mx-5 my-auto h-80">
	<Title>App opener</Title>
	<br />
	<Content>
		<div class="margins">
			<form on:submit={browse}>
				<input value/>
				<Textfield
					style="width: 100%;"
					helperLine$style="width: 100%;"
					bind:value
					label="Enter address"
				>
					<HelperText slot="helper">Website will open in another window</HelperText>
				</Textfield>
				<button type="submit"> Please save me </button>
			</form>
		</div>
        <p>a = {value}</p>
	</Content>
</Paper>

<style>
	* :global(AppShortcut) {
		display: inline-block;
		padding: 20em;
	}
	* :global(TextField) {
		padding: 20px auto;
	}
	* :global(Title) {
		padding: 50px auto !important;
		margin-bottom: 50px auto !important;
	}
</style>
