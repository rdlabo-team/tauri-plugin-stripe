<script>
  import Greet from './lib/Greet.svelte'
  import { initialize, createPaymentSheet, presentPaymentSheet } from 'tauri-plugin-stripe-payment-api'

	let response = ''

	function updateResponse(returnValue) {
		response += `[${new Date().toLocaleTimeString()}] ` + (typeof returnValue === 'string' ? returnValue : JSON.stringify(returnValue)) + '<br>'
	}

	async function _ping() {
    await initialize({
      publishableKey: 'pk_test_51MmARtKzMYim9cy3tOI5vOdHbai4G26V1AiDJmiE4aiAXc8BaSzh9Z0b0f8Novn0Jyyi8JqNdzLzcI2rUGT4g8ct00gfUVdLuM',
    });
    const { setupIntent, ephemeralKey, customer } = await fetch('https://j3x0ln9gj7.execute-api.ap-northeast-1.amazonaws.com/dev/intent/setup',{
      method: "POST",
    }).then(res => res.json());
    console.log({ setupIntent, ephemeralKey, customer} );
    await createPaymentSheet({
      setupIntentClientSecret: setupIntent,
      customerEphemeralKeySecret: ephemeralKey,
      customerId: customer,
      merchantDisplayName: 'rdlabo',
    }).catch(e => {
      console.error(e);
      throw e;
    });
    console.log('createPaymentSheet');
    await presentPaymentSheet();
	}
</script>

<main class="container">
  <h1>Welcome to Tauri!</h1>

  <div class="row">
    <a href="https://vitejs.dev" target="_blank">
      <img src="/vite.svg" class="logo vite" alt="Vite Logo" />
    </a>
    <a href="https://tauri.app" target="_blank">
      <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
    </a>
    <a href="https://svelte.dev" target="_blank">
      <img src="/svelte.svg" class="logo svelte" alt="Svelte Logo" />
    </a>
  </div>

  <p>
    Click on the Tauri, Vite, and Svelte logos to learn more.
  </p>

  <div class="row">
    <Greet />
  </div>

  <div>
    <button on:click="{_ping}">Ping</button>
    <div>{@html response}</div>
  </div>

</main>

<style>
  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }
</style>
