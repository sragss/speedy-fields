compile_metal:
	xcrun -sdk macosx metal -c src/metal/fp.metal -o src/metal/fp.air
	xcrun -sdk macosx metallib src/metal/fp.air -o src/metal/fp.metallib
