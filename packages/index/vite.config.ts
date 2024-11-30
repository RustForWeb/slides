import {defineConfig} from 'vite';

export default defineConfig({
    base: '/',
    css: {
        preprocessorOptions: {
            scss: {
                api: 'modern-compiler',
                silenceDeprecations: ['color-functions', 'global-builtin', 'import']
            }
        }
    }
});
