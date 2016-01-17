TEMPLATE=cpp -w -undef -nostdinc -E -P
MARKDOWN=pandoc -f markdown -t html --katex
SHARED=common-header.t.html common-footer.t.html css/style.css js/katex.min.js css/katex.min.css

all: index.html posts/rust-speed.html posts/rust-vs-go.html

tmp:
	mkdir tmp

tmp/rust-speed.md.html: posts/rust-speed.md tmp
	$(MARKDOWN) $< > $@

tmp/index.md.html: index.md tmp
	$(MARKDOWN) $< > $@

posts/rust-speed.html: tmp/rust-speed.md.html $(SHARED)
	$(TEMPLATE) "-D TITLE=Rust Speed" "-D POST=\"./tmp/rust-speed.md.html\"" post.t.html > posts/rust-speed.html

tmp/rust-vs-go.md.html: posts/rust-vs-go.md tmp
	$(MARKDOWN) $< > $@

posts/rust-vs-go.html: tmp/rust-vs-go.md.html $(SHARED)
	$(TEMPLATE) "-D TITLE=Rust vs Go" "-D POST=\"./tmp/rust-vs-go.md.html\"" post.t.html > posts/rust-vs-go.html

index.html: tmp/index.md.html index.t.html $(SHARED)
	$(TEMPLATE) index.t.html > index.html

clean:
	rm index.html -rf tmp/ posts/*.html
