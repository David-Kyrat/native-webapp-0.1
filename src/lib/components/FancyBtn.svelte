<script>
	export let text = 'click me';
	export let on_click;
	export let padding_x = 'px-5';
    export let disabled = false;
    export let type = '';
    
    $: hover_font_color = disabled ? "gray" : "violet";
    $: font_color = disabled ? "gray" : "black";
    $: brdr_col = disabled ? ['gray-400', 'gray-500'] : ['from-purple-600', 'to-blue-500'];

	$: span_class = `relative ${padding_x} py-2.5 transition-all ease-in duration-75 bg-white white:bg-gray-900 rounded-md group-hover:text-${hover_font_color}-600`;
	export let style = "";
	$: btn_class = `relative inline-flex items-center justify-center p-0.5 mb-2 mr-2 overflow-hidden text-sm font-medium hover:bg-gray-900 text-gray-900 rounded-lg group bg-gradient-to-br ${brdr_col[0]} ${brdr_col[1]}  white:text-dark focus:ring-2 focus:outline-none focus:ring-blue-200 white:focus:ring-blue-800 text-${font_color}-400`;
</script>

<button class={btn_class} on:click={on_click} {disabled} {type} style="{style}; ">
	<span class={span_class}> {text} </span>
</button>
    <span class={span_class}>{font_color}</span> <br/>
    <span>{brdr_col[0]}  ;  {brdr_col[1]}</span>
