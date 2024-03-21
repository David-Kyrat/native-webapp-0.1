<!-- <script>
	import Drawer, { AppContent, Content, Header, Title, Subtitle, Scrim } from '@smui/drawer';
	import Button, { Label } from '@smui/button';
	import List, { Item, Text, Graphic, Separator, Subheader } from '@smui/list';

	let open = false;
	let active = 'Inbox';

	function setActive(value) {
		active = value;
		open = false;
	}
	let pinned_locations = [
		'/home/noahl/Documents/ba5/IA-13X005',
		'/home/noahl/Documents/ba5/IA-13X005',
		'/home/noahl/Documents/ba5/IA-13X005',
		'/home/noahl/Documents/ba5/IA-13X005',
		'/home/noahl/Documents/ba5/IA-13X005',
		'/home/noahl/Documents/ba5/IA-13X005'
	];

	/// Get last element of path
	function path_last(params) {
		var path = params,
			paths = [],
			i = 1;
    let old_i = 0;
		while ((i = path.indexOf('/', i) + 1)) {
			paths.push(path.substr(old_i, i));
      old_i = i;
		}
    let len =paths.length;
    /* let out = paths[len-2] + "/" + paths[len-1];
    console.log(paths);
		return params.substring(params.lastIndexOf('/') + 1); */
		return paths[len-1];
	}
	let pinned_loc_items = new Map();
	let i = 0;
	for (const loc of pinned_locations) {
		pinned_loc_items.set(i, {
			id: i,
			uri: loc,
			on_click: () => setActive(i),
			activated: active === i,
			icon: 'folder',
			// text: loc.replace("C:/Users/noahm/OneDrive/Documents", "Docs")
			text: path_last(loc)
		});
		i += 1;
	}
</script> -->

<!-- <div class="drawer-container"> -->
<!-- Don't include fixed={false} if this is a page wide drawer.
        It adds a style for absolute positioning. -->
<!-- <Drawer variant="modal" bind:open> -->
<!-- 	<Header> -->
<!-- 		<Title>Pinned Locations</Title> -->
<!-- 		<Subtitle>Folder added to quick access</Subtitle> -->
<!-- 	</Header> -->
<!-- 	<Content> -->
<!-- 		<List> -->
<!-- 			{#each pinned_loc_items as loc_item} -->
<!-- 				<Item -->
<!-- 					href={loc_item[1].uri} -->
<!-- 					on:click={loc_item[1].on_click} -->
<!-- 					activated={loc_item[1].activated} -->
<!-- 				> -->
<!-- 					<Graphic class="material-icons" aria-hidden="true">{loc_item[1].icon}</Graphic> -->
<!-- 					<Text>{loc_item[1].text}</Text> -->
<!-- 				</Item> -->

				<!-- <Item
				href="javascript:void(0)"
				on:click={() => setActive('Inbox')}
				activated={active === 'Inbox'}
			>
				<Graphic class="material-icons" aria-hidden="true">inbox</Graphic>
				<Text>Inbox</Text>
			</Item>
			<Item
				href="javascript:void(0)"
				on:click={() => setActive('Star')}
				activated={active === 'Star'}
			>
				<Graphic class="material-icons" aria-hidden="true">star</Graphic>
				<Text>Star</Text>
			</Item>
			<Item
				href="javascript:void(0)"
				on:click={() => setActive('Sent Mail')}
				activated={active === 'Sent Mail'}
			>
				<Graphic class="material-icons" aria-hidden="true">send</Graphic>
				<Text>Sent Mail</Text>
			</Item>
			<Item
				href="javascript:void(0)"
				on:click={() => setActive('Drafts')}
				activated={active === 'Drafts'}
			>
				<Graphic class="material-icons" aria-hidden="true">drafts</Graphic>
				<Text>Drafts</Text>
			</Item>

			<Separator />
			<Subheader tag="h6">Labels</Subheader>
			<Item
				href="javascript:void(0)"
				on:click={() => setActive('Family')}
				activated={active === 'Family'}
			>
				<Graphic class="material-icons" aria-hidden="true">bookmark</Graphic>
				<Text>Family</Text>
			</Item>
			<Item
				href="javascript:void(0)"
				on:click={() => setActive('Friends')}
				activated={active === 'Friends'}
			>
				<Graphic class="material-icons" aria-hidden="true">bookmark</Graphic>
				<Text>Friends</Text>
			</Item>
			<Item
				href="javascript:void(0)"
				on:click={() => setActive('Work')}
				activated={active === 'Work'}
			>
				<Graphic class="material-icons" aria-hidden="true">bookmark</Graphic>
				<Text>Work</Text>
			</Item> -->
			<!-- {/each}
		</List>
	</Content>
</Drawer> -->

<!-- Don't include fixed={false} if this is a page wide drawer.
        It adds a style for absolute positioning. -->
<!-- <Scrim fixed={false} />
<AppContent class="app-content">
	<main class="main-content">
		<Button on:click={() => (open = !open)}>
			<Label>Toggle Drawer</Label>
		</Button>
		<br />
		<pre class="status">Active: {active}</pre>
	</main>
</AppContent> -->
<!-- </div> -->
