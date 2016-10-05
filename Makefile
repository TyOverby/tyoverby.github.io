TEMPLATE=cpp -w -undef -nostdinc -E -P
MARKDOWN=pandoc -f markdown -t html --katex --toc
SHARED=common-header.t.html common-footer.t.html css/style.css js/katex.min.js css/katex.min.css

all: index.html posts/rust-speed.html posts/rust-vs-go.html posts/webassembly.html posts/hetero-queue.html

tmp:
	mkdir tmp

tmp/rust-speed.md.html: posts/rust-speed.md tmp
	$(MARKDOWN) $< > $@

tmp/hetero-queue.md.html: posts/hetero-queue.md tmp
	$(MARKDOWN) $< > $@

tmp/webassembly.md.html: posts/webassembly.md tmp
	$(MARKDOWN) $< > $@

tmp/index.md.html: index.md tmp
	$(MARKDOWN) $< > $@

tmp/rust-vs-go.md.html: posts/rust-vs-go.md tmp
	$(MARKDOWN) $< > $@

posts/rust-speed.html: tmp/rust-speed.md.html $(SHARED)
	$(TEMPLATE) "-D KATEX" "-D TITLE=Rust Speed" "-D POST=\"./tmp/rust-speed.md.html\"" post.t.html > posts/rust-speed.html

posts/hetero-queue.html: tmp/hetero-queue.md.html $(SHARED)
	$(TEMPLATE) "-D TITLE=Heterogeneous List" "-D POST=\"./tmp/hetero-queue.md.html\"" post.t.html > posts/hetero-queue.html

posts/rust-vs-go.html: tmp/rust-vs-go.md.html $(SHARED)
	$(TEMPLATE) "-D TITLE=Rust vs Go" "-D POST=\"./tmp/rust-vs-go.md.html\"" post.t.html > posts/rust-vs-go.html

posts/webassembly.html: tmp/webassembly.md.html $(SHARED)
	$(TEMPLATE) "-D TITLE=Web Assembly" "-D POST=\"./tmp/webassembly.md.html\"" post.t.html > posts/webassembly.html

index.html: tmp/index.md.html index.t.html $(SHARED)
	$(TEMPLATE) index.t.html > index.html

clean:
	rm index.html -rf tmp/ posts/*.html
