<script>
    import {open} from "@tauri-apps/plugin-dialog";
    const loadDirectory = async () => {
      const res = await open({
        directory: true,
        title: "Select a Dataset Directory",
      });
    };
</script>
<h1>CodeCheck Labelling System</h1>
<p>Load a dataset directory to get started.</p>
<button on:click={_ => loadDirectory()}>Load Directory</button>
