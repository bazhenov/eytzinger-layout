std.csv:
	cargo bench --bench bench -- "std"
	./read-perf-data.sh ./target/criterion/std | sort -gt',' -k1 > $@

eytzinger.csv:
	cargo bench --bench bench -- "eytzinger/"
	./read-perf-data.sh ./target/criterion/eytzinger | sort -gt',' -k1 > $@

eytzinger-prefetch.csv:
	cargo bench --bench bench --features prefetch -- "eytzinger/"
	./read-perf-data.sh ./target/criterion/eytzinger | sort -gt',' -k1 > $@

eytzinger-branchless.csv:
	cargo bench --bench bench -- "eytzinger branchless"
	./read-perf-data.sh "./target/criterion/eytzinger branchless" | sort -gt',' -k1 > $@

eytzinger-branchless-prefetch.csv:
	cargo bench --bench bench --features prefetch -- "eytzinger branchless"
	./read-perf-data.sh "./target/criterion/eytzinger branchless" | sort -gt',' -k1 > $@
