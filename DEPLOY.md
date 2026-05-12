# 🚀 GoHiking Club — 生产部署指南

使用 **Docker Compose + Caddy** 一键部署，Caddy 自动处理 HTTPS（Let's Encrypt），无需手动配置证书。

---

## 📋 服务器要求

- **OS**: Ubuntu 22.04+ / Debian 12+ / 任何支持 Docker 的 Linux
- **CPU**: 1 核+
- **内存**: 1GB+（推荐 2GB）
- **磁盘**: 10GB+
- **网络**: 公网 IP，域名已解析到服务器（A 记录）

---

## 🛠️ 服务器初始化

### 1. 安装 Docker & Docker Compose

```bash
curl -fsSL https://get.docker.com | sh
sudo usermod -aG docker $USER
newgrp docker
```

### 2. 验证安装

```bash
docker --version
docker compose version
```

---

## 📦 部署步骤

### 1. 克隆代码到服务器

```bash
git clone <你的仓库地址> gohiking-club
cd gohiking-club
```

### 2. 配置环境变量

```bash
cp .env.example .env
nano .env
```

**必须修改的字段：**

```env
# 你的域名（如 gohiking.example.com）
DOMAIN=gohiking.example.com

# 数据库密码（高强度随机字符串）
POSTGRES_PASSWORD=YourStrongPassword123!

# JWT 密钥（至少 32 位随机字符串）
JWT_SECRET=change-me-to-a-64-char-random-string-xxxxxxxxxxxxxxxx

# AI 评分提供商（gemini / claude / openai / ollama）
AI_PROVIDER=gemini

# Gemini API Key（如果使用 Gemini）
GEMINI_API_KEY=AIzaSyxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
```

### 3. 一键部署

```bash
./deploy.sh
```

部署脚本会自动：
- ✅ 检查环境变量
- ✅ 拉取最新代码
- ✅ 构建 Docker 镜像
- ✅ 启动所有服务
- ✅ 等待后端就绪
- ✅ 清理旧镜像

---

## 🔧 常用运维命令

### 查看日志

```bash
# 全部服务日志
docker compose -f docker-compose.prod.yml logs -f

# 只看后端
docker compose -f docker-compose.prod.yml logs -f backend

# 只看 Caddy
docker compose -f docker-compose.prod.yml logs -f caddy
```

### 重启服务

```bash
docker compose -f docker-compose.prod.yml restart backend
```

### 更新部署（拉取最新代码后）

```bash
./deploy.sh
```

### 停止所有服务

```bash
docker compose -f docker-compose.prod.yml down
```

### 停止并删除数据（⚠️ 危险）

```bash
docker compose -f docker-compose.prod.yml down -v
```

---

## 📁 数据持久化

数据通过 Docker Volume 持久化：

| Volume | 说明 |
|--------|------|
| `pgdata` | PostgreSQL 数据库 |
| `uploads` | 上传的照片文件 |
| `caddy_data` | Caddy 证书和配置 |
| `caddy_config` | Caddy 运行时配置 |

备份数据库：

```bash
docker compose -f docker-compose.prod.yml exec postgres pg_dump -U gohiking gohiking > backup_$(date +%Y%m%d).sql
```

恢复数据库：

```bash
docker compose -f docker-compose.prod.yml exec -T postgres psql -U gohiking gohiking < backup_20250101.sql
```

---

## 🌐 架构说明

```
┌─────────────────────────────────────────────────────────┐
│                      用户请求                             │
│                   https://your-domain.com                  │
└─────────────────────────────────────────────────────────┘
                           │
                    ┌──────▼──────┐
                    │   Caddy     │  ← 自动 HTTPS (Let's Encrypt)
                    │  (80/443)   │
                    └──────┬──────┘
                           │
            ┌──────────────┼──────────────┐
            │              │              │
     ┌──────▼──────┐ ┌────▼────┐ ┌──────▼──────┐
     │  /api/*     │ │/uploads*│ │     /*      │
     │  Backend    │ │ Backend │ │  Frontend   │
     │  Rust/Axum  │ │(static) │ │  Nginx/SPA  │
     │  port 3000  │ │         │ │   port 80   │
     └──────┬──────┘ └─────────┘ └─────────────┘
            │
     ┌──────▼──────┐
     │  Postgres   │
     │   port 5432 │
     └─────────────┘
```

---

## 🔒 安全建议

1. **防火墙**：只开放 80、443 端口
   ```bash
   sudo ufw allow 80/tcp
   sudo ufw allow 443/tcp
   sudo ufw enable
   ```

2. **JWT Secret**：生产环境务必使用 64 位以上随机字符串
   ```bash
   openssl rand -base64 48
   ```

3. **数据库密码**：使用高强度密码
   ```bash
   openssl rand -base64 24
   ```

4. **定期更新**：
   ```bash
   docker compose -f docker-compose.prod.yml pull
   docker compose -f docker-compose.prod.yml up -d
   ```

---

## 🆘 故障排查

### Caddy 无法获取 HTTPS 证书

- 确认域名 A 记录已指向服务器公网 IP
- 确认服务器 80/443 端口未被占用
- 查看日志：`docker compose -f docker-compose.prod.yml logs caddy`

### 后端启动失败

- 检查数据库连接：`docker compose -f docker-compose.prod.yml logs backend`
- 确认 `DATABASE_URL` 格式正确
- 确认 postgres 服务已 healthy

### 前端白屏/404

- Caddy 已配置 SPA fallback
- 确认前端构建成功：`docker compose -f docker-compose.prod.yml logs frontend`

### 图片上传失败

- 检查 `uploads` volume 权限
- 确认 `APP__STORAGE__LOCAL__PUBLIC_URL_PREFIX` 域名正确

---

## 🎯 切换存储后端（可选）

默认使用本地文件存储。如需切换到 S3/MinIO：

```env
STORAGE_BACKEND=s3
S3_ENDPOINT=https://s3.amazonaws.com
S3_REGION=us-east-1
S3_BUCKET=your-bucket
S3_PUBLIC_ENDPOINT=https://your-cdn.com
```

然后重启：
```bash
docker compose -f docker-compose.prod.yml up -d backend
```
