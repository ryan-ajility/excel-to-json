// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

// https://astro.build/config
export default defineConfig({
	site: 'https://ryan-ajility.github.io',
	base: '/excel-to-json',
	integrations: [
		starlight({
			title: 'excel-to-json',
			logo: {
				src: './src/assets/excel-to-json-logo.svg',
			},
			social: [
				{ icon: 'github', label: 'GitHub', href: 'https://github.com/ryan-ajility/excel-to-json' },
			],
			sidebar: [
				{
					label: 'Getting Started',
					items: [
						{ label: 'Introduction', slug: 'getting-started/introduction' },
						{ label: 'Installation', slug: 'getting-started/installation' },
						{ label: 'Quick Start', slug: 'getting-started/quick-start' },
					],
				},
				{
					label: 'Usage',
					items: [
						{ label: 'Command Line', slug: 'usage/command-line' },
						{ label: 'JavaScript/Node.js', slug: 'usage/javascript' },
						{ label: 'TypeScript', slug: 'usage/typescript' },
						{ label: 'Other Languages', slug: 'usage/other-languages' },
					],
				},
				{
					label: 'API Reference',
					items: [
						{ label: 'CLI Options', slug: 'reference/cli-options' },
						{ label: 'Output Format', slug: 'reference/output-format' },
						{ label: 'Error Handling', slug: 'reference/error-handling' },
						{ label: 'Rust API Docs', slug: 'reference/rust-api' },
					],
				},
			],
			customCss: ['./src/styles/custom.css'],
		}),
	],
});
