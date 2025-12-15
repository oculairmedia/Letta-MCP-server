{ inputs, ... }: {
  imports = [
    inputs.rust-flake.flakeModules.default
    inputs.rust-flake.flakeModules.nixpkgs
  ];
  perSystem =
    { config
    , self'
    , pkgs
    , lib
    , ...
    }:
    let
      # Test script that runs local server tests
      testLocalServer = pkgs.writeShellScript "test-local-server" ''
        set -euo pipefail

        # We're in the build directory, which has the source
        echo "🚀 Starting local Letta server for integration tests..."

        # Check if docker is available
        if ! command -v docker &> /dev/null; then
          echo "⚠️  Docker not available in build environment, skipping integration tests"
          echo "   Run 'nix run .#test-local' to run tests with docker"
          exit 0
        fi

        # Start docker compose
        ${pkgs.docker-compose}/bin/docker-compose up -d

        # Cleanup function
        cleanup() {
          echo "🛑 Stopping local Letta server..."
          ${pkgs.docker-compose}/bin/docker-compose down || true
        }
        trap cleanup EXIT

        # Wait for server
        echo "⏳ Waiting for server to be ready..."
        max_attempts=30
        attempt=0

        while ! ${pkgs.curl}/bin/curl -s http://localhost:8283/v1/health >/dev/null 2>&1; do
          attempt=$((attempt + 1))
          if [ $attempt -ge $max_attempts ]; then
            echo "❌ Server failed to start after $max_attempts attempts"
            exit 1
          fi
          echo "  Attempt $attempt/$max_attempts..."
          sleep 2
        done

        echo "✅ Server is ready!"

        # Run integration tests
        echo "🧪 Running integration tests..."
        cargo test --test '*' -- --nocapture
      '';
    in
    {
      rust-project.crates."letta".crane.args = {
        # Enable checks
        doCheck = true;

        buildInputs = lib.optionals pkgs.stdenv.isDarwin (
          with pkgs.darwin.apple_sdk.frameworks; [
            IOKit
          ]
        );

        # Add docker-compose to native build inputs for the check phase
        nativeBuildInputs = [ pkgs.docker-compose pkgs.docker ];

        # Custom check phase that runs unit tests only
        # Integration tests require Docker which isn't available in sandbox
        checkPhase = ''
          runHook preCheck

          echo "🧪 Running unit tests..."
          cargo test --lib --bins
          echo "📚 Running doc tests..."
          cargo test --doc

          echo "✅ Unit tests passed!"
          echo ""
          echo "ℹ️  Integration tests require Docker and must be run separately"
          echo "   To run integration tests locally:"
          echo "   nix run .#test-local"

          runHook postCheck
        '';
      };

      packages = {
        default = self'.packages.letta;

        # Package for running tests with local server
        letta-with-tests = self'.packages.letta.overrideAttrs (oldAttrs: {
          checkPhase = ''
            runHook preCheck

            # Run unit tests
            echo "🧪 Running unit tests..."
            cargo test --lib --bins
            cargo test --doc

            # Run integration tests if requested
            if [ "''${LETTA_RUN_INTEGRATION_TESTS:-0}" = "1" ]; then
              ${testLocalServer}
            else
              echo "ℹ️  Skipping integration tests (set LETTA_RUN_INTEGRATION_TESTS=1 to enable)"
            fi

            runHook postCheck
          '';
          doCheck = true;
        });
      };

      # Apps for running different test suites
      apps = {
        test-local = {
          type = "app";
          program = toString (pkgs.writeShellScript "test-local-app" ''
            cd ${self'.packages.letta.src}
            ${testLocalServer}
          '');
        };

        test-cloud = {
          type = "app";
          program = toString (pkgs.writeShellScript "test-cloud-app" ''
            cd ${self'.packages.letta.src}

            if [ -z "''${LETTA_API_KEY:-}" ]; then
              echo "❌ LETTA_API_KEY environment variable is required"
              exit 1
            fi

            echo "🌩️  Running cloud API tests..."
            cargo test --test '*cloud*' -- --ignored --nocapture
          '');
        };

        test-all = {
          type = "app";
          program = toString (pkgs.writeShellScript "test-all-app" ''
            cd ${self'.packages.letta.src}

            echo "🧪 Running all tests..."

            # Unit tests
            echo "📦 Unit tests..."
            cargo test --lib --bins
            cargo test --doc

            # Integration tests with local server
            echo ""
            echo "🏠 Local server integration tests..."
            ${testLocalServer}

            # Cloud tests if API key is available
            if [ -n "''${LETTA_API_KEY:-}" ]; then
              echo ""
              echo "☁️  Cloud API tests..."
              cargo test --test '*cloud*' -- --ignored --nocapture
            else
              echo ""
              echo "⚠️  Skipping cloud tests (LETTA_API_KEY not set)"
            fi

            echo ""
            echo "✅ All tests completed!"
          '');
        };
      };
    };
}
