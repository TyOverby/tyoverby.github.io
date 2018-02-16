MARKDOWN=pandoc -s --template=template.html -f markdown -t html --katex --toc
SHARED=template.html css/style.css

all: posts/bincode_release.html \
	 posts/rust-vs-go.html \
	 posts/rust-speed.html \
	 posts/implicit.html \
	 index.html

posts:
	mkdir posts

posts/bincode_release.html: posts_md/bincode_release.md $(SHARED)
	$(MARKDOWN) $< -o $@

posts/rust-speed.html: posts_md/rust-speed.md $(SHARED)
	$(MARKDOWN) $< -o $@

posts/rust-vs-go.html: posts_md/rust-vs-go.md $(SHARED)
	$(MARKDOWN) $< -o $@

posts/implicit.html: posts_md/implicit.md $(SHARED)
	$(MARKDOWN) $< -o $@

index.html: index.md $(SHARED)
	$(MARKDOWN) $< -o $@
