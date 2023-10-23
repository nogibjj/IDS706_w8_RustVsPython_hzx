install:
	cargo install mdbook
	#install node
	#curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash - &&\
	#sudo apt-get install -y nodejs
	#npm install -g @githubnext/github-copilot-cli
	
	echo 'eval "$(github-copilot-cli alias -- "$0")"' >> ~/.bashrc

format:
	@echo "Formatting all projects with cargo"
	./format.sh

lint:
	@echo "Linting all projects with cargo"
	@rustup component add clippy 2> /dev/null
	./lint.sh

test:
	@echo "Testing all projects with cargo"
	./test.sh

check-gpu-linux:
	sudo lshw -C display

linkcheck:
	mdbook test -L data-eng-rust-tutorial

run:
	cargo run

release:
	cargo build --release

all: format lint test run

python_install:
	pip install --upgrade pip &&\
		pip install -r requirements.txt

python_test:
	python -m pytest -vv --cov=main --cov=mylib test_*.py

python_format:	
	black *.py 

python_lint:
	ruff check *.py mylib/*.py

python_container-lint:
	docker run --rm -i hadolint/hadolint < Dockerfile

python_refactor: format lint

python_deploy:
	#deploy goes here
		
# generate_and_push:
# 	# Add, commit, and push the generated files to GitHub
# 	@if [ -n "$$(git status --porcelain)" ]; then \
# 		git config --local user.email "action@github.com"; \
# 		git config --local user.name "GitHub Action"; \
# 		git add .; \
# 		git commit -m "Add metric log"; \
# 		git push; \
# 	else \
# 		echo "No changes to commit. Skipping commit and push."; \
# 	fi
