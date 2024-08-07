import { writable } from 'svelte/store';

export const BASE_URL = "https://ahamart-apigateway.service-aha.id/cms";
export const token = writable('');
export const refreshToken = writable('');