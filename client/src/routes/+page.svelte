<script lang="ts">
  import { onMount } from "svelte";

  type Users = {
    id: string;
    name: string;
  }[];

  let users: Users;

  onMount(async () => {
    const response = await fetch("http://localhost:5000/api");

    const data: Users = await response.json();

    if (!data) {
      console.error("No data found");
    }

    users = data;
  });
</script>

<h1 class="text-6xl">Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>

{#if users}
  {#each users as user}
    <p>{user.name}</p>
  {/each}
{:else}
  <p>loading...</p>
{/if}
