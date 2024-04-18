<script lang="ts">
	import { goto } from "$app/navigation";

	let username = "";
	let password = "";


	async function login() {
        const res = await fetch('/login/api', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({username, password})
        })

		const serverResponseJson = await res.json() as {
			success: boolean,
			message: string | undefined,
			token: string | undefined
		}

		console.log(serverResponseJson)

		if (serverResponseJson.success) {
			goto('/')
		}
	}
</script>

<div class="container flex mx-auto items-center justify-center">

	<div class="text-center bg-gradient-to-r from-purple-500 to-indigo-600 p-8 rounded-md shadow-lg relative">
		<div class="absolute inset-0 bg-white opacity-25 rounded-md pointer-events-none"></div> <!-- Effet de vitre -->
		<h1 class="text-4xl mb-8 text-white font-semibold relative z-10">Welcome Back!</h1>

		<div class="form relative z-10">
			<input class="input mb-4 p-3 w-full rounded-md bg-gray-200 placeholder-gray-500 text-grey-900 focus:outline-none focus:ring-2 focus:ring-indigo-500" title="Username" type="text" placeholder="Username" bind:value={username}/>
			<input class="input mb-4 p-3 w-full rounded-md bg-gray-200 placeholder-gray-500 text-grey-900 focus:outline-none focus:ring-2 focus:ring-indigo-500" title="Password" type="password" placeholder="Password" bind:value={password}/>
		</div>
		<button class="btn variant-filled bg-indigo-600 text-white py-3 px-6 rounded-md hover:bg-indigo-700 transition duration-300 relative z-10" on:click={login}>Login</button>
	</div>

</div>


