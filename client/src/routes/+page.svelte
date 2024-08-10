<script lang="ts">
  export let data;
</script>

<h1 class="text-6xl">Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>

{#if data.users}
  {#each data.users as user}
    <div class="pb-4">
      <p>{user.id}</p>
      <p>{user.first_name}</p>
      <p>{user.last_name}</p>
      <p>{user.email}</p>
      <p>{user.password}</p>
    </div>
  {/each}
{:else}
  <p>loading...</p>
{/if}
