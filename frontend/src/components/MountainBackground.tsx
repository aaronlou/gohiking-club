import { useEffect, useRef } from "react";

export function MountainBackground() {
  const containerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const container = containerRef.current;
    if (!container) return;

    // Create floating particles (pollen / dust / fireflies)
    const particles: HTMLDivElement[] = [];
    const particleCount = 25;

    for (let i = 0; i < particleCount; i++) {
      const p = document.createElement("div");
      const size = Math.random() * 3 + 1;
      p.style.cssText = `
        position: absolute;
        width: ${size}px;
        height: ${size}px;
        background: rgba(255, 230, 180, ${Math.random() * 0.4 + 0.2});
        border-radius: 50%;
        left: ${Math.random() * 100}%;
        top: ${Math.random() * 60 + 20}%;
        pointer-events: none;
        animation: float-particle ${Math.random() * 8 + 6}s ease-in-out infinite;
        animation-delay: ${Math.random() * 5}s;
        filter: blur(${Math.random() > 0.7 ? 1 : 0}px);
      `;
      container.appendChild(p);
      particles.push(p);
    }

    return () => {
      particles.forEach((p) => p.remove());
    };
  }, []);

  return (
    <div ref={containerRef} className="absolute inset-0 overflow-hidden">
      {/* Sky gradient - sunrise over mountains */}
      <div
        className="absolute inset-0"
        style={{
          background:
            "linear-gradient(180deg, #0d1f15 0%, #1a3a2a 15%, #2d5a3f 30%, #4a7c59 45%, #6b9b6e 55%, #8fb574 65%, #b8c97a 72%, #d4c47a 78%, #c9a86c 85%, #b08d5e 92%, #2a1a0f 100%)",
        }}
      />

      {/* Sun glow */}
      <div
        className="absolute rounded-full"
        style={{
          width: 300,
          height: 300,
          background:
            "radial-gradient(circle, rgba(255,220,150,0.35) 0%, rgba(255,200,100,0.15) 40%, transparent 70%)",
          left: "50%",
          top: "28%",
          transform: "translateX(-50%)",
          filter: "blur(20px)",
        }}
      />

      {/* Sun disk */}
      <div
        className="absolute rounded-full"
        style={{
          width: 80,
          height: 80,
          background:
            "radial-gradient(circle, rgba(255,230,180,0.9) 0%, rgba(255,200,120,0.4) 50%, transparent 70%)",
          left: "50%",
          top: "32%",
          transform: "translateX(-50%)",
          filter: "blur(2px)",
        }}
      />

      {/* Mist layers */}
      <div
        className="absolute inset-x-0 bottom-0 h-48"
        style={{
          background:
            "linear-gradient(to top, rgba(200,210,180,0.15) 0%, transparent 100%)",
        }}
      />

      {/* Distant mountain range - silhouette */}
      <svg
        className="absolute bottom-0 left-0 w-full"
        viewBox="0 0 1440 320"
        preserveAspectRatio="none"
        style={{ height: "45%", opacity: 0.4 }}
      >
        <path
          fill="#1a3322"
          d="M0,200 L60,180 L120,195 L180,165 L240,185 L300,150 L360,170 L420,140 L480,160 L540,130 L600,155 L660,120 L720,145 L780,110 L840,135 L900,105 L960,125 L1020,95 L1080,120 L1140,90 L1200,115 L1260,85 L1320,105 L1380,80 L1440,100 L1440,320 L0,320 Z"
        />
      </svg>

      {/* Middle mountain range */}
      <svg
        className="absolute bottom-0 left-0 w-full"
        viewBox="0 0 1440 280"
        preserveAspectRatio="none"
        style={{ height: "38%", opacity: 0.6 }}
      >
        <path
          fill="#0f2818"
          d="M0,220 L80,190 L160,210 L240,175 L320,200 L400,160 L480,185 L560,145 L640,170 L720,130 L800,155 L880,120 L960,145 L1040,110 L1120,135 L1200,100 L1280,125 L1360,95 L1440,115 L1440,280 L0,280 Z"
        />
      </svg>

      {/* Near mountain range - darkest */}
      <svg
        className="absolute bottom-0 left-0 w-full"
        viewBox="0 0 1440 240"
        preserveAspectRatio="none"
        style={{ height: "32%", opacity: 0.85 }}
      >
        <path
          fill="#0d1f15"
          d="M0,240 L100,200 L200,225 L300,185 L400,210 L500,170 L600,195 L700,155 L800,180 L900,140 L1000,165 L1100,130 L1200,155 L1300,125 L1400,145 L1440,140 L1440,240 L0,240 Z"
        />
      </svg>

      {/* Foreground trees silhouette */}
      <svg
        className="absolute bottom-0 left-0 w-full"
        viewBox="0 0 1440 120"
        preserveAspectRatio="none"
        style={{ height: "15%", opacity: 0.7 }}
      >
        <path
          fill="#08150e"
          d="M0,120 L20,90 L40,100 L60,75 L80,90 L100,70 L120,85 L140,65 L160,80 L180,60 L200,75 L220,55 L240,70 L260,50 L280,65 L300,45 L320,60 L340,40 L360,55 L380,35 L400,50 L420,30 L440,45 L460,25 L480,40 L500,20 L520,35 L540,55 L560,30 L580,45 L600,25 L620,40 L640,60 L660,35 L680,50 L700,30 L720,45 L740,65 L760,40 L780,55 L800,35 L820,50 L840,70 L860,45 L880,60 L900,40 L920,55 L940,75 L960,50 L980,65 L1000,45 L1020,60 L1040,80 L1060,55 L1080,70 L1100,50 L1120,65 L1140,85 L1160,60 L1180,75 L1200,55 L1220,70 L1240,90 L1260,65 L1280,80 L1300,60 L1320,75 L1340,95 L1360,70 L1380,85 L1400,65 L1420,80 L1440,100 L1440,120 L0,120 Z"
        />
      </svg>

      {/* Subtle topo pattern overlay - very faint */}
      <div
        className="absolute inset-0 opacity-[0.03] bg-[url('/topo-pattern-flip.svg')] bg-[length:400px_400px]"
      />

      {/* Vignette overlay for depth */}
      <div
        className="absolute inset-0"
        style={{
          background:
            "radial-gradient(ellipse at center, transparent 40%, rgba(5,15,8,0.4) 100%)",
        }}
      />
    </div>
  );
}
