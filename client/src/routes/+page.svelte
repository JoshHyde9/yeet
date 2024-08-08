<script lang="ts">
  import { onMount } from "svelte";

  type Response = {
    message: string;
  };

  let message: Response;

  onMount(async () => {
    const response = await fetch("http://localhost:5000");

    const data = await response.json();

    if (!data) {
      console.error("No data found");

      message.message = "No data found";
    }

    message = data;
  });
</script>

<h1 class="text-6xl">Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>

{#if message}
  <p>{message.message}</p>
{:else}
  <p>loading</p>
{/if}
