<h2 class="mt-2">Elevation</h2>

<div>
	<div class="flexy-dad">
		{#each [...Array(24)].map((_v, i) => i + 1) as n}
			<div class="mdc-elevation--z{n} flexy-boy">Elevation: {n}</div>
		{/each}
	</div>

	<style>
	</style>
</div>

<style>
	.flexy-dad {
		display: flex;
		flex-wrap: wrap;
	}

	.flexy-boy {
		display: flex;
		justify-content: center;
		align-items: center;
		width: 120px;
		height: 120px;
		margin: 0 36px 36px 0;
	}
</style>
