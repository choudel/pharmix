<script lang="ts">
  import Counter from './lib/Counter.svelte'
  import { invoke } from '@tauri-apps/api/tauri'
  async function getMessage(){
    const res = await invoke('my_full_send_command')
    return res
  }
  let promise = getMessage()
  invoke('custom', { runMessage: 'Hello!' })
 
  
  
</script>

<main>
  <div>
    {#await promise then value}
    <h1>{value}</h1>
    {/await}
  </div>
  <h1>Vite + Svelte</h1>

  <div class="card">
    <Counter />
  </div>

  <p>
    Check out <a href="https://github.com/sveltejs/kit#readme" target="_blank">SvelteKit</a>, the official Svelte app framework powered by Vite!
  </p>

  <p class="read-the-docs">
    Click on the Vite and Svelte logos to learn more
  </p>
</main>

<style>
  .read-the-docs {
    color: #888;
  }
</style>