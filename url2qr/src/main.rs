use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use csv::Error;

use qrcodegen::QrCode;
use qrcodegen::QrCodeEcc;


fn main() -> Result<(), Error> {

    let args: Vec<String> = env::args().collect();
    //dbg!(&args);
    let file_path = &args[1]; // args[0] = exe file path

    println!("Looking in file {}", file_path);
    
    let contents = fs::read_to_string(file_path)
                    .expect("Couldn't read from file");

    println!("contents:\n{}\n\n", contents);

    let mut reader = csv::Reader::from_reader(contents.as_bytes());

    let mut url_num: i32 = 0;

    for url in reader.records() {
        let url = url?;
        println!("{}", &url[0]);

        //Turn into QR code

        let qr = QrCode::encode_text(&url[0], QrCodeEcc::Medium).unwrap();

        //let _svg = to_svg_string(&qr, 4);

        print_qr(&qr, url_num);
        url_num += 1;
    }

    Ok(())
}

/*---- Utilities ----*/

// Using these from: https://github.com/nayuki/QR-Code-generator/blob/master/rust/examples/qrcodegen-demo.rs

// Prints the given QrCode object to the console and file.
fn print_qr(qr: &QrCode, url_num: i32) {

    let mut file_name: String = "qr".to_string();

    if url_num != 0 {
        file_name = file_name + "(" + &(url_num.to_string()).to_string() + ").txt";
    }
    else {
        file_name = file_name + ".txt";
    }

    //println!("QR PATH NAME: {}", file_name);
    
    let mut file = File::create(file_name).expect("Error while creating svg file");

    let mut qr_full: String = "".to_string();

	let border: i32 = 4;
	for y in -border .. qr.size() + border {
		for x in -border .. qr.size() + border {
			let c: &str = if qr.get_module(x, y) { "█" } else { " " };
			print!("{0}{0}", c);
            qr_full += c;
		}
        qr_full += "\n";
		println!();
	}
	println!();

    file.write_all(qr_full.as_bytes()).expect("Error writing to file");
}