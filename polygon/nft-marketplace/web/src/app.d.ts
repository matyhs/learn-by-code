/// <reference types="@sveltejs/kit" />

// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare namespace App {
	interface Locals {
		user: import('$lib/types/class').User
	}
	// interface Platform {}
	interface Session {
		user: import('$lib/types/class').User
	}
	// interface Stuff {}
}
