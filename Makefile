.PHONY: all build run test clean install

all: build

build:
	@echo "🚀 Building zenvpush..."
	@cargo build --release
	@echo "✨ Build complete!"
	@$(call progress_bar)

run:
	@echo "😎 Running zenvpush..."
	@cargo run -- --secrets-file secrets.txt
	@echo "🎉 Run complete!"
	@$(call progress_bar)

test:
	@echo "🧪 Running tests..."
	@cargo test
	@echo "🍻 Tests complete!"
	@$(call progress_bar)

clean:
	@echo "🧹 Cleaning project..."
	@cargo clean
	@echo "✨ Clean complete!"
	@$(call progress_bar)

install: build
	@echo "📦 Installing zenvpush to $(HOME)/.local/bin..."
	@mkdir -p $(HOME)/.local/bin
	@cp target/release/zenvpush $(HOME)/.local/bin/zenvpush
	@chmod +x $(HOME)/.local/bin/zenvpush
	@echo "💾 Installation complete!"
	@$(call progress_bar)

define progress_bar
	@echo -n "Progress: ["
	@for i in `seq 1 20`; do \
		sleep 0.05; \
		printf "#"; \
	done; \
	echo "] 100% 😜"
endef
