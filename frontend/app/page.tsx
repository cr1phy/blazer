export default function Home() {
  return (
    <svg width="100" height="100" viewBox="0 0 64 64" xmlns="http://www.w3.org/2000/svg">
      <defs>
        <linearGradient id="gradient" x1="0%" y1="0%" x2="100%" y2="100%">
          <stop offset="0%" style={{stopColor: "#1A237E", stopOpacity: 1}} />
          <stop offset="100%" style={{stopColor: "#00ACC1", stopOpacity: 1}} />
        </linearGradient>
      </defs>
      <polygon fill="url(#gradient)" points="24,4 8,36 28,36 20,60 44,28 24,28"/>
    </svg>

  );
}
