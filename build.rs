fn main() {
	println!("cargo::rerun-if-changed=src/pbrt/data/rgbspectrum_aces.c");
	println!("cargo::rerun-if-changed=src/pbrt/data/rgbspectrum_dci_p3.c");
	println!("cargo::rerun-if-changed=src/pbrt/data/rgbspectrum_rec2020.c");
	println!("cargo::rerun-if-changed=src/pbrt/data/rgbspectrum_srgb.c");
	cc::Build::new()
		.file("src/pbrt/data/rgbspectrum_aces.c")
		.file("src/pbrt/data/rgbspectrum_dci_p3.c")
		.file("src/pbrt/data/rgbspectrum_rec2020.c")
		.file("src/pbrt/data/rgbspectrum_srgb.c")
		.compile("pbrt_data");
}
