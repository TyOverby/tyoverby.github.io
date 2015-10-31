TEMPLATE=cpp -w -undef -o -nostdinc -E -P
MARKDOWN=pandoc -f markdown -t html --katex
SHARED=common-header.t.html common-footer.t.html style.css katex.min.js katex.min.css

all: index.html posts/rust-speed.html posts/rust-vs-go.html

tmp/rust-speed.md.html: posts/rust-speed.md
	$(MARKDOWN) $< > $@

posts/rust-speed.html: tmp/rust-speed.md.html $(SHARED)
	$(TEMPLATE) "-D TITLE=Rust Speed" "-D POST=\"./tmp/rust-speed.md.html\"" post.t.html > posts/rust-speed.html

tmp/rust-vs-go.md.html: posts/rust-vs-go.md
	$(MARKDOWN) $< > $@

posts/rust-vs-go.html: tmp/rust-vs-go.md.html $(SHARED)
	$(TEMPLATE) "-D TITLE=Rust vs Go" "-D POST=\"./tmp/rust-vs-go.md.html\"" post.t.html > posts/rust-vs-go.html

index.html: index.t.html $(SHARED)
	$(TEMPLATE) index.t.html > index.html

clean:
	rm index.html
	rm tmp/*
	rm posts/rust-speed.html
	rm posts/rust-vs-go.html
