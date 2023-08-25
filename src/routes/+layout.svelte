<script>
	import '../app.css';

	let pages = ['', 'bindings', 'about'];

	let navClass =
		'bg-white dark:bg-gray-900 fixed w-full z-20 top-0 left-0 border-b border-gray-200 dark:border-gray-600';
	let divClasses = [
		'max-w-screen-xl flex flex-wrap items-center justify-between mx-auto p-4',
		'items-center justify-between w-full md:flex md:w-auto md:order-1'
		//"w-full md:block md:w-auto",
	];
	let ulClass =
		'font-medium flex flex-col p-4 md:p-0 mt-4 border border-gray-100 rounded-lg bg-gray-50 md:flex-row md:space-x-8 md:mt-0 md:border-0 md:bg-white dark:bg-gray-800 md:dark:bg-gray-900 dark:border-gray-700';

	let aClassDef =
		'block py-2 pl-3 pr-4  rounded hover:bg-gray-100 md:hover:bg-transparent md:border-0 md:hover:text-blue-700 md:p-0 dark:text-white md:dark:hover:text-blue-500 dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent';
	let aInactive = 'text-gray-900';
	// let aActive = "text-white bg-blue-700";
	function capitalize(str) {
		return str[0].toUpperCase() + str.slice(1);
	}
</script>

<nav class={navClass}>
	<div class={divClasses[0]}>
		<div class={divClasses[1]} id="navbar-sticky">
			<ul class={ulClass}>
				{#each pages as pageName}
					<li>
						<a href="/{pageName}" class="{aClassDef} {aInactive}">
							{pageName == '' ? 'Home' : capitalize(pageName)}
						</a>
					</li>
				{/each}
			</ul>
		</div>
	</div>
</nav>

<main class="w-[90%] max-w-3xl mx-auto mt-24">
	<div class="prose-lg prose-img:rounded-xl prose-a:text-blue-600">
		<slot />
	</div>
</main>
