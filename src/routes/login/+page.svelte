<script>
  import { onMount } from 'svelte';
  import { get } from 'svelte/store';
	import '../../app.css';
  import { refreshToken, token, BASE_URL } from '../stores.js'; 
  import { goto } from '$app/navigation';

	let login = '';
	let password = '';
  let error = null;
  let loading = false;
  let response = null;

  onMount(() => {
    // Check if user is already logged in
    if ($token) {
      goto('/list');  // Redirect to dashboard if already logged in
    }
  });

	async function handleSubmit() {
    console.log("clicked!!");
		loading = true;
		error = null;
		response = null;

		try {
			const res = await fetch("https://p1.aha.id/api/cms_v2/sessions/login", {
				method: 'POST',
				body: JSON.stringify({
					login,
					password
				}),
				headers: {
					'Content-type': 'application/json; charset=UTF-8'
				}
			});

			if (!res.ok) {
				throw new Error('Failed to create post');
			}

			response = await res.json();
      console.log(response);
      token.set(response.token)
      console.log(get(token));
      goto('/list');
		} catch (err) {
			error = err.message;
      console.log(error);
		} finally {
			loading = false;
		}
	}
</script>

<svelte:head>
	<title>Login</title>
	<meta name="description" content="About this app" />
</svelte:head>

<div
	class="min-h-screen bg-gradient-to-br from-purple-100 to-indigo-200 flex items-center justify-center p-4"
>
	<div class="max-w-md w-full space-y-8 p-10 bg-white rounded-2xl shadow-lg">
		<div class="text-center">
			<h2 class="mt-6 text-4xl font-extrabold text-gray-900 tracking-tight">Welcome back</h2>
			<p class="mt-2 text-sm text-gray-600">Sign in to your account</p>
		</div>
		<form class="mt-8 space-y-6" on:submit|preventDefault={handleSubmit}>
			<div class="space-y-4">
				<div>
					<label for="email-address" class="sr-only">Email address</label>
					<input
						id="email-address"
						name="email"
						type="email"
						autocomplete="email"
						required
						class="appearance-none relative block w-full px-3 py-3 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm transition duration-300 ease-in-out"
						placeholder="Email address"
						bind:value={login}
					/>
				</div>
				<div>
					<label for="password" class="sr-only">Password</label>
					<input
						id="password"
						name="password"
						type="password"
						autocomplete="current-password"
						required
						class="appearance-none relative block w-full px-3 py-3 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm transition duration-300 ease-in-out"
						placeholder="Password"
						bind:value={password}
					/>
				</div>
			</div>

			<div class="flex items-center justify-between">
				<div class="flex items-center">
					<input
						id="remember-me"
						name="remember-me"
						type="checkbox"
						class="h-4 w-4 text-indigo-600 focus:ring-indigo-500 border-gray-300 rounded"
					/>
					<label for="remember-me" class="ml-2 block text-sm text-gray-900"> Remember me </label>
				</div>

				<div class="text-sm">
					<a href="#" class="font-medium text-indigo-600 hover:text-indigo-500">
						Forgot your password?
					</a>
				</div>
			</div>

			<div>
				<button
					type="submit"
					class="group relative w-full flex justify-center py-3 px-4 border border-transparent text-sm font-medium rounded-lg text-white bg-gradient-to-r from-indigo-600 to-purple-600 hover:from-indigo-700 hover:to-purple-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 transition duration-300 ease-in-out transform hover:-translate-y-1 hover:shadow-lg"
				>
					Sign in
				</button>
			</div>
		</form>
		<p class="mt-2 text-center text-sm text-gray-600">
			Not a member?
			<a href="#" class="font-medium text-indigo-600 hover:text-indigo-500">
				Start a 14 day free trial
			</a>
		</p>
	</div>
</div>
