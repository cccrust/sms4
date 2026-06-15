import { Fragment } from "react";

const URL_RE = /(https?:\/\/[^\s]+)/g;

function youtubeId(url: string): string | null {
  const u = url.replace(/^https?:\/\//, "").replace(/^www\./, "");
  const m = u.match(/^youtube\.com\/watch\?v=([\w-]+)/);
  if (m) return m[1];
  const s = u.match(/^youtu\.be\/([\w-]+)/);
  if (s) return s[1];
  const e = u.match(/^youtube\.com\/embed\/([\w-]+)/);
  if (e) return e[1];
  return null;
}

function vimeoId(url: string): string | null {
  const m = url.match(/vimeo\.com\/(\d+)/);
  return m ? m[1] : null;
}

function isVideoUrl(url: string): boolean {
  return /\.(mp4|webm|mov)(\?.*)?$/i.test(url);
}

function isAudioUrl(url: string): boolean {
  return /\.(mp3|wav|ogg|aac)(\?.*)?$/i.test(url);
}

function urlDomain(url: string): string | null {
  try {
    return new URL(url).hostname.replace(/^www\./, "");
  } catch {
    return null;
  }
}

export default function PostContent({ content }: { content: string }) {
  const parts: { type: "text" | "url" | "youtube" | "vimeo" | "video" | "audio"; value: string }[] = [];
  let last = 0;
  let m: RegExpExecArray | null;
  URL_RE.lastIndex = 0;

  while ((m = URL_RE.exec(content)) !== null) {
    if (m.index > last) parts.push({ type: "text", value: content.slice(last, m.index) });
    const url = m[0];
    const yid = youtubeId(url);
    if (yid) {
      parts.push({ type: "youtube", value: yid });
    } else {
      const vid = vimeoId(url);
      if (vid) {
        parts.push({ type: "vimeo", value: vid });
      } else if (isVideoUrl(url)) {
        parts.push({ type: "video", value: url });
      } else if (isAudioUrl(url)) {
        parts.push({ type: "audio", value: url });
      } else {
        parts.push({ type: "url", value: url });
      }
    }
    last = URL_RE.lastIndex;
  }
  if (last < content.length) parts.push({ type: "text", value: content.slice(last) });

  return (
    <>
      {parts.map((p, i) => {
        if (p.type === "text") return <Fragment key={i}>{p.value}</Fragment>;
        if (p.type === "url")
          return (
            <a key={i} href={p.value} target="_blank" rel="noopener noreferrer"
              className="text-blue-400 hover:underline break-all"
              onClick={(e) => e.stopPropagation()}
            >
              {urlDomain(p.value) || p.value}
            </a>
          );
        if (p.type === "youtube")
          return (
            <div key={i} className="my-2 aspect-video" onClick={(e) => e.stopPropagation()}>
              <iframe
                className="w-full h-full rounded-xl"
                src={`https://www.youtube.com/embed/${p.value}`}
                allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
                allowFullScreen
              />
            </div>
          );
        if (p.type === "vimeo")
          return (
            <div key={i} className="my-2 aspect-video" onClick={(e) => e.stopPropagation()}>
              <iframe
                className="w-full h-full rounded-xl"
                src={`https://player.vimeo.com/video/${p.value}`}
                allow="autoplay; fullscreen; picture-in-picture"
                allowFullScreen
              />
            </div>
          );
        if (p.type === "video")
          return (
            <div key={i} className="my-2" onClick={(e) => e.stopPropagation()}>
              <video controls className="w-full rounded-xl max-h-96" preload="metadata">
                <source src={p.value} />
              </video>
            </div>
          );
        if (p.type === "audio")
          return (
            <div key={i} className="my-2" onClick={(e) => e.stopPropagation()}>
              <audio controls className="w-full" preload="metadata">
                <source src={p.value} />
              </audio>
            </div>
          );
        return null;
      })}
    </>
  );
}
