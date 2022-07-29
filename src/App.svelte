<script lang="ts">
  import Counter from "./lib/Counter.svelte";
  import { createClient, setContextClient } from "@urql/svelte";
  import { queryStore, gql, getContextClient } from '@urql/svelte';

  import { invoke } from "@tauri-apps/api/tauri";
  import { invokeExchange } from "tauri-plugin-graphql-urql";

  async function getMessage() {
    const res = await invoke("my_custom_command", {
      number: 12,
    });
    return res;
  }
  let promise = getMessage();

  const client = createClient({
    url: "graphql",
  });
  setContextClient(client);
  const todos = queryStore({
    client: getContextClient(),
    query: gql`
      query {
        todos {
          id
          title
        }
      }
    `,
  });
</script>

<main>
  <h1>Vite + Svelte</h1>

  <div class="card">
    <Counter />
  </div>

  <p>
    Check out <a href="https://github.com/sveltejs/kit#readme" target="_blank"
      >SvelteKit</a
    >, the official Svelte app framework powered by Vite!
  </p>

  <p class="read-the-docs">Click on the Vite and Svelte logos to learn more</p>
</main>

<style>
  .read-the-docs {
    color: #888;
  }
</style>
