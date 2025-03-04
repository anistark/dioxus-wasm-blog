# Set shell to Bash
set shell := ["bash", "-cu"]

# Install dependencies and prepare project
install:
    rustup target add wasm32-unknown-unknown
    npm install

# Start Dioxus in development mode (ensures WASM is built)
serve:
    just clean && just build
    npx tailwindcss -i ./input.css -o ./dist/output.css --watch &
    dx serve

# Build the WASM app for production
build:
    just install
    cargo build --target wasm32-unknown-unknown --release
    dx build --release
    npx tailwindcss -i ./input.css -o ./dist/output.css --minify
    mkdir -p pkg
    cp dist/output.css pkg/

# Watch Tailwind in development
watch-tailwind:
    npx tailwindcss -i ./input.css -o ./dist/output.css --watch

# Clean up generated files
clean:
    rm -rf dist target pkg

# Deploy to GitHub Pages
deploy:
    just build
    git add .
    git commit -m "Deploy new version"
    git push origin main
    npx gh-pages -d pkg
