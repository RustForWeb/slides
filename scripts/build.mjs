import {readdir, cp, mkdir, rm, stat} from 'fs/promises';

const exists = async (path) => {
    try {
        await stat(path);
        return true;
    } catch (err) {
        return false;
    }
};

if (await exists('./dist')) {
    await rm('./dist', {
        recursive: true
    });
}
await mkdir('./dist', {
    recursive: true
});

for (const entry of (await readdir('./packages', {withFileTypes: true})).filter((entry) => entry.isDirectory())) {
    const source = `./packages/${entry.name}/dist`;
    const destination = entry.name === 'index' ? './dist' : `./dist/${entry.name}`;

    await cp(source, destination, {
        recursive: true
    });

    console.log(`Copied "${source}" to "${destination}".`);
}
