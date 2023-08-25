<script lang="ts">
	import { onMount } from 'svelte';
	// import type { TopAppBarComponentDev } from '@smui/top-app-bar';
	import TopAppBarComponentDev from '@smui/top-app-bar';
	import TopAppBar, { Row, Section, Title, AutoAdjust } from '@smui/top-app-bar';
	import IconButton from '@smui/icon-button';

	let darkTheme: boolean;
	let topAppBar: TopAppBarComponentDev;

	$: modeLabel = `switch to ${darkTheme ? 'light' : 'dark'} mode`;

	// This icon represents the mode to which the user can switch.
	$: modeIcon = darkTheme ? 'light_mode' : 'dark_mode';

	onMount(() => {
		darkTheme = window.matchMedia('(prefers-color-scheme: dark)').matches;
	});

	const toggleMode = () => (darkTheme = !darkTheme);
</script>

<svelte:head>
	{#if darkTheme === undefined}
		<!-- <link rel="stylesheet" href="/smui-light.css" media="(prefers-color-scheme: light)" /> -->
		<link rel="stylesheet" href="./../../theme/_smui-theme.scss" media="(prefers-color-scheme: light)" />
		<link rel="stylesheet" href="./../../theme/dark/_smui-theme.scss" media="screen and (prefers-color-scheme: dark)" />
	{:else if darkTheme}
		<link rel="stylesheet" href="./../../theme/_smui-theme.scss" media="print" />
		<link rel="stylesheet" href="./../../theme/dark/_smui-theme.scss" media="screen" />
	{:else}
		<link rel="stylesheet" href="./../../theme/_smui-theme.scss" />
	{/if}
</svelte:head>

<TopAppBar bind:this={topAppBar} variant="standard">
	<Row>
		<section>
			<IconButton class="material-icons">calendar</IconButton>
			<title>Standard</title>
		</section>
		<section >
			<IconButton
				aria-label={modeLabel}
				class="material-icons"
				on:click={toggleMode}
				title={modeLabel}
			>
				{modeIcon}
			</IconButton>
		</section>
	</Row>
</TopAppBar>
<AutoAdjust {topAppBar}>
	<slot />
</AutoAdjust>

