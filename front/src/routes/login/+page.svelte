<script>
	let username = "";
	let password = "";


	async function login() {
		console.log("username: " + username, "password: " + password);
        await fetch('/login/api', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({username, password})
        })
	}
</script>





<div class="container flex mx-auto items-center justify-center">

	<div class="text-center bg-gray-700 p-8 rounded-md">
		<h1 class="h1 mb-8">Go to your page!</h1>


		<div class="form">
			<input class="input mb-4 p-2 w-full" title="Username" type="test" placeholder="Username" bind:value={username}/>
			<input class="input mb-4 p-2 w-full" title="Password" type="password" placeholder="Password" bind:value={password}/>
		</div>
		<button class="btn variant-filled" on:click={login}>Login</button>
	</div>

</div>
