<script>
	import Tab, { Label } from '@smui/tab';
	import TabBar from '@smui/tab-bar';
    let uni_login_url = "https://appw.cursus.unige.ch/cursus/appw/api/student-grade-certificates"

	let active = 'Home';
</script>

<div>
	<TabBar tabs={['Log In']} let:tab bind:active>
		<Tab
			{tab}
			href={uni_login_url}
			target="href-tabs-frame"
		>
			<Label>{tab}</Label>
		</Tab>
	</TabBar>

	<iframe
		src="https://en.wikipedia.org/wiki/Home"
		title="Selected Tab"
		name="href-tabs-frame"
		style="width: 100%; height: 50em; border: 0;"
	/>
</div>


<!-- <div>
	<TabBar tabs={['Home', 'Merchandise', 'About Us']} let:tab bind:active>
		<Tab
			{tab}
			href="https://en.wikipedia.org/wiki/{tab.replace(/ /g, '_')}"
			target="href-tabs-frame"
		>
			<Label>{tab}</Label>
		</Tab>
	</TabBar>

	<iframe
		src="https://en.wikipedia.org/wiki/Home"
		title="Selected Tab"
		name="href-tabs-frame"
		style="width: 100%; height: 50em; border: 0;"
	/>
</div> -->
