use std::{env, fs, path::PathBuf, process::Command};

fn main() {
	println!("cargo::rerun-if-changed=src/pbrt/cmd/rgb2spec_opt.cpp");
	let rgb2spec_opt = compile_cpp("src/pbrt/cmd/rgb2spec_opt.cpp", "rgb2spec_opt");
	let files = generate_rgbspectrums(rgb2spec_opt);
	cc::Build::new().files(files).compile("pbrt_data");
}

fn generate_rgbspectrums(rgb2spec_opt: PathBuf) -> Vec<PathBuf> {
	let cmd_time = fs::metadata(&rgb2spec_opt).unwrap().modified().unwrap();
	let mapping =
		[("srgb", "sRGB"), ("dci_p3", "DCI_P3"), ("rec2020", "REC2020"), ("aces", "ACES2065_1")];
	// see rsr::pbrt::color::RES
	let res = "64";
	let out_dir = get_out_dir();
	let mut ret = Vec::new();

	for (name, gamut) in mapping {
		let output = out_dir.join(format!("rgbspectrum_{name}.c"));
		if output.is_file() {
			if is_ci() {
				println!("cargo::warning=skip regenerating {} in CI", output.display());
				continue;
			}
			let output_time = fs::metadata(&output).unwrap().modified().unwrap();
			if cmd_time < output_time {
				continue;
			}
		}
		run(
			Command::new(&rgb2spec_opt).arg(res).arg(&output).arg(gamut),
			&format!("failed to generate {gamut} rgbspectrum"),
		);
		ret.push(output);
	}

	ret
}

fn compile_cpp(file: &str, output: &str) -> PathBuf {
	let out_dir = get_out_dir();
	let output = out_dir.join(format!("{output}.exe"));
	run(
		cc::Build::new()
			.cpp(true)
			.opt_level(3)
			.get_compiler()
			.to_command()
			.arg(file)
			.arg("-o")
			.arg(&output),
		"failed to compile C++ code",
	);

	output
}

fn run(cmd: &mut Command, err_msg: &str) {
	let status = cmd.status().expect(err_msg);
	if !status.success() {
		println!("cargo::error={err_msg}");
		std::process::exit(1);
	}
}

fn get_out_dir() -> PathBuf {
	PathBuf::from(env::var("OUT_DIR").unwrap())
}

fn is_ci() -> bool {
	if let Ok(github) = env::var("GITHUB_ACTIONS")
		&& github == "true"
	{
		return true;
	}

	false
}
