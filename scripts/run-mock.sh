#!/bin/bash
set -euo pipefail

echo "🚀 Starting Mock Mode (no API keys needed)"
MOCK_WEATHER=1 MOCK_S3=1 cargo run --features mock

