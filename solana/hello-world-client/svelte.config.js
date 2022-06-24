import adapter from '@sveltejs/adapter-auto';
// import path from 'path';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	kit: {
		adapter: adapter(),
		// vite: {
		// 	resolve: {
		// 		alias: {
		// 			$js: path.resolve('./src/lib/js')
		// 		}
		// 	}
		// }
	}
};

export default config;
