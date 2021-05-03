const height = 500;
const width = 500;

async function main() {
    const lib = await import("../pkg/index.js").catch(console.error);

    const canvas = document.createElement("canvas");
    canvas.width = width;
    canvas.height = height;

    const ctx = canvas.getContext("2d");
    lib.draw(ctx, width, height, 0.032, 0.032, 0.011);

    document.body.appendChild(canvas);
}

main();