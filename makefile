run_m1:
	cargo run --release --features npz --bin viewer ../../gs/GS_model_VQ/point_cloud/iteration_35000/point_cloud.npz ../../gs/GS_model_VQ/cameras.json

run_m2:
	cargo run --release --features npz --bin viewer ../../gs/Municipal/point_cloud/iteration_35000/point_cloud.npz ../../gs/Municipal/cameras.json