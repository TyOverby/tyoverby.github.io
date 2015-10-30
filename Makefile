TEMPLATE=cpp -w -undef -o -nostdinc -E -P
MARKDOWN=pandoc -f markdown -t html
SHARED=common-header.t.html common-footer.t.html style.css

all: index.html posts/rust-speed.html posts/rust-vs-go.html

posts/rust-speed.md.html: posts/rust-speed.md
	$(MARKDOWN) $< > $@

posts/rust-speed.html: posts/rust-speed.md.html $(SHARED)
	$(TEMPLATE) "-D TITLE=Rust Speed" "-D POST=\"./posts/rust-speed.md.html\"" post.t.html > posts/rust-speed.html

posts/rust-vs-go.md.html: posts/rust-vs-go.md
	$(MARKDOWN) $< > $@

posts/rust-vs-go.html: posts/rust-vs-go.md.html $(SHARED)
	$(TEMPLATE) "-D TITLE=Rust vs Go" "-D POST=\"./posts/rust-vs-go.md.html\"" post.t.html > posts/rust-vs-go.html

index.html: index.t.html $(SHARED)
	$(TEMPLATE) index.t.html > index.html

clean:
	rm index.html
	rm posts/*.md.html
	rm posts/rust-speed.html
	rm posts/rust-vs-go.html

publish:
	rm posts/*.md.html
