use blob_codec::BlobCodec;

mod ckzg;
mod constants;
mod evaluation;

pub fn run(z_point: [u8; 32]) {
    let blob = BlobCodec::from_dir("./test_files").unwrap().to_bytes();

    let ckzg_y_point = ckzg::calculate_kzg(blob, z_point).unwrap().to_vec();
    let mut evaluation_y_point = evaluation::evaluate(blob, z_point, true)
        .unwrap()
        .to_bytes()
        .to_vec();

    evaluation_y_point.reverse();

    println!("ckzg: {ckzg_y_point:?}");
    println!("evaluation: {evaluation_y_point:?}");

    assert_eq!(ckzg_y_point, evaluation_y_point);
}
