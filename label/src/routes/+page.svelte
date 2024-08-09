<script lang="ts">
    import {open} from "@tauri-apps/plugin-dialog";
    import {invoke} from "@tauri-apps/api/core";
    let err: String | null = null;
    const loadDirectory = async () => {
      const res = await open({
        directory: true,
        title: "Select a Dataset Directory",
      });

      if (res === null) {
        return;
      }

      try {
        await invoke("validate_directory", {path: res});
        window.location.href = "/items";
      } catch (error) {
        err = error;
      }
    };
</script>
<h1>CodeCheck Labelling System</h1>
<p>Load a dataset directory to get started.</p>
<button on:click={_ => loadDirectory()}>Load Directory</button>

{#if err !== null}
    <p style="color: red">Error: {err}</p>
{/if}
