const PIXEL_SIZE = 16;

async function main() {
	const wasm = await import("./pkg");
	const bg = await import("./pkg/index_bg.wasm");

	const bufferWidth = wasm.WasmChipEight.buffer_width();
	const bufferHeight = wasm.WasmChipEight.buffer_height();

	const canvas = document.getElementById("chip-8-canvas");
	canvas.width = bufferWidth * PIXEL_SIZE;
	canvas.height = bufferHeight * PIXEL_SIZE;

	const ctx = canvas.getContext("2d");

	const textBox = document.getElementById("chip-8-steps");

	const program = wasm.WasmChipEight.get_example_program();

	const randomSeed = Math.floor(Math.random() * (Math.pow(2, 32) - 1));
	const chipEight = new wasm.WasmChipEight(program, randomSeed);

	setInterval(() => {
		const stepResult = chipEight.step();
		textBox.value += stepResult + "\n";

		// First clear full display, then manually draw in black squares.
		ctx.fillStyle = "white";
		ctx.fillRect(0, 0, bufferWidth * PIXEL_SIZE, bufferHeight * PIXEL_SIZE);

		ctx.fillStyle = "black";
		for(var y = 0; y < bufferHeight; y++) {
			for(var x = 0; x < bufferWidth; x++) {
				if(chipEight.get_buffer(x, y)) {
					ctx.fillRect(x * PIXEL_SIZE, y * PIXEL_SIZE, PIXEL_SIZE, PIXEL_SIZE);
				}
			}
		}
	}, 1);
};

main()
	.catch(console.error);