<script>
	import Paper, { Title, Content } from '@smui/paper';
	import Textfield from '@smui/textfield';
	import HelperText from '@smui/textfield/helper-text';
	import { page } from '$app/stores';

	let app_shortcuts = [
		{
			title: 'ProtonPass',
			icon: 'https://static.wikia.nocookie.net/logopedia/images/e/e4/Proton_Pass_Icon.svg',
			url: 'https://pass.proton.me'
		},
		{
			title: 'ProtonMail',
			icon: 'https://www.privacyguides.org/en/assets/img/email/protonmail.svg',
			url: 'https://mail.proton.me'
		},
		{
			title: 'UniMail',
			icon: 'https://upload.wikimedia.org/wikipedia/commons/d/df/Microsoft_Office_Outlook_%282018%E2%80%93present%29.svg',
			url: 'http://outlook.com/owa/unige.ch/munz3@unigech.mail.onmicrosoft.com'
		},
		{
			title: 'Gmail',
			icon: 'https://upload.wikimedia.org/wikipedia/commons/7/7e/Gmail_icon_%282020%29.svg',
			url: 'https://mail.google.com/mail/u/0/#inbox'
		}
	];
	// $: app_nb = app_shortcuts.length;
	// let col_nb = 4;
	/* Still doesnt work consistently :
    let grid_cols = "grid-cols-3";
	$: grid_cols = `grid-cols-${col_nb}`;
	let gap = `gap-8`;
	gap = `gap-${col_nb * 2}`; */
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

