import {defineConfig} from 'vite';

export default defineConfig({
    base: '/2024-11-leptos-meetup/',
    css: {
        preprocessorOptions: {
            scss: {
                api: 'modern-compiler',
                silenceDeprecations: ['color-functions', 'global-builtin', 'import']
            }
        }
    }
});
