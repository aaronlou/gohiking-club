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
      canvas.height = 280;
    };
    resize();
    window.addEventListener("resize", resize);

    const draw = () => {
      const w = canvas.width;
      const h = canvas.height;
      t += 0.003;

      ctx.clearRect(0, 0, w, h);

      // Draw subtle mountain silhouettes
      ctx.fillStyle = "rgba(255,255,255,0.06)";
      for (let i = 0; i < 3; i++) {
        ctx.beginPath();
        ctx.moveTo(0, h);
        const offset = i * 120 + t * 20;
        for (let x = 0; x <= w; x += 4) {
          const y =
            h -
            40 -
            Math.sin((x + offset) * 0.003 + i) * 30 -
            Math.sin((x + offset) * 0.007) * 20 -
            i * 25;
          ctx.lineTo(x, y);
        }
        ctx.lineTo(w, h);
        ctx.closePath();
        ctx.fill();
      }

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
        style={{ height: 280 }}
      />
      <div className="relative z-10">
        <h1 className="font-display text-4xl sm:text-5xl font-bold text-cream-50 leading-tight">
          记录每一次
          <br />
          <span className="text-earth-400">徒步之旅</span>
        </h1>
        <p className="mt-4 max-w-lg text-sm sm:text-base text-cream-200/80 leading-relaxed">
          上传你的户外照片，AI 自动评分，与徒步爱好者分享精彩瞬间，发现下一个目的地。
        </p>
      </div>
    </div>
  );
}
