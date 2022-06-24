import { writable, type Writable } from "svelte/store";

export const accountCache: Writable<string | null> = writable(null);