<script>
	import '../app.css';
</script>


<main class="w-[100%] max-w-5xl mx-auto mt-8">
	<div class="prose-lg prose-img:rounded-xl prose-a:text-blue-600">
		<slot />
	</div>
</main>

<svelte:head>
	<!-- Fonts -->
	<link rel="stylesheet" href="https://fonts.googleapis.com/icon?family=Material+Icons" />
	<link
		rel="stylesheet"
		href="https://fonts.googleapis.com/css?family=Roboto:300,400,500,600,700"
	/>

	<!-- Material Typography -->
	<link
		rel="stylesheet"
		href="https://unpkg.com/@material/typography@14.0.0/dist/mdc.typography.css"
	/>

	<!-- SMUI -->
	<link rel="stylesheet" href="https://unpkg.com/svelte-material-ui/bare.css" />

	<!-- NerdFonts -->
	<link rel="stylesheet" href="https://www.nerdfonts.com/assets/css/webfont.css" />
</svelte:head>
