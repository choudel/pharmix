<script lang="ts">
  import Counter from "./lib/Counter.svelte";
  import { createClient, setContextClient } from "@urql/svelte";
  import { queryStore, gql, getContextClient } from "@urql/svelte";

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
    exchanges: [invokeExchange],
  });
  setContextClient(client);
  const todos = queryStore({
    client: getContextClient(),
    query: gql`
      query {
        list{ 
          
            id
            title
            body
                    
        } 
        drugs{
          id
          dci
          description
        } 
        drug(id:2){
        dci
        }    
      }
    `,
  });

</script>

<main>
  {#if $todos.fetching}
    <p>Loading...</p>
  {:else if $todos.error}
    <p>Oh no... {$todos.error.message}</p>
  {:else}
  <h1>{$todos.data.drug[0].dci}</h1>
    <ul>
      {#each $todos.data.list as todo}
        <li>{todo.title}</li>
        <li>{todo.body}</li>
      {/each}
      {#each $todos.data.drugs as todo}
        <li>{todo.dci}</li>
        <li>{todo.description}</li>
      {/each}
    </ul>
  {/if}
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
