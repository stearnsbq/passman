<script lang="ts">
  import { Router, Link, Route } from "svelte-routing";
  import Login from "./routes/login/login.svelte";
  import Setup from "./routes/setup/setup.svelte";
  import Vault from './routes/vault/vault.svelte';
</script>

<Router>
  <Route path="/" component={Login}>
  </Route>
  <Route path="setup" component={Setup}>
  </Route>
  <Route path="vault" component={Vault}>
  </Route>
</Router>

<style>
</style>
