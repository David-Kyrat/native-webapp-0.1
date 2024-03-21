<script>
  import AppShortcut from "$lib/components/AppShortcut.svelte"
  // import ImageList, { Item, ImageAspectContainer, Image, Supporting, Label } from "@smui/image-list"
  import { Supporting, Label } from "@smui/image-list"
  import Paper, { Title, Content } from "@smui/paper"
  import LayoutGrid, { Cell } from "@smui/layout-grid"
  /* import { page } from "$app/stores"
  $: is_active = $page.url.pathname === "/" */

  let app_shortcuts = [
    {
      title: "ProtonPass",
      icon: "https://static.wikia.nocookie.net/logopedia/images/e/e4/Proton_Pass_Icon.svg",
      url: "https://pass.proton.me"
    },
    {
      title: "ProtonMail",
      icon: "https://www.privacyguides.org/en/assets/img/email/protonmail.svg",
      url: "https://mail.proton.me"
    },
    {
      title: "UniMail",
      icon: "https://upload.wikimedia.org/wikipedia/commons/d/df/Microsoft_Office_Outlook_%282018%E2%80%93present%29.svg",
      url: "http://outlook.com/owa/unige.ch/munz3@unigech.mail.onmicrosoft.com"
    },
    {
      title: "Gmail",
      icon: "https://upload.wikimedia.org/wikipedia/commons/7/7e/Gmail_icon_%282020%29.svg",
      url: "https://mail.google.com/mail/u/0/#inbox"
    }
  ]
  // $: app_nb = app_shortcuts.length;
  // let col_nb = 4;
  /* Still doesnt work consistently :
    let grid_cols = "grid-cols-3";
	$: grid_cols = `grid-cols-${col_nb}`;
	let gap = `gap-8`;
	gap = `gap-${col_nb * 2}`; */
</script>

<!-- App shortcut grid -->
<!-- <div class="flex flex-row flex-wrap justify-between content-between mx-3 max-w-5xl"> -->
<!-- <div class="grid grid-cols-3 gap-6 mx-5 place-content-evenly max-w-xl"> -->
<!-- <div class="grid {app_nb2} gap-{app_nb}"> -->
<!-- {#each app_shortcuts as ap} -->
<!-- 	<AppShortcut title={ap.title} icon={ap.icon} url={ap.url} styleclass="" /> -->
<!-- {/each} -->
<!-- </div> -->

<Paper class="max-w-[95%] mx-auto">
  <Title>Apps</Title>
  <Content>
    <LayoutGrid>
      {#each app_shortcuts as ap}
        <Cell>
          <AppShortcut title={ap.title} icon={ap.icon} url={ap.url} styleclass="" />
          <Supporting class="!justify-center">
            <Label class="text-center">{ap.title}</Label>
          </Supporting>
        </Cell>
      {/each}
    </LayoutGrid>
  </Content>
</Paper>

<style>
  :global(AppShortcut) {
    display: inline-block;
    padding: 20em;
  }
</style>
