#!/bin/bash
set -e

# ═══════════════════════════════════════════════════════════════
# GoHiking Club — Production Deploy Script
# ═══════════════════════════════════════════════════════════════
#
# Usage:
#   ./deploy.sh           # 本地构建镜像并部署（默认）
#   ./deploy.sh --remote  # 从镜像仓库拉取预构建镜像并部署
#   ./deploy.sh --no-pull # 不执行 git pull，直接部署当前代码
#
# ═══════════════════════════════════════════════════════════════

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

info()  { echo -e "${GREEN}[INFO]${NC}  $1"; }
warn()  { echo -e "${YELLOW}[WARN]${NC}  $1"; }
error() { echo -e "${RED}[ERROR]${NC} $1"; }
step()  { echo -e "${BLUE}[STEP]${NC}  $1"; }

# ── Parse args ──
REMOTE_MODE=false
NO_PULL=false

for arg in "$@"; do
    case "$arg" in
        --remote) REMOTE_MODE=true ;;
        --no-pull) NO_PULL=true ;;
    esac
done

# ── Check prerequisites ──
step "Checking prerequisites..."

command -v docker >/dev/null 2>&1 || { error "Docker is not installed"; exit 1; }
command -v docker-compose >/dev/null 2>&1 || docker compose version >/dev/null 2>&1 || { error "Docker Compose is not installed"; exit 1; }

if [ ! -f ".env" ]; then
    if [ -f ".env.example" ]; then
        warn ".env not found, copying from .env.example"
        cp .env.example .env
        error "Please edit .env with your actual configuration before deploying"
        exit 1
    else
        error ".env file not found"
        exit 1
    fi
fi

# ── Load and validate env ──
export $(grep -v '^#' .env | xargs)

if [ -z "$DOMAIN" ]; then
    error "DOMAIN is not set in .env"
    exit 1
fi

if [ -z "$JWT_SECRET" ] || [ "${#JWT_SECRET}" -lt 32 ]; then
    error "JWT_SECRET must be at least 32 characters"
    exit 1
fi

if [ -z "$POSTGRES_PASSWORD" ]; then
    error "POSTGRES_PASSWORD is not set"
    exit 1
fi

info "Deploying to domain: $DOMAIN"

# ── Pull latest code (optional) ──
if [ "$NO_PULL" = false ] && [ -d ".git" ]; then
    step "Pulling latest code..."
    git pull origin main || true
fi

# ── Deploy ──
if [ "$REMOTE_MODE" = true ]; then
    step "Deploying with pre-built images from registry..."

    if [ -z "${BACKEND_IMAGE:-}" ] || [ -z "${FRONTEND_IMAGE:-}" ]; then
        error "REMOTE_MODE requires BACKEND_IMAGE and FRONTEND_IMAGE to be set in .env"
        error "Example:"
        error "  BACKEND_IMAGE=ghcr.io/yourname/gohiking-backend:latest"
        error "  FRONTEND_IMAGE=ghcr.io/yourname/gohiking-frontend:latest"
        exit 1
    fi

    docker compose -f docker-compose.prod.yml pull backend frontend
    docker compose -f docker-compose.prod.yml up -d
else
    step "Building images locally..."
    docker compose -f docker-compose.prod.yml build
    docker compose -f docker-compose.prod.yml up -d
fi

# ── Wait for backend health ──
step "Waiting for backend to be ready..."
for i in {1..30}; do
    if curl -sf http://localhost:3000/api/events >/dev/null 2>&1; then
        info "Backend is ready"
        break
    fi
    sleep 2
    if [ "$i" -eq 30 ]; then
        warn "Backend health check timed out, but continuing..."
    fi
done

# ── Cleanup ──
step "Cleaning up old images..."
docker system prune -f --volumes=false

info "═══════════════════════════════════════════════════"
info "Deployment complete!"
info ""
info "Your app should be available at:"
info "  https://$DOMAIN"
info ""
info "View logs:"
info "  docker compose -f docker-compose.prod.yml logs -f"
info "═══════════════════════════════════════════════════"
