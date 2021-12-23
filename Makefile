version = 0.1.3
upgrade:
	# 苹果电脑自带的 sed 不支持 \+。
	sed -i "" 's/^version = "[0-9]*.[0-9]*.[0-9]*"/version = "${version}"/' Cargo.toml
	sed -i "" 's/^noiton = "[0-9]*.[0-9]*.[0-9]*"/noiton = "${version}"/g' README.md

tests:
	cargo test --color=always --package noiton --lib noiton::database::tests::query_database
