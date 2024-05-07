run_m1:
	cargo run --release --features npz --bin viewer ../GS_DATA_RAW/GS_model_VQ/point_cloud/iteration_35000/point_cloud.npz ../GS_DATA_RAW/GS_model_VQ/cameras.json

run_m2:
	cargo run --release --features npz --bin viewer ../GS_DATA_RAW/Municipal/point_cloud/iteration_35000/point_cloud.npz ../GS_DATA_RAW/Municipal/cameras.json