async function main() {
	const wasm = await import("./pkg");
	const bg = await import("./pkg/index_bg.wasm");

	const program = wasm.WasmChipEight.get_example_program();
	console.log(program);
	const chipEight = new wasm.WasmChipEight(program);
	for(;;) {
		console.log(chipEight.step());
	}

	const a = 4;
	const b = 5;

	const result = wasm.add(a, b);

	console.log(`${a} + ${b} = ${result}`);
};

main()
	.catch(console.error);