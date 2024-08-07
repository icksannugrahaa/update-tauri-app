import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';

export default defineConfig({
	plugins: [
		sveltekit(),
		{
			name: 'tailwind',
			enforce: 'pre',
			apply: 'build',
			transformIndexHtml(html) {
				return html.replace(
					/<link rel="stylesheet".*?>/,
					'<link rel="stylesheet" href="/src/styles.css">'
				)
			},
		},
	],
	test: {
		include: ['src/**/*.{test,spec}.{js,ts}']
	}
});