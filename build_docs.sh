cargo doc --no-deps --open
rm -rf ./docs
echo "<meta http-equiv=\"refresh\" content=\"0; url=surreal_model_layer\">" > target/doc/index.html
cp -r target/doc ./docs