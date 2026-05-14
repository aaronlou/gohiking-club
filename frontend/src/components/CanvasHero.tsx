import { useEffect, useRef } from "react";

export function CanvasHero() {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    let animationId: number;
    let t = 0;

    const resize = () => {
      const parent = canvas.parentElement;
      if (!parent) return;
      canvas.width = parent.clientWidth;
      canvas.height = 320;
    };
    resize();
    window.addEventListener("resize", resize);

    // Particle system for "fireflies" or "pollen"
    const particles: {
      x: number;
      y: number;
      size: number;
      speedX: number;
      speedY: number;
      opacity: number;
      phase: number;
    }[] = [];

    for (let i = 0; i < 40; i++) {
      particles.push({
        x: Math.random() * canvas.width,
        y: Math.random() * canvas.height * 0.7,
        size: Math.random() * 2 + 0.5,
        speedX: (Math.random() - 0.5) * 0.3,
        speedY: (Math.random() - 0.5) * 0.15,
        opacity: Math.random() * 0.5 + 0.2,
        phase: Math.random() * Math.PI * 2,
      });
    }

    const draw = () => {
      const w = canvas.width;
      const h = canvas.height;
      t += 0.004;

      ctx.clearRect(0, 0, w, h);

      // Draw layered mountain silhouettes with more detail
      const drawMountainLayer = (
        color: string,
        baseHeight: number,
        amplitude: number,
        frequency: number,
        offset: number,
        yShift: number
      ) => {
        ctx.fillStyle = color;
        ctx.beginPath();
        ctx.moveTo(0, h);

        for (let x = 0; x <= w; x += 2) {
          const y =
            h -
            baseHeight -
            yShift -
            Math.sin((x + offset) * frequency + t * 0.3) * amplitude -
            Math.sin((x + offset) * frequency * 2.3) * (amplitude * 0.4) -
            Math.sin((x + offset) * frequency * 0.7 + t * 0.2) * (amplitude * 0.6);
          ctx.lineTo(x, y);
        }

        ctx.lineTo(w, h);
        ctx.closePath();
        ctx.fill();
      };

      // Back layer - faintest
      drawMountainLayer("rgba(255,255,255,0.04)", 60, 25, 0.003, 0, 40);
      // Middle layer
      drawMountainLayer("rgba(255,255,255,0.06)", 45, 20, 0.004, 80, 20);
      // Front layer - most visible
      drawMountainLayer("rgba(255,255,255,0.08)", 30, 15, 0.005, 160, 0);

      // Draw floating particles (fireflies / golden pollen)
      particles.forEach((p) => {
        p.x += p.speedX + Math.sin(t + p.phase) * 0.2;
        p.y += p.speedY + Math.cos(t * 0.7 + p.phase) * 0.1;

        // Wrap around
        if (p.x < -10) p.x = w + 10;
        if (p.x > w + 10) p.x = -10;
        if (p.y < -10) p.y = h * 0.7 + 10;
        if (p.y > h * 0.7 + 10) p.y = -10;

        const pulse = Math.sin(t * 2 + p.phase) * 0.3 + 0.7;
        ctx.beginPath();
        ctx.arc(p.x, p.y, p.size, 0, Math.PI * 2);
        ctx.fillStyle = `rgba(255, 220, 150, ${p.opacity * pulse})`;
        ctx.fill();

        // Glow for larger particles
        if (p.size > 1.5) {
          ctx.beginPath();
          ctx.arc(p.x, p.y, p.size * 3, 0, Math.PI * 2);
          ctx.fillStyle = `rgba(255, 200, 100, ${p.opacity * pulse * 0.15})`;
          ctx.fill();
        }
      });

      animationId = requestAnimationFrame(draw);
    };

    draw();

    return () => {
      window.removeEventListener("resize", resize);
      cancelAnimationFrame(animationId);
    };
  }, []);

  return (
    <div className="relative">
      <canvas
        ref={canvasRef}
        className="pointer-events-none absolute inset-0"
        style={{ height: 320 }}
      />
      <div className="relative z-10">
        <h1 className="font-display text-4xl sm:text-5xl lg:text-6xl font-bold text-cream-50 leading-tight drop-shadow-lg">
          记录每一次
          <br />
          <span className="text-amber-300">徒步之旅</span>
        </h1>
        <p className="mt-5 max-w-lg text-sm sm:text-base text-cream-200/90 leading-relaxed drop-shadow-md">
          上传你的户外照片，AI 自动评分，与徒步爱好者分享精彩瞬间，发现下一个目的地。
        </p>
      </div>
    </div>
  );
}
