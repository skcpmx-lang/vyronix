/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{ts,tsx}"],
  darkMode: "class",
  theme: {
    extend: {
      colors: {
        // VYRONIX brand palette (original, premium diagnostic identity).
        vx: {
          50: "#eef7ff",
          100: "#d9edff",
          200: "#bce0ff",
          300: "#8ecdff",
          400: "#59b0ff",
          500: "#3391ff",
          600: "#1b71f5",
          700: "#145ae1",
          800: "#1749b6",
          900: "#19408f",
          950: "#142857",
        },
        accent: {
          DEFAULT: "#22d3ee",
          soft: "#67e8f9",
        },
        surface: {
          light: "#ffffff",
          dark: "#0b1220",
        },
      },
      fontFamily: {
        sans: ["Segoe UI", "system-ui", "-apple-system", "sans-serif"],
        mono: ["Cascadia Code", "Consolas", "monospace"],
      },
      boxShadow: {
        card: "0 1px 3px rgba(0,0,0,0.08), 0 4px 16px rgba(0,0,0,0.06)",
        glow: "0 0 24px rgba(51,145,255,0.25)",
      },
      animation: {
        "fade-in": "fadeIn 0.25s ease-out",
        "slide-up": "slideUp 0.3s ease-out",
        "pulse-soft": "pulseSoft 2s ease-in-out infinite",
      },
      keyframes: {
        fadeIn: { "0%": { opacity: "0" }, "100%": { opacity: "1" } },
        slideUp: { "0%": { opacity: "0", transform: "translateY(8px)" }, "100%": { opacity: "1", transform: "translateY(0)" } },
        pulseSoft: { "0%,100%": { opacity: "1" }, "50%": { opacity: "0.6" } },
      },
    },
  },
  plugins: [],
};
